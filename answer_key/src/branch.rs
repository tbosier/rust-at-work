//! Turn Millbrook's text-only rows into invoices, or into a reason they can't be.

use crate::dates::Date;
use crate::model::{BranchRow, PurchaseOrder, Supplier};
use crate::money::{Cents, parse_amount};
use std::collections::{HashMap, HashSet};
use std::fmt;

#[derive(Debug, Clone)]
pub struct BranchInvoice {
    pub reference: String,
    pub supplier_id: i64,
    pub po_id: i64,
    pub date: Date,
    pub amount: Cents,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Field {
    Ref,
    Vendor,
    PoRef,
    InvDate,
    Amount,
}

impl fmt::Display for Field {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name = match self {
            Field::Ref => "ref",
            Field::Vendor => "vendor",
            Field::PoRef => "po_ref",
            Field::InvDate => "inv_date",
            Field::Amount => "amount",
        };
        f.write_str(name)
    }
}

#[derive(Debug, Clone)]
pub struct Reject {
    pub reference: String,
    pub field: Field,
    pub value: String,
    pub reason: String,
}

/// The matching rule from Dana's email: ignore case, outer and repeated
/// spaces and a trailing period, and treat "and" and "&" as the same.
pub fn normalize_vendor(name: &str) -> String {
    let lower = name.to_lowercase();
    let words: Vec<&str> = lower.split_whitespace().collect();
    let joined = words.join(" ").replace(" and ", " & ");
    joined.strip_suffix('.').unwrap_or(&joined).to_string()
}

/// `104233`, `PO-104233`, `PO 104233`, with spaces around any of them.
pub fn parse_po_ref(raw: &str) -> Option<i64> {
    let s = raw.trim();
    let digits = s
        .strip_prefix("PO-")
        .or_else(|| s.strip_prefix("PO "))
        .unwrap_or(s);
    if digits.is_empty() || !digits.bytes().all(|b| b.is_ascii_digit()) {
        return None;
    }
    digits.parse().ok()
}

pub struct Parser<'a> {
    vendors: HashMap<String, i64>,
    pos: HashMap<i64, &'a PurchaseOrder>,
    seen: HashSet<String>,
}

impl<'a> Parser<'a> {
    pub fn new(suppliers: &[Supplier], pos: &'a [PurchaseOrder]) -> Parser<'a> {
        Parser {
            vendors: suppliers
                .iter()
                .map(|s| (normalize_vendor(&s.name), s.supplier_id))
                .collect(),
            pos: pos.iter().map(|p| (p.po_id, p)).collect(),
            seen: HashSet::new(),
        }
    }

    /// Fields are checked in column order; the first problem is the one reported.
    pub fn parse(&mut self, row: &BranchRow) -> Result<BranchInvoice, Reject> {
        let reference = row.reference.as_deref().unwrap_or("").trim().to_string();
        let reject = |field: Field, value: &Option<String>, reason: String| Reject {
            reference: reference.clone(),
            field,
            value: value.clone().unwrap_or_default(),
            reason,
        };
        if reference.is_empty() {
            return Err(reject(Field::Ref, &row.reference, "missing".into()));
        }
        if !self.seen.insert(reference.clone()) {
            return Err(reject(Field::Ref, &row.reference, "duplicate ref".into()));
        }

        let vendor = row.vendor.as_deref().unwrap_or("");
        let Some(&supplier_id) = self.vendors.get(&normalize_vendor(vendor)) else {
            return Err(reject(Field::Vendor, &row.vendor, "unknown vendor".into()));
        };

        let Some(po_id) = parse_po_ref(row.po_ref.as_deref().unwrap_or("")) else {
            return Err(reject(Field::PoRef, &row.po_ref, "not a PO number".into()));
        };
        let Some(po) = self.pos.get(&po_id) else {
            return Err(reject(Field::PoRef, &row.po_ref, "unknown PO".into()));
        };
        if po.supplier_id != supplier_id {
            return Err(reject(
                Field::PoRef,
                &row.po_ref,
                "PO belongs to another supplier".into(),
            ));
        }

        let date = Date::parse(row.inv_date.as_deref().unwrap_or(""))
            .map_err(|e| reject(Field::InvDate, &row.inv_date, e.to_string()))?;
        let amount = parse_amount(row.amount.as_deref().unwrap_or(""))
            .map_err(|e| reject(Field::Amount, &row.amount, e.to_string()))?;

        Ok(BranchInvoice {
            reference,
            supplier_id,
            po_id,
            date,
            amount,
        })
    }
}

pub fn parse_all(
    rows: &[BranchRow],
    suppliers: &[Supplier],
    pos: &[PurchaseOrder],
) -> (Vec<BranchInvoice>, Vec<Reject>) {
    let mut parser = Parser::new(suppliers, pos);
    let mut good = Vec::new();
    let mut bad = Vec::new();
    for row in rows {
        match parser.parse(row) {
            Ok(inv) => good.push(inv),
            Err(r) => bad.push(r),
        }
    }
    (good, bad)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vendor_names_match_loosely_but_not_too_loosely() {
        let canon = normalize_vendor("Tamsk Ridge Seals & Gaskets");
        assert_eq!(normalize_vendor("  TAMSK RIDGE SEALS AND GASKETS "), canon);
        assert_eq!(
            normalize_vendor("Quarrow Bearing Co"),
            normalize_vendor("Quarrow Bearing Co.")
        );
        assert_ne!(
            normalize_vendor("Old Varrow"),
            normalize_vendor("Old Varrow Hardware")
        );
    }

    #[test]
    fn po_refs() {
        assert_eq!(parse_po_ref("104233"), Some(104233));
        assert_eq!(parse_po_ref("PO-104233"), Some(104233));
        assert_eq!(parse_po_ref(" PO 104233 "), Some(104233));
        assert_eq!(parse_po_ref(""), None);
        assert_eq!(parse_po_ref("see email"), None);
        assert_eq!(parse_po_ref("PO-"), None);
        assert_eq!(parse_po_ref("+104233"), None);
    }
}
