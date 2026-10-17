// region: cents
/// Cents in a fine written exactly like `$1.25`. Deliberately strict:
/// two decimals, no commas, no sign.
fn fine_cents(raw: &str) -> Option<i64> {
    let s = raw.trim().strip_prefix('$')?;
    let (dollars, cents) = s.split_once('.')?;
    let digits = |t: &str| !t.is_empty() && t.bytes().all(|b| b.is_ascii_digit());
    if !digits(dollars) || cents.len() != 2 || !digits(cents) {
        return None;
    }
    let dollars: i64 = dollars.parse().ok()?;
    let cents: i64 = cents.parse().ok()?;
    dollars.checked_mul(100)?.checked_add(cents)
}
// endregion

fn main() {
    // region: run
    for raw in ["$0.29", " $1.25 ", "$12.5", "1.25", "$1.2.5", "$.25"] {
        println!("{raw:?} -> {:?}", fine_cents(raw));
    }
    // endregion
}
