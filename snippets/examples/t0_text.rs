fn main() {
    // region: text
    let literal = "Carriage bolt";
    let owned: String = literal.to_string();
    let size = String::from("M10");
    let label = format!("{owned} {size}");
    println!("{label}, {} bytes", label.len());
    // endregion
    println!("---");
    // region: utf8
    for word in ["M10", "Ø25mm", "5µm", "café"] {
        println!("{word:<6} {} bytes, {} chars", word.len(), word.chars().count());
    }
    println!("{:?}", "café".as_bytes());
    println!("{:?}", "café".chars().next());
    println!("{}", &"BRG-6204"[..3]);
    // endregion
}
