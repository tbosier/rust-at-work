use std::collections::HashMap;

fn main() {
    let mut members: HashMap<i64, &str> = HashMap::new();
    members.insert(2, "Grace");
    let name: &str = members.get(&2);
    println!("{name}");
}
