struct Loan {
    loaned_on: String,
}

/// Loans that went out before this date count as early.
const CUTOFF: &str = "2026-09-15";

fn count_early(loans: &[Loan]) -> usize {
    loans.iter().filter(|loan| loan.loaned_on < CUTOFF).count()
}

fn main() {
    let loans = vec![Loan {
        loaned_on: "2026-08-03".to_string(),
    }];
    println!("{}", count_early(&loans));
}
