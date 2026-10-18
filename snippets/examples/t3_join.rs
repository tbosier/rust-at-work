// Times a nested-loop join against a hash join on the real invoices and POs,
// copied k times over. Run with --release; the shown output is a release run.
use rusqlite::Connection;
// region: joins
use std::collections::HashMap;
// endregion
use std::error::Error;
use std::time::Instant;

// region: joins
struct Po {
    po_id: i64,
    supplier_id: i64,
}

struct Inv {
    po_id: i64,
}

fn nested_loop(invoices: &[Inv], pos: &[Po]) -> i64 {
    let mut checksum = 0;
    for inv in invoices {
        if let Some(po) = pos.iter().find(|p| p.po_id == inv.po_id) {
            checksum += po.supplier_id;
        }
    }
    checksum
}

fn hash_join(invoices: &[Inv], pos: &[Po]) -> i64 {
    let index: HashMap<i64, &Po> = pos.iter().map(|p| (p.po_id, p)).collect();
    let mut checksum = 0;
    for inv in invoices {
        if let Some(po) = index.get(&inv.po_id) {
            checksum += po.supplier_id;
        }
    }
    checksum
}
// endregion

fn main() -> Result<(), Box<dyn Error>> {
    let db = Connection::open("../../data/kettle_creek.db")?;
    let mut s = db.prepare("SELECT po_id, supplier_id FROM purchase_orders")?;
    let pos: Vec<Po> = s
        .query_map([], |r| {
            Ok(Po {
                po_id: r.get(0)?,
                supplier_id: r.get(1)?,
            })
        })?
        .collect::<Result<_, _>>()?;
    let mut s = db.prepare("SELECT po_id FROM invoices")?;
    let invs: Vec<Inv> = s
        .query_map([], |r| Ok(Inv { po_id: r.get(0)? }))?
        .collect::<Result<_, _>>()?;
    let ks: &[i64] = if cfg!(debug_assertions) {
        &[1, 2]
    } else {
        &[1, 4, 16, 64]
    };
    println!(
        "{:>9} {:>9} {:>14} {:>12}",
        "POs", "invoices", "nested loop", "hash join"
    );
    for &k in ks {
        let big_pos: Vec<Po> = (0..k)
            .flat_map(|c| {
                pos.iter().map(move |p| Po {
                    po_id: p.po_id + c * 1_000_000,
                    supplier_id: p.supplier_id,
                })
            })
            .collect();
        let big_inv: Vec<Inv> = (0..k)
            .flat_map(|c| {
                invs.iter().map(move |i| Inv {
                    po_id: i.po_id + c * 1_000_000,
                })
            })
            .collect();
        let t = Instant::now();
        let a = nested_loop(&big_inv, &big_pos);
        let tn = t.elapsed();
        let t = Instant::now();
        let b = hash_join(&big_inv, &big_pos);
        let th = t.elapsed();
        assert_eq!(a, b);
        println!(
            "{:>9} {:>9} {:>11.1} ms {:>9.2} ms",
            big_pos.len(),
            big_inv.len(),
            tn.as_secs_f64() * 1e3,
            th.as_secs_f64() * 1e3
        );
    }
    Ok(())
}
