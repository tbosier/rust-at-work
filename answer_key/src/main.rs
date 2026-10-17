//! Answer key for "Rust at work", Part 1. Not for the learner.

use clap::{Parser, Subcommand};
use kettle::{branch, db, model::Status, money::Cents, pack, reconcile};
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs::File;
use std::io::{BufWriter, Write};

#[derive(Parser)]
struct Cli {
    #[arg(long, default_value = "../data/kettle_creek.db")]
    db: String,
    #[arg(long, default_value = "out")]
    out: String,
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand)]
enum Cmd {
    T1,
    T2,
    T3,
    T4,
    T5 {
        #[arg(long, value_parser = parse_month)]
        month: String,
    },
}

const CUTOFF: &str = "2026-09-04";

/// `2026-09`: four digits, a dash, two digits, month 01 to 12.
fn parse_month(s: &str) -> Result<String, String> {
    let ok = s.len() == 7
        && s.as_bytes()[4] == b'-'
        && s.bytes()
            .enumerate()
            .all(|(i, b)| i == 4 || b.is_ascii_digit())
        && matches!(
            &s[5..],
            "01" | "02" | "03" | "04" | "05" | "06" | "07" | "08" | "09" | "10" | "11" | "12"
        );
    if ok {
        Ok(s.to_string())
    } else {
        Err(format!("expected a month like 2026-09, got {s:?}"))
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    std::fs::create_dir_all(&cli.out)?;
    let conn = db::open(&cli.db)?;
    let out = |name: &str| format!("{}/{}", cli.out, name);
    match cli.cmd {
        Cmd::T1 => {
            let pos = db::purchase_orders(&conn)?;
            let open: Vec<_> = pos.iter().filter(|p| p.status == Status::Open).collect();
            let mut stale: Vec<_> = open
                .iter()
                .filter(|p| p.order_date.as_str() < "2026-08-06")
                .collect();
            stale.sort_by(|a, b| (&a.order_date, a.po_id).cmp(&(&b.order_date, b.po_id)));
            println!("open POs: {}", open.len());
            println!("stale (ordered before 2026-08-06): {}", stale.len());
            let mut f = BufWriter::new(File::create(out("stale_open_pos.csv"))?);
            writeln!(f, "po_id,supplier_id,buyer,order_date")?;
            for p in &stale {
                writeln!(
                    f,
                    "{},{},{},{}",
                    p.po_id, p.supplier_id, p.buyer, p.order_date
                )?;
            }
        }
        Cmd::T2 => {
            let suppliers = db::suppliers(&conn)?;
            let names: HashMap<i64, &str> = suppliers
                .iter()
                .map(|s| (s.supplier_id, s.name.as_str()))
                .collect();
            let invoices = db::invoices(&conn)?;
            let q3 = |d: &str| ("2026-07-01"..="2026-09-30").contains(&d);
            let mut by: HashMap<i64, (u32, Cents)> = HashMap::new();
            for inv in invoices.iter().filter(|i| q3(&i.invoice_date)) {
                let e = by.entry(inv.supplier_id).or_default();
                e.0 += 1;
                e.1 += inv.amount;
            }
            let mut rows: Vec<_> = by.into_iter().collect();
            rows.sort_by(|a, b| b.1.1.cmp(&a.1.1).then_with(|| names[&a.0].cmp(names[&b.0])));
            let mut w = csv::Writer::from_path(out("spend_by_supplier_2026Q3.csv"))?;
            w.write_record(["supplier_id", "supplier", "invoices", "total"])?;
            for (id, (n, total)) in &rows {
                w.write_record([
                    id.to_string(),
                    names[id].to_string(),
                    n.to_string(),
                    total.to_string(),
                ])?;
            }
            w.flush()?;
            let grand: Cents = rows.iter().map(|r| r.1.1).sum();
            println!(
                "suppliers: {}  invoices: {}  total: {}",
                rows.len(),
                rows.iter().map(|r| r.1.0).sum::<u32>(),
                grand
            );
            // The float version, for the guide's float discussion.
            let raw = db::invoice_amounts_f64(&conn)?;
            let mut f: HashMap<i64, f64> = HashMap::new();
            for (sid, _, a) in raw.iter().filter(|r| q3(&r.1)) {
                *f.entry(*sid).or_default() += a;
            }
            let mut noisy: Vec<_> = f
                .iter()
                .filter(|(_, v)| {
                    format!("{v}")
                        .split('.')
                        .nth(1)
                        .is_some_and(|d| d.len() > 2)
                })
                .collect();
            noisy.sort_by_key(|(k, _)| **k);
            println!(
                "f64 totals that print with more than 2 decimals: {}",
                noisy.len()
            );
            for (k, v) in noisy.iter().take(3) {
                println!("  supplier {k}: {v}");
            }
            let fgrand: f64 = f.values().sum();
            println!("f64 grand total: {fgrand}");
        }
        Cmd::T3 | Cmd::T4 | Cmd::T5 { .. } => {
            let suppliers = db::suppliers(&conn)?;
            let names: HashMap<i64, &str> = suppliers
                .iter()
                .map(|s| (s.supplier_id, s.name.as_str()))
                .collect();
            let pos = db::purchase_orders(&conn)?;
            let lines = db::po_lines(&conn)?;
            let invoices = db::invoices(&conn)?;
            let with_branch = !matches!(cli.cmd, Cmd::T3);
            let (good, rejects) = if with_branch {
                branch::parse_all(&db::branch_rows(&conn)?, &suppliers, &pos)
            } else {
                (Vec::new(), Vec::new())
            };
            let input = reconcile::Inputs {
                pos: &pos,
                lines: &lines,
                invoices: &invoices,
                branch: &good,
                include_millbrook: with_branch,
                billing_cutoff: CUTOFF,
            };
            let ds = reconcile::reconcile(&input);
            let name_of = |id: i64| names.get(&id).copied().unwrap_or("");
            if let Cmd::T5 { month } = &cli.cmd {
                let dup: HashSet<(i64, &str)> = ds
                    .iter()
                    .filter(|d| d.kind == reconcile::Kind::DuplicateInvoice)
                    .map(|d| (d.po_id, d.invoice_number.as_deref().unwrap()))
                    .collect();
                // Drop the later copies only: keep the lowest invoice_id per (po, number).
                let mut sorted: Vec<_> = invoices.iter().collect();
                sorted.sort_by_key(|i| i.invoice_id);
                let mut seen = HashSet::new();
                let main: Vec<_> = sorted
                    .into_iter()
                    .filter(|i| {
                        !dup.contains(&(i.po_id, i.invoice_number.as_str()))
                            || seen.insert((i.po_id, i.invoice_number.clone()))
                    })
                    .cloned()
                    .collect();
                let spend = pack::spend_rows(month, &main, &good, &names);
                let month_invoices = spend.iter().map(|r| r.invoices).sum();
                let rows: Vec<_> = ds
                    .iter()
                    .map(|d| pack::DiscrepancyRow {
                        d,
                        supplier: name_of(d.supplier_id),
                    })
                    .collect();
                let p = pack::Pack {
                    month,
                    spend,
                    month_invoices,
                    discrepancies: rows,
                    rejects: &rejects,
                };
                let path = out(&format!("kettle_pack_{month}.xlsx"));
                pack::write(&p, &path)?;
                let total: Cents = p.spend.iter().map(|r| r.total).sum();
                println!(
                    "wrote {path}: {} suppliers, {} invoices, spend {}",
                    p.spend.len(),
                    p.month_invoices,
                    total
                );
                return Ok(());
            }
            let mut w = csv::Writer::from_path(out("discrepancies.csv"))?;
            w.write_record([
                "kind",
                "po_id",
                "invoice_number",
                "supplier",
                "expected",
                "billed",
                "difference",
            ])?;
            for d in &ds {
                w.write_record([
                    d.kind.to_string(),
                    d.po_id.to_string(),
                    d.invoice_number.clone().unwrap_or_default(),
                    name_of(d.supplier_id).to_string(),
                    d.expected.map(|c| c.to_string()).unwrap_or_default(),
                    d.billed.to_string(),
                    d.difference().map(|c| c.to_string()).unwrap_or_default(),
                ])?;
            }
            w.flush()?;
            for kind in reconcile::ALL_KINDS {
                let n = ds.iter().filter(|d| d.kind == kind).count();
                println!("{kind:<18} {n}");
            }
            let over: Cents = ds
                .iter()
                .filter(|d| d.kind == reconcile::Kind::Overbilled)
                .filter_map(|d| d.difference())
                .sum();
            println!("total overbilled: {over}");
            println!("rows: {}", ds.len());
            if with_branch {
                println!(
                    "branch accepted: {}  rejected: {}",
                    good.len(),
                    rejects.len()
                );
                let mut by_field: HashMap<String, usize> = HashMap::new();
                for r in &rejects {
                    *by_field
                        .entry(format!("{} / {}", r.field, r.reason))
                        .or_default() += 1;
                }
                let mut v: Vec<_> = by_field.into_iter().collect();
                v.sort();
                for (k, n) in v {
                    println!("  {k}: {n}");
                }
                let mut w = csv::Writer::from_path(out("rejects.csv"))?;
                w.write_record(["ref", "field", "value", "reason"])?;
                for r in &rejects {
                    w.write_record([&r.reference, &r.field.to_string(), &r.value, &r.reason])?;
                }
                w.flush()?;
            }
        }
    }
    Ok(())
}
