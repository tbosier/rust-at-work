//! A few raw Millbrook rows in, accepted invoices and rejects out.

use kettle::branch::{Field, parse_all};
use kettle::model::{Branch, BranchRow, PurchaseOrder, Status, Supplier};

fn row(r: &str, vendor: &str, po: &str, date: &str, amount: &str) -> BranchRow {
    let s = |t: &str| Some(t.to_string());
    BranchRow {
        reference: s(r),
        vendor: s(vendor),
        po_ref: s(po),
        inv_date: s(date),
        amount: s(amount),
    }
}

#[test]
fn rows_are_accepted_or_rejected_with_the_first_problem() {
    let suppliers = vec![
        Supplier {
            supplier_id: 1,
            name: "Quarrow Bearing Co.".into(),
        },
        Supplier {
            supplier_id: 2,
            name: "Tamsk Ridge Seals & Gaskets".into(),
        },
    ];
    let pos = vec![PurchaseOrder {
        po_id: 104233,
        supplier_id: 2,
        branch: Branch::Millbrook,
        buyer: "T. Haskins".into(),
        order_date: "2026-08-01".into(),
        received_date: Some("2026-08-10".into()),
        status: Status::Received,
    }];
    let rows = vec![
        row(
            "MB-1",
            " TAMSK RIDGE SEALS AND GASKETS ",
            "PO-104233",
            "8/14/2026",
            "$1,204.50",
        ),
        row(
            "MB-2",
            "Tamsk Ridge Seals & Gaskets",
            "104233",
            "2026-08-15",
            "(45.00)",
        ),
        row(
            "MB-1",
            "Tamsk Ridge Seals & Gaskets",
            "104233",
            "2026-08-15",
            "1.00",
        ),
        row("MB-3", "Tamsk Ridge", "104233", "2026-08-15", "1.00"),
        row("MB-4", "Quarrow Bearing Co", "104233", "2026-08-15", "1.00"),
        row(
            "MB-5",
            "Tamsk Ridge Seals & Gaskets",
            "104233",
            "02/29/2025",
            "1.00",
        ),
        row(
            "MB-6",
            "Tamsk Ridge Seals & Gaskets",
            "104233",
            "2026-08-15",
            "12.345",
        ),
    ];
    let (good, bad) = parse_all(&rows, &suppliers, &pos);
    let amounts: Vec<i64> = good.iter().map(|g| g.amount.0).collect();
    assert_eq!(amounts, vec![120450, -4500]);
    let fields: Vec<(&str, Field)> = bad
        .iter()
        .map(|r| (r.reference.as_str(), r.field))
        .collect();
    assert_eq!(
        fields,
        vec![
            ("MB-1", Field::Ref),
            ("MB-3", Field::Vendor),
            ("MB-4", Field::PoRef),
            ("MB-5", Field::InvDate),
            ("MB-6", Field::Amount),
        ]
    );
}
