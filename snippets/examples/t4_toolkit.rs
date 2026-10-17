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
}
