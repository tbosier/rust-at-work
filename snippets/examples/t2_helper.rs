// region: helper
use std::error::Error;
use std::fs::File;
use std::io::Write;

// Reads a count typed on a slip and writes it to a report.
fn count_report(path: &str, typed: &str) -> Result<(), Box<dyn Error>> {
    let count: u32 = typed.trim().parse()?;
    let mut file = File::create(path)?;
    writeln!(file, "count,{count}")?;
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    count_report("count.csv", " 12 ")?;
    println!("{:?}", count_report("count.csv", "twelve"));
    Ok(())
}
// endregion
