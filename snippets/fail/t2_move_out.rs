use std::collections::HashMap;

struct Product {
    category: String,
}

fn per_category(products: &[Product]) -> HashMap<String, usize> {
    let mut counts: HashMap<String, usize> = HashMap::new();
    for p in products {
        *counts.entry(p.category).or_default() += 1;
    }
    counts
}

fn main() {
    let products = vec![Product {
        category: "chain".to_string(),
    }];
    println!("{:?}", per_category(&products));
}
