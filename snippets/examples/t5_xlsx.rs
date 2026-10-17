use rusqlite::Connection;
use snippets::library;
use std::error::Error;

struct Fine {
    loan_id: i64,
    cents: i64,
}

fn fines(db: &Connection) -> rusqlite::Result<Vec<Fine>> {
    let mut stmt = db.prepare("SELECT loan_id, amount FROM fines")?;
    let rows = stmt.query_map([], |r| {
        let dollars: f64 = r.get(1)?;
        Ok(Fine {
            loan_id: r.get(0)?,
            cents: (dollars * 100.0).round() as i64,
        })
    })?;
    rows.collect()
}

// region: write
use rust_xlsxwriter::{Format, FormatBorder, Workbook, XlsxError};

fn write_fines(path: &str, fines: &[Fine]) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();
    let header = Format::new()
        .set_bold()
        .set_border_bottom(FormatBorder::Thin);
    let money = Format::new().set_num_format("$#,##0.00;[Red]($#,##0.00)");

    let sheet = workbook.add_worksheet();
    sheet.set_name("Fines")?;
    sheet.write_string_with_format(0, 0, "Loan", &header)?;
    sheet.write_string_with_format(0, 1, "Amount", &header)?;
    for (i, fine) in fines.iter().enumerate() {
        let row = i as u32 + 1;
        sheet.write_number(row, 0, fine.loan_id as f64)?;
        sheet.write_number_with_format(row, 1, fine.cents as f64 / 100.0, &money)?;
    }
    sheet.set_column_width(0, 8)?;
    sheet.set_column_width(1, 14)?;
    sheet.set_freeze_panes(1, 0)?;
    sheet.autofilter(0, 0, fines.len() as u32, 1)?;

    workbook.save(path)
}
// endregion

fn main() -> Result<(), Box<dyn Error>> {
    let db = library()?;
    // region: use
    let mut all = fines(&db)?;
    all.push(Fine {
        loan_id: 4,
        cents: -30,
    }); // a refund
    write_fines("library_fines.xlsx", &all)?;
    let size = std::fs::metadata("library_fines.xlsx")?.len();
    println!(
        "wrote library_fines.xlsx: {} fines, {} KB",
        all.len(),
        size / 1024
    );
    // endregion
    Ok(())
}
