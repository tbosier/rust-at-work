// Hidden: expected output for ticket 4's drills.
use rusqlite::Connection;
use std::collections::BTreeMap;

#[derive(Debug, PartialEq)]
enum Size {
    Inch { num: u32, den: u32 },
    MetricThread(u32),
    Millimeters(u32),
}

#[derive(Debug, PartialEq)]
enum SizeError {
    NotASize(String),
    ZeroDenominator,
}

fn number(s: &str) -> Option<u32> {
    if s.is_empty() || !s.bytes().all(|b| b.is_ascii_digit()) {
        return None;
    }
    s.parse().ok()
}

fn parse_size(word: &str) -> Result<Size, SizeError> {
    let bad = || SizeError::NotASize(word.to_string());
    if let Some(frac) = word.strip_suffix("in") {
        let (n, d) = frac.split_once('/').ok_or_else(bad)?;
        let (num, den) = (number(n).ok_or_else(bad)?, number(d).ok_or_else(bad)?);
        if den == 0 {
            return Err(SizeError::ZeroDenominator);
        }
        return Ok(Size::Inch { num, den });
    }
    if let Some(mm) = word.strip_suffix("mm") {
        return number(mm).map(Size::Millimeters).ok_or_else(bad);
    }
    if let Some(m) = word.strip_prefix('M') {
        return number(m).map(Size::MetricThread).ok_or_else(bad);
    }
    Err(bad())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db = Connection::open("../../data/kettle_creek.db")?;
    let mut stmt = db.prepare("SELECT description FROM products")?;
    let descs: Vec<String> = stmt
        .query_map([], |r| r.get(0))?
        .collect::<rusqlite::Result<_>>()?;
    let mut counts: BTreeMap<&str, usize> = BTreeMap::new();
    for d in &descs {
        let last = d.split_whitespace().last().unwrap_or("");
        let kind = match parse_size(last) {
            Ok(Size::Inch { .. }) => "inch",
            Ok(Size::MetricThread(_)) => "metric thread",
            Ok(Size::Millimeters(_)) => "millimeters",
            Err(_) => "no size",
        };
        *counts.entry(kind).or_default() += 1;
    }
    println!("{} descriptions", descs.len());
    for (k, n) in &counts {
        println!("{k:<14} {n}");
    }
    println!("---");
    for w in ["3/8in", "M10", "25mm", "1/0in", "M", "duty", "1/2"] {
        println!("{w:<6} {:?}", parse_size(w));
    }
    Ok(())
}
