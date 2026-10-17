fn total_len(names: Vec<String>) -> usize {
    names.iter().map(|n| n.len()).sum()
}

fn main() {
    let names = vec![String::from("Ada"), String::from("Grace")];
    let n = total_len(names);
    println!("{n} letters across {} names", names.len());
}
