mod money {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    pub struct Cents(i64);
}

use money::Cents;

fn main() {
    let total = Cents(0);
    println!("{total:?}");
}
