// region: closures
struct Item {
    sku: String,
    qty: u32,
}

fn main() {
    let items = vec![
        Item {
            sku: "BRG-6204".to_string(),
            qty: 40,
        },
        Item {
            sku: "BLT-M10".to_string(),
            qty: 500,
        },
        Item {
            sku: "HSE-12".to_string(),
            qty: 12,
        },
    ];

    let total: u32 = items.iter().map(|item| item.qty).sum();
    println!("{total} units");

    if let Some(item) = items.iter().max_by_key(|item| item.qty) {
        println!("most: {} ({})", item.sku, item.qty);
    }

    let mut few = Vec::new();
    for item in &items {
        if item.qty < 100 {
            few.push(&item.sku);
        }
    }
    println!("under 100: {few:?}");
}
// endregion
