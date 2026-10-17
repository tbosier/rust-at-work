// Hidden: builds a tiny workbook, then lists the files inside it with unzip -l.
use rust_xlsxwriter::{Format, Workbook};
use std::error::Error;
use std::process::Command;

fn main() -> Result<(), Box<dyn Error>> {
    let path = std::env::temp_dir().join("library_fines.xlsx");
    let mut wb = Workbook::new();
    let money = Format::new().set_num_format("$#,##0.00");
    let ws = wb.add_worksheet();
    ws.set_name("Fines")?;
    ws.write_string(0, 0, "Loan")?;
    ws.write_string(0, 1, "Amount")?;
    ws.write_number(1, 0, 6)?;
    ws.write_number_with_format(1, 1, 24.95, &money)?;
    wb.save(&path)?;

    let list = Command::new("unzip").arg("-l").arg(&path).output()?;
    let text = String::from_utf8(list.stdout)?;
    for line in text.lines().skip(1) {
        // Columns: length, date, time, name. Blank the date and time so the
        // listing doesn't change every run.
        let cols: Vec<&str> = line.split_whitespace().collect();
        if cols.len() == 4 && cols[0].bytes().all(|b| b.is_ascii_digit()) {
            println!("{:>9}  {:<10} {:<5}   {}", cols[0], "(date)", "", cols[3]);
        } else {
            println!("{line}");
        }
    }
    Ok(())
}
