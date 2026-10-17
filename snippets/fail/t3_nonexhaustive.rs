enum Standing {
    Lost,
    Overdue { days: i64 },
    Out,
    Returned,
}

fn action(standing: Standing) -> String {
    match standing {
        Standing::Lost => "bill the replacement".to_string(),
        Standing::Overdue { days } => format!("{days} days late"),
        Standing::Returned => "nothing".to_string(),
    }
}

fn main() {
    println!("{}", action(Standing::Out));
}
