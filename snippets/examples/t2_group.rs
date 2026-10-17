// region: all
use rusqlite::Connection;
use snippets::library;
use std::collections::HashMap;
use std::error::Error;

struct Loan {
    loan_id: i64,
    member: String,
}

struct Fine {
    loan_id: i64,
    amount: f64,
}

fn load_loans(db: &Connection) -> rusqlite::Result<Vec<Loan>> {
    let mut stmt = db.prepare("SELECT loan_id, member FROM loans")?;
    let rows = stmt.query_map([], |r| {
        Ok(Loan {
            loan_id: r.get(0)?,
            member: r.get(1)?,
        })
    })?;
    rows.collect()
}

fn load_fines(db: &Connection) -> rusqlite::Result<Vec<Fine>> {
    let mut stmt = db.prepare("SELECT loan_id, amount FROM fines")?;
    let rows = stmt.query_map([], |r| {
        Ok(Fine {
            loan_id: r.get(0)?,
            amount: r.get(1)?,
        })
    })?;
    rows.collect()
}

/// Total fines per member, with how many fines each.
fn fines_by_member<'a>(loans: &'a [Loan], fines: &[Fine]) -> HashMap<&'a str, (u32, f64)> {
    let member_of: HashMap<i64, &str> = loans
        .iter()
        .map(|l| (l.loan_id, l.member.as_str()))
        .collect();

    let mut totals: HashMap<&str, (u32, f64)> = HashMap::new();
    for fine in fines {
        let member = member_of[&fine.loan_id];
        let entry = totals.entry(member).or_insert((0, 0.0));
        entry.0 += 1;
        entry.1 += fine.amount;
    }
    totals
}

fn main() -> Result<(), Box<dyn Error>> {
    let db = library()?;
    let loans = load_loans(&db)?;
    let fines = load_fines(&db)?;
    let totals = fines_by_member(&loans, &fines);

    let mut rows: Vec<(&str, u32, f64)> = totals
        .into_iter()
        .map(|(m, (n, total))| (m, n, total))
        .collect();
    rows.sort_by(|a, b| b.2.total_cmp(&a.2).then_with(|| a.0.cmp(b.0)));

    for (member, n, total) in &rows {
        println!("{member:<7} {n} fines  {total:>6.2}   (unformatted: {total})");
    }
    Ok(())
}
// endregion
