use std::fs::File;
use std::io::Write;

// Reads a count typed on a slip and writes it to a report.
fn count_report(path: &str, typed: &str) -> std::io::Result<()> {
    let count: u32 = typed.trim().parse()?;
    let mut file = File::create(path)?;
    writeln!(file, "count,{count}")?;
    Ok(())
}

fn main() {
    println!("{:?}", count_report("count.csv", " 12 "));
}
