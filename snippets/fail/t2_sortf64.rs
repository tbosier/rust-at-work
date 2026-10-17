fn main() {
    let mut totals = vec![("Ada", 25.25), ("Grace", 1.1), ("Alan", 0.0)];
    totals.sort_by_key(|t| t.1);
    println!("{totals:?}");
}
