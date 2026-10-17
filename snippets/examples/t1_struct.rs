// region: struct
#[derive(Debug)]
struct Loan {
    loan_id: i64,
    member: String,
    loaned_on: String,
    returned_on: Option<String>,
    status: String,
}
// endregion

fn main() {
    // region: make
    let loan = Loan {
        loan_id: 2,
        member: String::from("Grace"),
        loaned_on: "2026-08-11".to_string(),
        returned_on: None,
        status: "out".to_string(),
    };
    println!("{} borrowed on {}", loan.member, loan.loaned_on);
    println!("{:?}", loan);
    println!("{loan:#?}");
    // endregion
    let _ = (loan.loan_id, &loan.returned_on, &loan.status);
}
