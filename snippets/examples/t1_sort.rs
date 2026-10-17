fn main() {
    // region: sort
    let mut words = vec!["pear", "fig", "kiwi", "apple"];

    words.sort_by_key(|w| w.len());
    println!("{words:?}");

    words.sort_by_key(|w| (w.len(), *w));
    println!("{words:?}");

    words.sort();
    println!("{words:?}");

    words.sort_by(|a, b| b.cmp(a));
    println!("{words:?}");
    // endregion
}
