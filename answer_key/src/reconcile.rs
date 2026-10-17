//! Compare what each PO said we'd pay with what suppliers billed.

use crate::branch::BranchInvoice;
use crate::model::{Branch, Invoice, PoLine, PurchaseOrder, Status};
use crate::money::Cents;
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::fmt;

/// Declaration order is report order.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Kind {
    OrphanInvoice,
    DuplicateInvoice,
    BilledCancelled,
    BilledOpen,
    Overbilled,
    Underbilled,
    NotInvoiced,
}

pub const ALL_KINDS: [Kind; 7] = [
    Kind::OrphanInvoice,
    Kind::DuplicateInvoice,
    Kind::BilledCancelled,
    Kind::BilledOpen,
    Kind::Overbilled,
    Kind::Underbilled,
    Kind::NotInvoiced,
];

impl fmt::Display for Kind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            Kind::OrphanInvoice => "ORPHAN_INVOICE",
            Kind::DuplicateInvoice => "DUPLICATE_INVOICE",
            Kind::BilledCancelled => "BILLED_CANCELLED",
            Kind::BilledOpen => "BILLED_OPEN",
            Kind::Overbilled => "OVERBILLED",
            Kind::Underbilled => "UNDERBILLED",
            Kind::NotInvoiced => "NOT_INVOICED",
        };
        f.write_str(s)
    }
}

#[derive(Debug, Clone)]
pub struct Discrepancy {
    pub kind: Kind,
    pub po_id: i64,
    pub invoice_number: Option<String>,
    pub supplier_id: i64,
    pub expected: Option<Cents>,
    pub billed: Cents,
}

impl Discrepancy {
    pub fn difference(&self) -> Option<Cents> {
        self.expected.map(|e| self.billed - e)
    }
}

pub struct Inputs<'a> {
    pub pos: &'a [PurchaseOrder],
    pub lines: &'a [PoLine],
    pub invoices: &'a [Invoice],
    pub branch: &'a [BranchInvoice],
    pub include_millbrook: bool,
    /// Received on or before this ISO date and still unbilled: NOT_INVOICED.
    pub billing_cutoff: &'a str,
}

pub fn reconcile(input: &Inputs) -> Vec<Discrepancy> {
    let mut out = Vec::new();
    let in_scope = |po: &PurchaseOrder| input.include_millbrook || po.branch == Branch::Main;

    let mut expected: HashMap<i64, Cents> = HashMap::new();
    for line in input.lines {
        *expected.entry(line.po_id).or_default() += Cents(line.unit_price.0 * line.quantity);
    }
    let pos: HashMap<i64, &PurchaseOrder> = input.pos.iter().map(|p| (p.po_id, p)).collect();

    // Main-branch invoices: the first copy of each (po, number) counts; later copies are duplicates.
    let mut sorted: Vec<&Invoice> = input.invoices.iter().collect();
    sorted.sort_by_key(|i| i.invoice_id);
    let mut seen: HashSet<(i64, &str)> = HashSet::new();
    let mut billed: HashMap<i64, Cents> = HashMap::new();
    for inv in sorted {
        let Some(po) = pos.get(&inv.po_id) else {
            out.push(Discrepancy {
                kind: Kind::OrphanInvoice,
                po_id: inv.po_id,
                invoice_number: Some(inv.invoice_number.clone()),
                supplier_id: inv.supplier_id,
                expected: None,
                billed: inv.amount,
            });
            continue;
        };
        if !in_scope(po) {
            continue;
        }
        if !seen.insert((inv.po_id, &inv.invoice_number)) {
            out.push(Discrepancy {
                kind: Kind::DuplicateInvoice,
                po_id: inv.po_id,
                invoice_number: Some(inv.invoice_number.clone()),
                supplier_id: inv.supplier_id,
                expected: None,
                billed: inv.amount,
            });
            continue;
        }
        *billed.entry(inv.po_id).or_default() += inv.amount;
    }
    if input.include_millbrook {
        for inv in input.branch {
            *billed.entry(inv.po_id).or_default() += inv.amount;
        }
    }
    let invoiced: HashSet<i64> = input
        .invoices
        .iter()
        .map(|i| i.po_id)
        .chain(
            input
                .branch
                .iter()
                .filter(|_| input.include_millbrook)
                .map(|i| i.po_id),
        )
        .collect();

    for po in input.pos.iter().filter(|p| in_scope(p)) {
        let want = expected.get(&po.po_id).copied().unwrap_or_default();
        let got = billed.get(&po.po_id).copied().unwrap_or_default();
        let has_bills = invoiced.contains(&po.po_id);
        let kind = match (po.status, has_bills) {
            (Status::Cancelled, true) => Some(Kind::BilledCancelled),
            (Status::Open, true) => Some(Kind::BilledOpen),
            (Status::Cancelled | Status::Open, false) => None,
            (Status::Received, false) => {
                let due = po
                    .received_date
                    .as_deref()
                    .is_some_and(|d| d <= input.billing_cutoff);
                due.then_some(Kind::NotInvoiced)
            }
            (Status::Received, true) => match got.cmp(&want) {
                Ordering::Greater => Some(Kind::Overbilled),
                Ordering::Less => Some(Kind::Underbilled),
                Ordering::Equal => None,
            },
        };
        if let Some(kind) = kind {
            let expected = if kind == Kind::BilledCancelled {
                Cents(0)
            } else {
                want
            };
            out.push(Discrepancy {
                kind,
                po_id: po.po_id,
                invoice_number: None,
                supplier_id: po.supplier_id,
                expected: Some(expected),
                billed: got,
            });
        }
    }

    out.sort_by(|a, b| {
        (a.kind, a.po_id, &a.invoice_number).cmp(&(b.kind, b.po_id, &b.invoice_number))
    });
    out
}
