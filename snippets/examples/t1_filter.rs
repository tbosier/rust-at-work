use rusqlite::Connection;
use snippets::library;
use std::error::Error;

#[derive(Debug)]
struct Loan {
    loan_id: i64,
    member: String,
    loaned_on: String,
    status: String,
}

fn load_loans(db: &Connection) -> rusqlite::Result<Vec<Loan>> {
    let mut stmt = db.prepare("SELECT loan_id, member, loaned_on, status FROM loans")?;
    let rows = stmt.query_map([], |row| {
        Ok(Loan {
            loan_id: row.get(0)?,
            member: row.get(1)?,
            loaned_on: row.get(2)?,
            status: row.get(3)?,
        })
    })?;
    rows.collect()
}

// region: count
fn count_out(loans: &[Loan]) -> usize {
    loans.iter().filter(|loan| loan.status == "out").count()
}
// endregion

// region: early
fn out_before<'a>(loans: &'a [Loan], cutoff: &str) -> Vec<&'a Loan> {
    loans
        .iter()
        .filter(|loan| loan.status == "out" && loan.loaned_on.as_str() < cutoff)
        .collect()
}
// endregion

fn main() -> Result<(), Box<dyn Error>> {
    let db = library()?;
    let loans = load_loans(&db)?;
    // region: use
    println!("out right now: {}", count_out(&loans));

    let mut early = out_before(&loans, "2026-09-15");
    early.sort_by_key(|loan| (loan.member.as_str(), loan.loan_id));
    for loan in &early {
        println!("{} {} {}", loan.member, loan.loan_id, loan.loaned_on);
    }
    // endregion
    Ok(())
}
