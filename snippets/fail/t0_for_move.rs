fn main() {
    let skus = vec![String::from("BRG-6204"), String::from("BLT-M10")];
    for sku in skus {
        println!("{sku}");
    }
    println!("{} skus", skus.len());
}
