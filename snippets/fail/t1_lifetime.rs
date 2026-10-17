struct Loan {
    status: String,
    loaned_on: String,
}

fn out_before(loans: &[Loan], cutoff: &str) -> Vec<&Loan> {
    loans
        .iter()
        .filter(|loan| loan.status == "out" && loan.loaned_on.as_str() < cutoff)
        .collect()
}

fn main() {
    let loans = vec![Loan {
        status: "out".into(),
        loaned_on: "2026-08-11".into(),
    }];
    println!("{}", out_before(&loans, "2026-09-01").len());
}
