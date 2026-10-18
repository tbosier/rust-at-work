#[allow(clippy::useless_vec)]
fn main() {
    // region: tuple
    let line = ("BRG-6204", 40, 1_299);
    let (sku, qty, price) = line;
    println!("{sku}: {qty} at {price}");
    println!("by position: {} {}", line.0, line.1);

    let week: [u32; 5] = [12, 0, 7, 45, 3];
    println!("{} days, first {}, last {}", week.len(), week[0], week[4]);

    let mut skus = vec!["BRG-6204", "BLT-M10"];
    skus.push("HSE-12");
    println!("{skus:?}: {} items, empty: {}", skus.len(), skus.is_empty());
    // endregion
    println!("---");
    // region: get
    println!("{:?}", skus.get(1));
    println!("{:?}", skus.get(10));
    println!("{:?} {:?}", skus.first(), skus.last());
    println!("{}", skus[skus.len() - 1]);
    // endregion
    println!("---");
    // region: slices
    let qty = vec![12, 0, 7, 45, 3, 18];
    println!("{:?}", &qty[1..3]);
    println!("{:?}", &qty[..2]);
    println!("{:?}", &qty[4..]);
    // endregion
    println!("---");
    // region: walk
    for (i, q) in qty.iter().enumerate().take(3) {
        println!("line {i}: {q}");
    }
    for q in qty.iter().rev().take(2) {
        println!("from the end: {q}");
    }
    let total: i64 = qty.iter().sum();
    println!("total {total}, biggest {:?}, smallest {:?}", qty.iter().max(), qty.iter().min());
    // endregion
}
