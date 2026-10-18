trait TextRow {
    fn cells(&self) -> Vec<String>;
}

struct Product {
    sku: String,
}

impl TextRow for Product {
    fn cells(&self) -> Vec<String> {
        vec![self.sku.clone()]
    }
}

fn print_table<T: TextRow>(rows: &[T]) {
    for r in rows {
        println!("{}", r.cells().join(" | "));
    }
}

fn main() {
    let products = vec![Product {
        sku: "BRG-6204".to_string(),
    }];
    let bearings: Vec<&Product> = products
        .iter()
        .filter(|p| p.sku.starts_with("BRG"))
        .collect();
    print_table(&bearings);
}
