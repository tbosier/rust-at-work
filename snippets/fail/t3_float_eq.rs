use std::collections::HashMap;

fn main() {
    let mut by_price: HashMap<f64, &str> = HashMap::new();
    by_price.insert(0.30, "loan 4");
    println!("{:?}", by_price.get(&0.30));
}
