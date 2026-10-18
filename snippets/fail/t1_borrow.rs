fn main() {
    let mut loan_ids = vec![1, 2, 3];
    let first = &loan_ids[0];
    loan_ids.push(4);
    println!("first loan: {first}");
}
