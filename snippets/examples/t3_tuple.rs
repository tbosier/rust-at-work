// region: tuple
use std::cmp::Ordering;

#[derive(Debug, Clone, Copy)]
enum LoanStatus {
    Out,
    Returned,
    Lost,
}

const LOAN_DAYS: i64 = 14;

fn verdict(status: LoanStatus, days_out: i64) -> &'static str {
    match (status, days_out.cmp(&LOAN_DAYS)) {
        (LoanStatus::Lost, _) => "bill the replacement",
        (LoanStatus::Out, Ordering::Greater) => "overdue: send a reminder",
        (LoanStatus::Out, Ordering::Less | Ordering::Equal) => "out, not due yet",
        (LoanStatus::Returned, Ordering::Greater) => "returned late: fine it",
        (LoanStatus::Returned, Ordering::Less | Ordering::Equal) => "returned on time",
    }
}
// endregion

fn main() {
    // region: tuple_use
    let loans = [
        (LoanStatus::Out, 3),
        (LoanStatus::Out, 20),
        (LoanStatus::Returned, 14),
        (LoanStatus::Returned, 15),
        (LoanStatus::Lost, 2),
    ];
    for (status, days) in loans {
        println!(
            "{:<9} {days:>2} days: {}",
            format!("{status:?}"),
            verdict(status, days)
        );
    }
    // endregion
    println!("---");
    // region: fold
    let late_days = [3, 0, 12, 5];
    // 25 cents a day, at most $2 per loan, in cents.
    let fines = late_days
        .iter()
        .fold(0, |total, days| total + (days * 25).min(200));
    let longest = late_days.iter().fold(0, |most, &days| most.max(days));
    println!("fines: {fines} cents, longest: {longest} days");
    // endregion
}
