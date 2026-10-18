// Hidden: produces the expected output for week zero's drills and ticket.

struct OrderLine {
    sku: String,
    quantity: i64,
    unit_cents: i64,
}

impl OrderLine {
    fn new(sku: &str, quantity: i64, unit_cents: i64) -> Self {
        OrderLine {
            sku: sku.to_string(),
            quantity,
            unit_cents,
        }
    }

    fn line_cents(&self) -> i64 {
        self.quantity * self.unit_cents
    }
}

fn main() {
    // Drill 1
    let qty = [12, 0, 7, 45, 3, 18, 0, 22];
    let mut ordering_days = 0;
    let mut total = 0;
    let mut biggest = 0;
    for q in qty {
        if q == 0 {
            continue;
        }
        ordering_days += 1;
        total += q;
        if q > biggest {
            biggest = q;
        }
    }
    let average = total as f64 / ordering_days as f64;
    println!("{} days, {ordering_days} with orders", qty.len());
    println!("{total} units, biggest day {biggest}");
    println!("average {average:.2} units per ordering day");
    println!("---");
    // Drill 2
    let skus = ["BRG-6204", "BLT-M10", "HSE-12", "BRG-6305", "GKT-8"];
    for (i, sku) in skus.iter().enumerate() {
        if sku.starts_with("BRG") {
            println!("{i}: {sku}");
        }
    }
    println!("last two: {:?}", &skus[skus.len() - 2..]);
    println!("first has {} bytes", skus[0].len());
    println!("---");
    // Drill 3
    let prices = [
        ("BRG-6204", 1_299),
        ("BLT-M10", 18),
        ("GKT-8", 205),
        ("CHN-40", 12_995),
    ];
    for (sku, cents) in prices {
        println!("{sku:<10}{:>5}.{:02}", cents / 100, cents % 100);
    }
    println!("---");
    // The ticket
    let lines = [
        OrderLine::new("BRG-6204", 40, 1_299),
        OrderLine::new("BLT-M10", 500, 18),
        OrderLine::new("HSE-12", 12, 4_550),
        OrderLine::new("GKT-8", 75, 210),
        OrderLine::new("CHN-40", 6, 12_995),
    ];
    println!("{:<10} {:>5} {:>9} {:>10}", "SKU", "QTY", "UNIT", "LINE");
    for line in &lines {
        let c = line.line_cents();
        println!(
            "{:<10} {:>5} {:>6}.{:02} {:>7}.{:02}",
            line.sku,
            line.quantity,
            line.unit_cents / 100,
            line.unit_cents % 100,
            c / 100,
            c % 100
        );
    }
    let mut total = 0;
    let mut best = &lines[0];
    for line in &lines {
        total += line.line_cents();
        if line.line_cents() > best.line_cents() {
            best = line;
        }
    }
    let average = total as f64 / lines.len() as f64;
    let mut above = 0;
    for line in &lines {
        if line.line_cents() as f64 > average {
            above += 1;
        }
    }
    println!();
    println!(
        "total:   {}.{:02} over {} lines",
        total / 100,
        total % 100,
        lines.len()
    );
    println!("average: {:.2}", average / 100.0);
    println!(
        "biggest: {} at {}.{:02}",
        best.sku,
        best.line_cents() / 100,
        best.line_cents() % 100
    );
    println!("above average: {above}");
}
