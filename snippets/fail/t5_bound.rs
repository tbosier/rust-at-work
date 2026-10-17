trait TableRow {
    fn headers() -> &'static [&'static str];
    fn cells(&self) -> Vec<String>;
}

fn print_table<T: TableRow>(rows: &[T]) {
    println!("{}", T::headers().join(" | "));
    for row in rows {
        println!("{}", row.cells().join(" | "));
    }
}

struct Fine {
    loan_id: i64,
    cents: i64,
}

fn main() {
    let fines = vec![Fine {
        loan_id: 6,
        cents: 2495,
    }];
    print_table(&fines);
}
