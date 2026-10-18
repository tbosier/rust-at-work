// region: enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Standing {
    Lost,
    Overdue { days: i64 },
    Out,
    Returned,
}
// endregion

// region: match
fn action(standing: Standing) -> String {
    match standing {
        Standing::Lost => "bill the replacement".to_string(),
        Standing::Overdue { days } if days > 30 => format!("{days} days late: send a letter"),
        Standing::Overdue { days } => format!("{days} days late: send a reminder"),
        Standing::Out => "nothing yet".to_string(),
        Standing::Returned => "nothing".to_string(),
    }
}
// endregion

// region: from_text
use std::error::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum LoanStatus {
    Out,
    Returned,
    Lost,
}

fn loan_status(text: &str) -> Option<LoanStatus> {
    match text {
        "out" => Some(LoanStatus::Out),
        "returned" => Some(LoanStatus::Returned),
        "lost" => Some(LoanStatus::Lost),
        _ => None,
    }
}

fn require_status(text: &str) -> Result<LoanStatus, Box<dyn Error>> {
    let status = loan_status(text).ok_or_else(|| format!("unknown loan status {text:?}"))?;
    Ok(status)
}
// endregion

fn main() {
    // region: use
    let mut all = vec![
        Standing::Returned,
        Standing::Overdue { days: 3 },
        Standing::Lost,
        Standing::Out,
        Standing::Overdue { days: 40 },
    ];
    all.sort();
    for s in &all {
        println!("{:<26} {}", format!("{s:?}"), action(*s));
    }
    let late = all
        .iter()
        .filter(|s| matches!(s, Standing::Overdue { .. }))
        .count();
    println!("{late} overdue");
    // endregion
    println!("---");
    // region: from_text_use
    println!("{:?}", loan_status("out"));
    println!("{:?}", loan_status("Lost"));
    match require_status("Lost") {
        Ok(status) => println!("{status:?}"),
        Err(e) => println!("error: {e}"),
    }
    // endregion
}
