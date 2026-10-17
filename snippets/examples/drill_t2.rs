// Hidden: expected output for ticket 2's drills and sanity checks.
use rusqlite::Connection;
use std::collections::HashMap;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let db = Connection::open("../../data/kettle_creek.db")?;
    let mut stmt = db.prepare("SELECT sku, category, supplier_id, list_price FROM products")?;
    let products: Vec<(String, String, i64, f64)> = stmt
        .query_map([], |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?, r.get(3)?)))?
        .collect::<Result<_, _>>()?;
    let mut per_cat: HashMap<&str, usize> = HashMap::new();
    for p in &products {
        *per_cat.entry(p.1.as_str()).or_default() += 1;
    }
    let mut v: Vec<_> = per_cat.into_iter().collect();
    v.sort();
    for (c, n) in v {
        println!("{c:<20} {n:>3}");
    }
    println!("---");
    let mut stmt = db.prepare("SELECT supplier_id, name FROM suppliers")?;
    let names: HashMap<i64, String> = stmt
        .query_map([], |r| Ok((r.get(0)?, r.get(1)?)))?
        .collect::<Result<_, _>>()?;
    let mut by: HashMap<i64, (usize, f64)> = HashMap::new();
    for p in &products {
        let e = by.entry(p.2).or_default();
        e.0 += 1;
        e.1 += p.3;
    }
    let mut avg: Vec<(&str, usize, f64)> = by
        .iter()
        .map(|(id, (n, t))| (names[id].as_str(), *n, t / *n as f64))
        .collect();
    avg.sort_by(|a, b| b.2.total_cmp(&a.2).then_with(|| a.0.cmp(b.0)));
    for (name, n, a) in avg.iter().take(3) {
        println!("{name:<28} {n} products, average list price {a:.2}");
    }
    println!("---");
    let mut stmt = db.prepare("SELECT supplier_id, invoice_date, amount FROM invoices")?;
    let inv: Vec<(i64, String, f64)> = stmt
        .query_map([], |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?)))?
        .collect::<Result<_, _>>()?;
    let q3: Vec<_> = inv
        .iter()
        .filter(|i| i.1.as_str() >= "2026-07-01" && i.1.as_str() <= "2026-09-30")
        .collect();
    let s1: Vec<_> = q3.iter().filter(|i| i.0 == 1).collect();
    let t: f64 = s1.iter().map(|i| i.2).sum();
    eprintln!("loaded {} invoices", inv.len());
    println!("{} invoices totaling {:.2}", s1.len(), t);
    println!("---");
    println!("{t}");
    Ok(())
}
