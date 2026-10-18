// region: money
mod money {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    pub struct Cents(i64);

    impl Cents {
        pub const ZERO: Cents = Cents(0);

        pub fn new(cents: i64) -> Cents {
            Cents(cents)
        }

        pub fn get(self) -> i64 {
            self.0
        }
    }
}

use money::Cents;
// endregion

fn main() {
    // region: money_use
    let a = Cents::ZERO;
    let b = Cents::default();
    let c = Cents::new(1250);
    println!("{a:?} {b:?} {}", c.get());
    // endregion
}
