trait TableRow {
    fn headers() -> &'static [&'static str];
    fn cells(&self) -> Vec<String>;
}

fn print_rows(_rows: &[Box<dyn TableRow>]) {}

fn main() {}
