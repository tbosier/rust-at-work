fn main() {
    // region: types
    let quantity: u32 = 40;
    let price_cents: i64 = 1_299;
    let weight_kg: f64 = 2.5;
    let in_stock: bool = true;
    let grade: char = 'A';
    let line_count: usize = 3;
    println!("{quantity} {price_cents} {weight_kg} {in_stock} {grade} {line_count}");
    // endregion
    println!("---");
    // region: division
    println!("{}", 7 / 2);
    println!("{}", -7 / 2);
    println!("{}", 7 % 2);
    println!("{}", -7 % 2);
    println!("{}", 7.0 / 2.0);
    // endregion
    println!("---");
    // region: cast
    let total_cents: i64 = 4_599;
    let lines: usize = 4;
    let average = total_cents as f64 / lines as f64;
    println!("{average:.2}");
    println!("{}", 3.99_f64 as i64);
    println!("{}", 300_i64 as u8);
    println!("{}", -1_i64 as u32);
    // endregion
}
