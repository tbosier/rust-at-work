// region: vars
const BOX_SIZE: i64 = 5;

fn main() {
    let ordered: i64 = 12;
    let mut shipped = 0;
    shipped += BOX_SIZE;
    shipped += BOX_SIZE;
    println!("shipped {shipped} of {ordered}");

    let sku = "  BRG-6204 ";
    let sku = sku.trim();
    println!("[{sku}]");
}
// endregion
