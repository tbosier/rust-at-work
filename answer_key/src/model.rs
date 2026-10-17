use crate::money::Cents;

#[derive(Debug, Clone)]
pub struct Supplier {
    pub supplier_id: i64,
    pub name: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    Open,
    Received,
    Cancelled,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Branch {
    Main,
    Millbrook,
}

#[derive(Debug, Clone)]
pub struct PurchaseOrder {
    pub po_id: i64,
    pub supplier_id: i64,
    pub branch: Branch,
    pub buyer: String,
    pub order_date: String,
    pub received_date: Option<String>,
    pub status: Status,
}

#[derive(Debug, Clone)]
pub struct PoLine {
    pub po_id: i64,
    pub quantity: i64,
    pub unit_price: Cents,
}

#[derive(Debug, Clone)]
pub struct Invoice {
    pub invoice_id: i64,
    pub invoice_number: String,
    pub supplier_id: i64,
    pub po_id: i64,
    pub invoice_date: String,
    pub amount: Cents,
}

/// One row of `branch_invoices`, exactly as the old system left it.
#[derive(Debug, Clone)]
pub struct BranchRow {
    pub reference: Option<String>,
    pub vendor: Option<String>,
    pub po_ref: Option<String>,
    pub inv_date: Option<String>,
    pub amount: Option<String>,
}
