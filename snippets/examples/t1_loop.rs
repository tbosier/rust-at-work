struct Loan {
    status: String,
}

// region: loop
fn count_out(loans: &[Loan]) -> usize {
    let mut n = 0;
    for loan in loans {
        if loan.status == "out" {
            n += 1;
        }
    }
    n
}
// endregion

fn main() {
    let loans = vec![
        Loan {
            status: "out".into(),
        },
        Loan {
            status: "returned".into(),
        },
    ];
    println!("{}", count_out(&loans));
}
