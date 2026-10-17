use std::error::Error;

struct Loan {
    loan_id: i64,
    member: String,
    loaned_on: String,
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
    let a = Loan {
        loan_id: 2,
        member: "Grace".into(),
        loaned_on: "2026-08-11".into(),
    };
    let b = Loan {
        loan_id: 3,
        member: "Alan".into(),
        loaned_on: "2026-09-01".into(),
    };
    let dir = std::env::temp_dir().join("snip_t1_write.csv");
    write_report(dir.to_str().unwrap(), &[&a, &b])?;
    print!("{}", std::fs::read_to_string(&dir)?);
    Ok(())
}
