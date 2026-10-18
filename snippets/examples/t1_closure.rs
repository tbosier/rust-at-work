fn main() {
    // region: closures
    // A closure can use variables from around it. This one uses `fine_per_day`.
    let fine_per_day = 25;
    let days_late = [0, 3, 12];
    let fines: Vec<i64> = days_late.iter().map(|days| days * fine_per_day).collect();
    println!("fines in cents: {fines:?}");

    // `|_|` takes an argument and ignores it.
    let parsed: Result<i64, &str> = "12x".parse::<i64>().map_err(|_| "not a number");
    println!("{parsed:?}");

    // A function's name is a value too: `String::from` is passed, not called.
    let returned_on: Option<&str> = Some("2026-08-20");
    let owned: Option<String> = returned_on.map(String::from);
    println!("{owned:?}");

    // `||` takes no arguments. This one only runs if there's no member.
    let member: Option<&str> = None;
    let shown = member
        .map(String::from)
        .unwrap_or_else(|| format!("nobody, fined {fine_per_day}c a day"));
    println!("{shown}");
    // endregion
}
