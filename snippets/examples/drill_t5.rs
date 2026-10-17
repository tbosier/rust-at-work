// Hidden: produces the expected output for ticket 5's drills.
use clap::Parser;
use rusqlite::Connection;
use rust_xlsxwriter::{Format, Workbook};
use std::error::Error;

#[derive(Parser)]
struct Args {
    #[arg(long)]
    category: String,
}

struct Product {
    sku: String,
    description: String,
    category: String,
    list_price: f64,
}

struct Supplier {
    supplier_id: i64,
    name: String,
    state: String,
}

trait TerminalRow {
    fn headers() -> &'static [&'static str];
    fn cells(&self) -> Vec<String>;
}

impl TerminalRow for Supplier {
    fn headers() -> &'static [&'static str] {
        &["id", "name", "state"]
    }
    fn cells(&self) -> Vec<String> {
        vec![
            self.supplier_id.to_string(),
            self.name.clone(),
            self.state.clone(),
        ]
    }
}

impl TerminalRow for Product {
    fn headers() -> &'static [&'static str] {
        &["sku", "description", "price"]
    }
    fn cells(&self) -> Vec<String> {
        vec![
            self.sku.clone(),
            self.description.clone(),
            format!("{:.2}", self.list_price),
        ]
    }
}

fn print_table<T: TerminalRow>(rows: &[T]) {
    let mut widths: Vec<usize> = T::headers().iter().map(|h| h.len()).collect();
    let cells: Vec<Vec<String>> = rows.iter().map(|r| r.cells()).collect();
    for row in &cells {
        for (w, c) in widths.iter_mut().zip(row) {
            *w = (*w).max(c.len());
        }
    }
    let line = |vals: Vec<&str>| {
        let parts: Vec<String> = vals
            .iter()
            .zip(&widths)
            .map(|(v, w)| format!("{v:<w$}"))
            .collect();
        println!("{}", parts.join("  ").trim_end());
    };
    line(T::headers().to_vec());
    for row in &cells {
        line(row.iter().map(|s| s.as_str()).collect());
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let db = Connection::open("../../data/kettle_creek.db")?;
    let mut stmt = db.prepare("SELECT sku, description, category, list_price FROM products")?;
    let products: Vec<Product> = stmt
        .query_map([], |r| {
            Ok(Product {
                sku: r.get(0)?,
                description: r.get(1)?,
                category: r.get(2)?,
                list_price: r.get(3)?,
            })
        })?
        .collect::<rusqlite::Result<_>>()?;
    let mut stmt = db.prepare("SELECT supplier_id, name, state FROM suppliers")?;
    let suppliers: Vec<Supplier> = stmt
        .query_map([], |r| {
            Ok(Supplier {
                supplier_id: r.get(0)?,
                name: r.get(1)?,
                state: r.get(2)?,
            })
        })?
        .collect::<rusqlite::Result<_>>()?;

    let args = Args::try_parse_from(["drills", "--category", "Bearings"])?;
    let mut picked: Vec<&Product> = products
        .iter()
        .filter(|p| p.category == args.category)
        .collect();
    picked.sort_by(|a, b| a.sku.cmp(&b.sku));
    println!("{} products in {}", picked.len(), args.category);
    for p in picked.iter().take(3) {
        println!("{}  {}", p.sku, p.description);
    }
    println!("---");
    print_table(&suppliers[..4]);
    println!();
    let mut bearings: Vec<Product> = products
        .into_iter()
        .filter(|p| p.category == "Bearings")
        .collect();
    bearings.sort_by(|a, b| a.sku.cmp(&b.sku));
    print_table(&bearings[..3]);
    println!("---");
    let mut wb = Workbook::new();
    let bold = Format::new().set_bold();
    let ws = wb.add_worksheet();
    ws.set_name("Suppliers")?;
    for (c, h) in ["supplier_id", "name", "state"].iter().enumerate() {
        ws.write_string_with_format(0, c as u16, *h, &bold)?;
    }
    for (i, s) in suppliers.iter().enumerate() {
        let r = i as u32 + 1;
        ws.write_number(r, 0, s.supplier_id as f64)?;
        ws.write_string(r, 1, &s.name)?;
        ws.write_string(r, 2, &s.state)?;
    }
    wb.save("suppliers.xlsx")?;
    println!(
        "wrote suppliers.xlsx: 1 sheet, {} suppliers",
        suppliers.len()
    );
    Ok(())
}
