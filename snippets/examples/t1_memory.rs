#[allow(dead_code)]
struct Loan {
    loan_id: i64,
    member: String,
    loaned_on: String,
    returned_on: Option<String>,
    status: String,
}

fn main() {
    // region: sizes
    use std::mem::size_of;
    println!("i64:            {} bytes", size_of::<i64>());
    println!("String:         {} bytes", size_of::<String>());
    println!("Option<String>: {} bytes", size_of::<Option<String>>());
    println!("Loan:           {} bytes", size_of::<Loan>());
    println!("Vec<Loan>:      {} bytes", size_of::<Vec<Loan>>());
    // endregion
    println!("---");
    // region: growth
    let mut ids: Vec<i64> = Vec::new();
    let mut last_cap = ids.capacity();
    println!("len {:>4}  capacity {:>4}", ids.len(), last_cap);
    for id in 0..2017 {
        ids.push(id);
        if ids.capacity() != last_cap {
            last_cap = ids.capacity();
            println!("len {:>4}  capacity {:>4}", ids.len(), last_cap);
        }
    }
    // endregion
}
