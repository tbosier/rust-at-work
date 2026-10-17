// region: all
use rusqlite::Connection;
use snippets::library;
use std::error::Error;

#[derive(Debug)]
struct Loan {
    loan_id: i64,
    member: String,
    loaned_on: String,
    returned_on: Option<String>,
    status: String,
}

fn load_loans(db: &Connection) -> rusqlite::Result<Vec<Loan>> {
    let mut stmt =
        db.prepare("SELECT loan_id, member, loaned_on, returned_on, status FROM loans")?;
    let rows = stmt.query_map([], |row| {
        Ok(Loan {
            loan_id: row.get(0)?,
            member: row.get(1)?,
            loaned_on: row.get(2)?,
            returned_on: row.get(3)?,
            status: row.get(4)?,
        })
    })?;
    rows.collect()
}

fn main() -> Result<(), Box<dyn Error>> {
    let db = library()?;
    let loans = load_loans(&db)?;
    println!("loaded {} loans", loans.len());
    for loan in &loans {
        println!(
            "{} {:<6} {} {:?} {}",
            loan.loan_id, loan.member, loan.loaned_on, loan.returned_on, loan.status
        );
    }
    Ok(())
}
// endregion
