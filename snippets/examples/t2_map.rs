// region: basics
use std::collections::HashMap;
// endregion

fn main() {
    // region: basics
    let mut shelves: HashMap<&str, u32> = HashMap::new();
    shelves.insert("fiction", 412);
    shelves.insert("history", 158);
    shelves.insert("fiction", 415); // same key: replaces the old value

    println!("{} keys", shelves.len());
    println!("{:?}", shelves.get("history"));
    println!("{:?}", shelves.get("poetry"));
    // endregion
    println!("---");
    // region: option
    match shelves.get("poetry") {
        Some(n) => println!("poetry: {n}"),
        None => println!("no poetry shelf"),
    }
    if let Some(n) = shelves.get("fiction") {
        println!("fiction: {n}");
    }
    let poetry = shelves.get("poetry").copied().unwrap_or(0);
    println!("poetry, or 0: {poetry}");
    // endregion
    println!("---");
    // region: entry
    let words = ["out", "returned", "out", "lost", "out", "returned"];
    let mut counts: HashMap<&str, u32> = HashMap::new();
    for w in words {
        *counts.entry(w).or_insert(0) += 1;
    }
    let mut sorted: Vec<(&str, u32)> = counts.into_iter().collect();
    sorted.sort();
    println!("{sorted:?}");
    // endregion
    println!("---");
    // region: borrowed
    let stock: HashMap<&str, (u32, f64)> =
        HashMap::from([("roller chain", (3, 41.5)), ("carriage bolt", (12, 9.0))]);
    for (name, (count, value)) in &stock {
        // Borrowing the map hands out references: `count` is a &u32 here.
        if *count > 5 {
            println!("{name}: {count} in stock, worth {value:.2}");
        }
    }
    let mut pieces = 0;
    for &(count, _) in stock.values() {
        // `.values()` borrows just the values. The `&` in the pattern
        // takes the numbers out of the reference.
        pieces += count;
    }
    println!("{pieces} pieces in all");
    // endregion
}
