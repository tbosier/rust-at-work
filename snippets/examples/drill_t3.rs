// Hidden: real numbers for ticket 3's drills and its float measurement.
use rusqlite::Connection;
use std::collections::{HashMap, HashSet};
use std::error::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Terms {
    Net15,
    Net30,
    Net45,
    Net60,
    Other(i64),
}

fn terms(days: i64) -> Terms {
    match days {
        15 => Terms::Net15,
        30 => Terms::Net30,
        45 => Terms::Net45,
        60 => Terms::Net60,
        d => Terms::Other(d),
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let db = Connection::open("../../data/kettle_creek.db")?;

    // Drill 1: suppliers by payment terms.
    let mut stmt = db.prepare("SELECT payment_terms_days FROM suppliers")?;
    let days: Vec<i64> = stmt
        .query_map([], |r| r.get(0))?
        .collect::<Result<_, _>>()?;
    let mut by: HashMap<Terms, usize> = HashMap::new();
    for d in &days {
        *by.entry(terms(*d)).or_default() += 1;
    }
    let mut rows: Vec<_> = by.into_iter().collect();
    rows.sort();
    for (t, n) in rows {
        println!("{t:?}: {n}");
    }
    println!("---");

    // Drill 2: list price per category, f64 versus cents.
    let mut stmt = db.prepare("SELECT category, list_price FROM products")?;
    let prods: Vec<(String, f64)> = stmt
        .query_map([], |r| Ok((r.get(0)?, r.get(1)?)))?
        .collect::<Result<_, _>>()?;
    let mut f: HashMap<&str, f64> = HashMap::new();
    let mut c: HashMap<&str, i64> = HashMap::new();
    for (cat, p) in &prods {
        *f.entry(cat).or_default() += p;
        *c.entry(cat).or_default() += (p * 100.0).round() as i64;
    }
    let mut cats: Vec<&&str> = f.keys().collect();
    cats.sort();
    for cat in cats {
        let cents = c[*cat];
        println!(
            "{:<20} {:<22} {}.{:02}",
            cat,
            f[*cat],
            cents / 100,
            cents % 100
        );
    }
    println!("---");

    // Drill 3: products from inactive suppliers, through an index of references.
    let mut stmt = db.prepare("SELECT supplier_id, name, active FROM suppliers")?;
    let sups: Vec<(i64, String, bool)> = stmt
        .query_map([], |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?)))?
        .collect::<Result<_, _>>()?;
    let index: HashMap<i64, &(i64, String, bool)> = sups.iter().map(|s| (s.0, s)).collect();
    let mut stmt = db.prepare("SELECT sku, supplier_id FROM products")?;
    let all: Vec<(String, i64)> = stmt
        .query_map([], |r| Ok((r.get(0)?, r.get(1)?)))?
        .collect::<Result<_, _>>()?;
    let mut stale: Vec<(&str, &str)> = Vec::new();
    for (sku, sid) in &all {
        let Some(sup) = index.get(sid) else {
            println!("{sku}: unknown supplier {sid}");
            continue;
        };
        if !sup.2 {
            stale.push((sku, &sup.1));
        }
    }
    stale.sort();
    let names: HashSet<&str> = stale.iter().map(|s| s.1).collect();
    println!(
        "{} products from {} inactive suppliers",
        stale.len(),
        names.len()
    );
    for (sku, name) in stale.iter().take(3) {
        println!("  {sku}  {name}");
    }
    println!("---");

    // Measurement: received MAIN POs, f64 == versus cents ==.
    let mut stmt = db.prepare("SELECT po_id, branch, status FROM purchase_orders")?;
    let pos: Vec<(i64, String, String)> = stmt
        .query_map([], |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?)))?
        .collect::<Result<_, _>>()?;
    let mut stmt = db.prepare("SELECT po_id, quantity, unit_price FROM po_lines")?;
    let lines: Vec<(i64, i64, f64)> = stmt
        .query_map([], |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?)))?
        .collect::<Result<_, _>>()?;
    let mut stmt = db.prepare("SELECT invoice_id, invoice_number, po_id, amount FROM invoices")?;
    let mut invs: Vec<(i64, String, i64, f64)> = stmt
        .query_map([], |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?, r.get(3)?)))?
        .collect::<Result<_, _>>()?;
    invs.sort_by_key(|i| i.0);
    let mut want_f: HashMap<i64, f64> = HashMap::new();
    let mut want_c: HashMap<i64, i64> = HashMap::new();
    for (po, q, p) in &lines {
        *want_f.entry(*po).or_default() += *q as f64 * p;
        *want_c.entry(*po).or_default() += q * (p * 100.0).round() as i64;
    }
    let mut seen = HashSet::new();
    let mut got_f: HashMap<i64, f64> = HashMap::new();
    let mut got_c: HashMap<i64, i64> = HashMap::new();
    for (_, num, po, a) in &invs {
        if seen.insert((*po, num.as_str())) {
            *got_f.entry(*po).or_default() += a;
            *got_c.entry(*po).or_default() += (a * 100.0).round() as i64;
        }
    }
    let (mut billed, mut f_bad, mut c_bad, mut f_false) = (0, 0, 0, 0);
    for (po, branch, status) in &pos {
        if branch != "MAIN" || status != "received" || !got_c.contains_key(po) {
            continue;
        }
        billed += 1;
        let fe = want_f[po] != got_f[po];
        let ce = want_c[po] != got_c[po];
        f_bad += fe as i32;
        c_bad += ce as i32;
        f_false += (fe && !ce) as i32;
    }
    println!("received MAIN POs with invoices: {billed}");
    println!("mismatched by f64 ==: {f_bad}");
    println!("mismatched in cents:  {c_bad}");
    println!("false alarms from f64: {f_false}");
    let multi = {
        let mut n: HashMap<i64, usize> = HashMap::new();
        for (_, _, po, _) in &invs {
            *n.entry(*po).or_default() += 1;
        }
        n.values().filter(|&&k| k > 1).count()
    };
    let main_received = pos
        .iter()
        .filter(|p| p.1 == "MAIN" && p.2 == "received")
        .count();
    println!("MAIN received POs: {main_received}; POs with 2+ invoice rows: {multi}");
    Ok(())
}
