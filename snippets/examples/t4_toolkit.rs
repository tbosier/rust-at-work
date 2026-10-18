fn main() {
    // region: tools
    let raw = "  Card: LIB-0042  ";
    let s = raw.trim();
    println!("{s:?}");
    println!("{:?}", s.strip_prefix("Card: "));
    println!("{:?}", s.strip_prefix("card: "));
    println!("{:?}", "LIB-0042".split_once('-'));
    println!("{:?}", "LIB0042".split_once('-'));
    println!("{}", "0042".bytes().all(|b| b.is_ascii_digit()));
    println!("{:?}", "0042".parse::<u32>());
    // endregion
    println!("---");
    // region: traps
    println!("{:?}", "+42".parse::<u32>());
    println!("{:?}", " 42".parse::<u32>());
    println!("{:?}", "".parse::<u32>());
    println!("{:?}", "99999999999".parse::<u32>());
    println!("{:?}", "1_000".parse::<u32>());
    println!("{}", '٤'.is_numeric());
    println!("{}", '٤'.is_ascii_digit());
    println!("{}", "".bytes().all(|b| b.is_ascii_digit()));
    // endregion
    println!("---");
    // region: floats
    let fine: f64 = "0.29".parse().unwrap();
    println!("{}", fine * 100.0);
    println!("{}", (fine * 100.0) as i64);
    println!("{}", (fine * 100.0).round() as i64);
    // endregion
    println!("---");
    // region: words
    let typed = "  the  PRAGMATIC   programmer ";
    let lower = typed.to_lowercase();
    println!("{lower:?}");
    let words: Vec<&str> = lower.split_whitespace().collect();
    println!("{words:?}");
    println!("{:?}", words.join(" "));
    println!("{:?}", "Kernighan, Ritchie".replace(", ", " and "));
    println!("{}", "LIB-0042".starts_with("LIB-"));
    // endregion
    println!("---");
    // region: deref
    let shelf: Option<String> = Some(" A12 ".to_string());
    let missing: Option<String> = None;
    let s: Option<&str> = shelf.as_deref();
    println!("{:?}", s.map(|t| t.trim()));
    println!("{:?}", missing.as_deref().unwrap_or(""));
    // endregion
}
