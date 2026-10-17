use std::collections::HashMap;

struct Book {
    title: String,
    shelf: String,
}

fn main() {
    let books = vec![
        Book {
            title: "SICP".into(),
            shelf: "cs".into(),
        },
        Book {
            title: "TAOCP".into(),
            shelf: "cs".into(),
        },
    ];
    let mut per_shelf: HashMap<&str, u32> = HashMap::new();
    for b in &books {
        *per_shelf.entry(b.shelf.as_str()).or_default() += 1;
    }
    drop(books);
    println!("{per_shelf:?}");
}
