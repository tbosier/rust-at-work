#[allow(clippy::useless_vec)]
fn main() {
    // region: format
    let sku = "BRG-6204";
    let qty = 40;
    let price = 12.5;
    println!("{sku} x {qty}");
    println!("{} boxes of {}", qty / 8, 8);
    println!("[{sku:<10}] [{qty:>5}] [{qty:^5}] [{qty:05}]");
    println!("[{price:.2}] [{price:>8.2}] [{price:*>8.2}]");
    // endregion
    println!("---");
    // region: debug
    let line = (sku, qty);
    let skus = vec!["BRG-6204", "BLT-M10"];
    println!("{line:?}");
    println!("{skus:?}");
    println!("{skus:#?}");
    let label = format!("{qty} units of {sku}");
    println!("{label}");
    // endregion
}
