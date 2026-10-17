// Hidden: produces the expected output for ticket 1's drills.
use rusqlite::Connection;
use std::error::Error;

struct Supplier {
    supplier_id: i64,
    name: String,
    payment_terms_days: i64,
    active: bool,
}

fn load(db: &Connection) -> rusqlite::Result<Vec<Supplier>> {
    let mut stmt =
        db.prepare("SELECT supplier_id, name, payment_terms_days, active FROM suppliers")?;
    let rows = stmt.query_map([], |r| {
        Ok(Supplier {
            supplier_id: r.get(0)?,
            name: r.get(1)?,
            payment_terms_days: r.get(2)?,
            active: r.get(3)?,
        })
    })?;
    rows.collect()
}

fn main() -> Result<(), Box<dyn Error>> {
    let db = Connection::open("../../data/kettle_creek.db")?;
    let mut s = load(&db)?;
    println!(
        "{} suppliers, {} active",
        s.len(),
        s.iter().filter(|x| x.active).count()
    );
    println!("---");
    s.sort_by(|a, b| {
        b.payment_terms_days
            .cmp(&a.payment_terms_days)
            .then(a.name.cmp(&b.name))
    });
    for x in s.iter().take(5) {
        println!(
            "{:>3} days  {} ({})",
            x.payment_terms_days, x.name, x.supplier_id
        );
    }
    println!("---");
    // Drill 3: two text columns swapped in SELECT: compiles and runs, silently wrong.
    let mut stmt = db.prepare("SELECT supplier_id, state, name FROM suppliers")?;
    let (id, name, state): (i64, String, String) =
        stmt.query_row([], |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?)))?;
    println!("supplier {id}: name = {name:?}, state = {state:?}");
    // Drill 3: an integer column read as String: a runtime error.
    let mut stmt = db.prepare("SELECT name, supplier_id FROM suppliers")?;
    let bad: rusqlite::Result<(i64, String)> = stmt.query_row([], |r| Ok((r.get(0)?, r.get(1)?)));
    println!("{}", bad.unwrap_err());
    Ok(())
}
