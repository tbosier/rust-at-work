use rusqlite::Connection;
use snippets::library;
use std::error::Error;

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

const CUTOFF: &str = "2026-09-15";

fn out_before_cutoff(loans: &[Loan]) -> Vec<&Loan> {
    loans
        .iter()
        .filter(|loan| loan.status == "out" && loan.loaned_on.as_str() < CUTOFF)
        .collect()
}

// region: write
use std::fs::File;
use std::io::{BufWriter, Write};

fn write_report(path: &str, loans: &[&Loan]) -> std::io::Result<()> {
    let file = File::create(path)?;
    let mut out = BufWriter::new(file);
    writeln!(out, "loan_id,member,loaned_on")?;
    for loan in loans {
        writeln!(out, "{},{},{}", loan.loan_id, loan.member, loan.loaned_on)?;
    }
    out.flush()?;
    Ok(())
}
// endregion

fn main() -> Result<(), Box<dyn Error>> {
    let db = library()?;
    let loans = load_loans(&db)?;
    // Run in a scratch folder, so the files land there.
    let dir = std::env::temp_dir().join("snip_t1_write");
    std::fs::create_dir_all(&dir)?;
    std::env::set_current_dir(&dir)?;
    // region: call
    let mut early = out_before_cutoff(&loans);
    early.sort_by_key(|loan| (loan.member.as_str(), loan.loan_id));
    std::fs::create_dir_all("out")?; // File::create makes files, not folders
    write_report("out/early_loans.csv", &early)?;
    print!("{}", std::fs::read_to_string("out/early_loans.csv")?);
    // endregion
    println!("---");
    // region: missing
    let result = write_report("reports/early_loans.csv", &early);
    println!("{result:?}");
    // endregion
    Ok(())
}
