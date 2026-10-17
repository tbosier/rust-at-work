use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let path = std::env::temp_dir().join("snip_t2_books.csv");
    // region: csv
    let mut w = csv::Writer::from_path(&path)?;
    w.write_record(["book_id", "title", "fines"])?;
    w.write_record(["101", "The Mythical Man-Month", "0.30"])?;
    w.write_record(["106", "Programming Pearls, 2nd ed.", "1.10"])?;
    w.write_record(["107", "The \"Dragon Book\"", "0.00"])?;
    let total = 1.1_f64;
    w.write_record(["108".to_string(), "Code".to_string(), format!("{total:.2}")])?;
    w.flush()?;
    // endregion
    print!("{}", std::fs::read_to_string(&path)?);
    Ok(())
}
