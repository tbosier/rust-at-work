fn main() {
    // region: option
    let skus = ["BRG-6204", "BLT-M10"];
    match skus.get(5) {
        Some(sku) => println!("found {sku}"),
        None => println!("nothing at position 5"),
    }
    if let Some(sku) = skus.first() {
        println!("the first is {sku}");
    }
    // endregion
    println!("---");
    // region: result
    for raw in ["40", "forty", " 40"] {
        match raw.parse::<u32>() {
            Ok(qty) => println!("{raw:?} is {qty}"),
            Err(e) => println!("{raw:?} is no good: {e}"),
        }
    }
    let per_case: u32 = "12".parse().expect("I typed this number myself");
    println!("{per_case} per case");
    // endregion
}
