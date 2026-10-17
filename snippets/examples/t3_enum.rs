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
}
