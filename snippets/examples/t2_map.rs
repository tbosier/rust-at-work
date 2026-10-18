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
}
