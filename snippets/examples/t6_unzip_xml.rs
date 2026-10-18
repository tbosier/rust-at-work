// Hidden: builds the same tiny workbook, then prints parts of its XML.
use rust_xlsxwriter::{Format, Workbook};
use std::error::Error;
use std::process::Command;

fn main() -> Result<(), Box<dyn Error>> {
    let path = std::env::temp_dir().join("library_fines_xml.xlsx");
    let mut wb = Workbook::new();
    let money = Format::new().set_num_format("$#,##0.00");
    let ws = wb.add_worksheet();
    ws.set_name("Fines")?;
    ws.write_string(0, 0, "Loan")?;
    ws.write_string(0, 1, "Amount")?;
    ws.write_number(1, 0, 6)?;
    ws.write_number_with_format(1, 1, 24.95, &money)?;
    wb.save(&path)?;

    let sheet = Command::new("unzip")
        .arg("-p")
        .arg(&path)
        .arg("xl/worksheets/sheet1.xml")
        .output()?;
    let xml = String::from_utf8(sheet.stdout)?;
    let start = xml.find("<sheetData>").ok_or("no sheetData")?;
    let end = xml.find("</sheetData>").ok_or("no sheetData end")? + "</sheetData>".len();
    println!(
        "{}",
        xml[start..end]
            .replace("<row", "\n  <row")
            .replace("</sheetData>", "\n</sheetData>")
    );
    println!("---");
    let strings = Command::new("unzip")
        .arg("-p")
        .arg(&path)
        .arg("xl/sharedStrings.xml")
        .output()?;
    let xml = String::from_utf8(strings.stdout)?;
    let start = xml.find("<si>").ok_or("no strings")?;
    let end = xml.rfind("</si>").ok_or("no strings end")? + "</si>".len();
    println!("{}", &xml[start..end]);
    Ok(())
}
