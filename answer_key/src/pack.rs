//! Ticket 5: the monthly workbook.

use crate::branch::{BranchInvoice, Reject};
use crate::money::Cents;
use crate::reconcile::{ALL_KINDS, Discrepancy};
use rust_xlsxwriter::{Format, FormatBorder, Workbook, Worksheet, XlsxError};
use std::collections::HashMap;

pub struct SpendRow {
    pub supplier: String,
    pub invoices: u32,
    pub total: Cents,
}

/// One row of a sheet. Each report row type says how it lays itself out.
pub trait SheetRow {
    fn headers() -> &'static [&'static str];
    fn write(&self, ws: &mut Worksheet, row: u32, money: &Format) -> Result<(), XlsxError>;
}

impl SheetRow for SpendRow {
    fn headers() -> &'static [&'static str] {
        &["Supplier", "Invoices", "Total"]
    }
    fn write(&self, ws: &mut Worksheet, row: u32, money: &Format) -> Result<(), XlsxError> {
        ws.write_string(row, 0, &self.supplier)?;
        ws.write_number(row, 1, self.invoices)?;
        ws.write_number_with_format(row, 2, self.total.as_dollars(), money)?;
        Ok(())
    }
}

pub struct DiscrepancyRow<'a> {
    pub d: &'a Discrepancy,
    pub supplier: &'a str,
}

impl SheetRow for DiscrepancyRow<'_> {
    fn headers() -> &'static [&'static str] {
        &[
            "Kind",
            "PO",
            "Invoice number",
            "Supplier",
            "Expected",
            "Billed",
            "Difference",
        ]
    }
    fn write(&self, ws: &mut Worksheet, row: u32, money: &Format) -> Result<(), XlsxError> {
        ws.write_string(row, 0, self.d.kind.to_string())?;
        ws.write_number(row, 1, self.d.po_id as f64)?;
        if let Some(n) = &self.d.invoice_number {
            ws.write_string(row, 2, n)?;
        }
        ws.write_string(row, 3, self.supplier)?;
        if let Some(e) = self.d.expected {
            ws.write_number_with_format(row, 4, e.as_dollars(), money)?;
        }
        ws.write_number_with_format(row, 5, self.d.billed.as_dollars(), money)?;
        if let Some(diff) = self.d.difference() {
            ws.write_number_with_format(row, 6, diff.as_dollars(), money)?;
        }
        Ok(())
    }
}

impl SheetRow for Reject {
    fn headers() -> &'static [&'static str] {
        &["Ref", "Field", "Value", "Reason"]
    }
    fn write(&self, ws: &mut Worksheet, row: u32, _money: &Format) -> Result<(), XlsxError> {
        ws.write_string(row, 0, &self.reference)?;
        ws.write_string(row, 1, self.field.to_string())?;
        ws.write_string(row, 2, &self.value)?;
        ws.write_string(row, 3, &self.reason)?;
        Ok(())
    }
}

fn table<T: SheetRow>(
    wb: &mut Workbook,
    name: &str,
    rows: &[T],
    widths: &[f64],
    header: &Format,
    money: &Format,
) -> Result<(), XlsxError> {
    let ws = wb.add_worksheet();
    ws.set_name(name)?;
    for (col, h) in T::headers().iter().enumerate() {
        ws.write_string_with_format(0, col as u16, *h, header)?;
    }
    for (i, r) in rows.iter().enumerate() {
        r.write(ws, i as u32 + 1, money)?;
    }
    for (col, w) in widths.iter().enumerate() {
        ws.set_column_width(col as u16, *w)?;
    }
    ws.set_freeze_panes(1, 0)?;
    let last_col = T::headers().len() as u16 - 1;
    ws.autofilter(0, 0, rows.len() as u32, last_col)?;
    Ok(())
}

pub struct Pack<'a> {
    pub month: &'a str,
    pub spend: Vec<SpendRow>,
    pub month_invoices: u32,
    pub discrepancies: Vec<DiscrepancyRow<'a>>,
    pub rejects: &'a [Reject],
}

pub fn spend_rows(
    month: &str,
    main: &[crate::model::Invoice],
    branch: &[BranchInvoice],
    names: &HashMap<i64, &str>,
) -> Vec<SpendRow> {
    let mut by: HashMap<i64, (u32, Cents)> = HashMap::new();
    for inv in main.iter().filter(|i| i.invoice_date.starts_with(month)) {
        let e = by.entry(inv.supplier_id).or_default();
        e.0 += 1;
        e.1 += inv.amount;
    }
    for inv in branch.iter().filter(|i| i.date.month_key() == month) {
        let e = by.entry(inv.supplier_id).or_default();
        e.0 += 1;
        e.1 += inv.amount;
    }
    let mut rows: Vec<SpendRow> = by
        .into_iter()
        .map(|(id, (n, total))| SpendRow {
            supplier: names[&id].to_string(),
            invoices: n,
            total,
        })
        .collect();
    rows.sort_by(|a, b| {
        b.total
            .cmp(&a.total)
            .then_with(|| a.supplier.cmp(&b.supplier))
    });
    rows
}

pub fn write(pack: &Pack, path: &str) -> Result<(), XlsxError> {
    let mut wb = Workbook::new();
    let header = Format::new()
        .set_bold()
        .set_border_bottom(FormatBorder::Thin);
    let money = Format::new().set_num_format("$#,##0.00;[Red]($#,##0.00)");
    let label = Format::new().set_bold();

    let ws = wb.add_worksheet();
    ws.set_name("Summary")?;
    ws.set_column_width(0, 34)?;
    ws.set_column_width(1, 16)?;
    let total: Cents = pack.spend.iter().map(|r| r.total).sum();
    let mut row = 0;
    ws.write_string_with_format(row, 0, "Kettle Creek Supply monthly pack", &label)?;
    row += 1;
    ws.write_string(row, 0, "Month")?;
    ws.write_string(row, 1, pack.month)?;
    row += 2;
    ws.write_string(row, 0, "Invoices dated this month")?;
    ws.write_number(row, 1, pack.month_invoices)?;
    row += 1;
    ws.write_string(row, 0, "Spend this month")?;
    ws.write_number_with_format(row, 1, total.as_dollars(), &money)?;
    row += 2;
    ws.write_string_with_format(row, 0, "Open discrepancies", &label)?;
    for kind in ALL_KINDS {
        row += 1;
        let n = pack
            .discrepancies
            .iter()
            .filter(|d| d.d.kind == kind)
            .count();
        ws.write_string(row, 0, kind.to_string())?;
        ws.write_number(row, 1, n as f64)?;
    }
    row += 2;
    ws.write_string(row, 0, "Millbrook rows rejected")?;
    ws.write_number(row, 1, pack.rejects.len() as f64)?;

    table(
        &mut wb,
        "Spend",
        &pack.spend,
        &[34.0, 10.0, 16.0],
        &header,
        &money,
    )?;
    table(
        &mut wb,
        "Discrepancies",
        &pack.discrepancies,
        &[20.0, 9.0, 16.0, 30.0, 14.0, 14.0, 14.0],
        &header,
        &money,
    )?;
    table(
        &mut wb,
        "Rejects",
        pack.rejects,
        &[14.0, 10.0, 34.0, 30.0],
        &header,
        &money,
    )?;
    wb.save(path)
}
