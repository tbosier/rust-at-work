//! The only module that talks to SQLite. Every query is a plain SELECT of
//! columns: filtering, joining and adding up happen in Rust.

use crate::model::{Branch, BranchRow, Invoice, PoLine, PurchaseOrder, Status, Supplier};
use crate::money::Cents;
use rusqlite::{Connection, Result};

pub fn open(path: &str) -> Result<Connection> {
    Connection::open_with_flags(path, rusqlite::OpenFlags::SQLITE_OPEN_READ_ONLY)
}

pub fn suppliers(db: &Connection) -> Result<Vec<Supplier>> {
    let mut stmt = db.prepare("SELECT supplier_id, name FROM suppliers")?;
    let rows = stmt.query_map([], |r| {
        Ok(Supplier {
            supplier_id: r.get(0)?,
            name: r.get(1)?,
        })
    })?;
    rows.collect()
}

/// An unknown status is a schema change we haven't handled: fail loudly.
fn status(text: &str, col: usize) -> Result<Status> {
    match text {
        "open" => Ok(Status::Open),
        "received" => Ok(Status::Received),
        "cancelled" => Ok(Status::Cancelled),
        other => Err(rusqlite::Error::FromSqlConversionFailure(
            col,
            rusqlite::types::Type::Text,
            format!("unknown PO status {other:?}").into(),
        )),
    }
}

pub fn purchase_orders(db: &Connection) -> Result<Vec<PurchaseOrder>> {
    let mut stmt = db.prepare(
        "SELECT po_id, supplier_id, branch, buyer, order_date, received_date, status \
         FROM purchase_orders",
    )?;
    let rows = stmt.query_map([], |r| {
        let branch: String = r.get(2)?;
        let st: String = r.get(6)?;
        Ok(PurchaseOrder {
            po_id: r.get(0)?,
            supplier_id: r.get(1)?,
            branch: if branch == "MILLBROOK" {
                Branch::Millbrook
            } else {
                Branch::Main
            },
            buyer: r.get(3)?,
            order_date: r.get(4)?,
            received_date: r.get(5)?,
            status: status(&st, 6)?,
        })
    })?;
    rows.collect()
}

pub fn po_lines(db: &Connection) -> Result<Vec<PoLine>> {
    let mut stmt = db.prepare("SELECT po_id, quantity, unit_price FROM po_lines")?;
    let rows = stmt.query_map([], |r| {
        Ok(PoLine {
            po_id: r.get(0)?,
            quantity: r.get(1)?,
            unit_price: Cents::from_dollars(r.get(2)?),
        })
    })?;
    rows.collect()
}

pub fn invoices(db: &Connection) -> Result<Vec<Invoice>> {
    let mut stmt = db.prepare(
        "SELECT invoice_id, invoice_number, supplier_id, po_id, invoice_date, amount FROM invoices",
    )?;
    let rows = stmt.query_map([], |r| {
        Ok(Invoice {
            invoice_id: r.get(0)?,
            invoice_number: r.get(1)?,
            supplier_id: r.get(2)?,
            po_id: r.get(3)?,
            invoice_date: r.get(4)?,
            amount: Cents::from_dollars(r.get(5)?),
        })
    })?;
    rows.collect()
}

/// Raw f64 amounts, for the ticket 2 float demonstration.
pub fn invoice_amounts_f64(db: &Connection) -> Result<Vec<(i64, String, f64)>> {
    let mut stmt = db.prepare("SELECT supplier_id, invoice_date, amount FROM invoices")?;
    let rows = stmt.query_map([], |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?)))?;
    rows.collect()
}

pub fn branch_rows(db: &Connection) -> Result<Vec<BranchRow>> {
    // rowid is the order the rows were keyed. SQL promises no order without
    // ORDER BY, so sort here: "duplicate" means "a later row with the same ref".
    let mut stmt =
        db.prepare("SELECT rowid, ref, vendor, po_ref, inv_date, amount FROM branch_invoices")?;
    let rows = stmt.query_map([], |r| {
        Ok((
            r.get::<_, i64>(0)?,
            BranchRow {
                reference: r.get(1)?,
                vendor: r.get(2)?,
                po_ref: r.get(3)?,
                inv_date: r.get(4)?,
                amount: r.get(5)?,
            },
        ))
    })?;
    let mut rows: Vec<(i64, BranchRow)> = rows.collect::<Result<_>>()?;
    rows.sort_by_key(|(rowid, _)| *rowid);
    Ok(rows.into_iter().map(|(_, row)| row).collect())
}
