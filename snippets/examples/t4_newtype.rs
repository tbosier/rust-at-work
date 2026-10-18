// region: newtype
mod card {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct CardNumber(u32);

    impl CardNumber {
        /// The only way to make a CardNumber: from text that really is one.
        pub fn parse(raw: &str) -> Option<CardNumber> {
            let digits = raw.trim().strip_prefix("LIB-")?;
            if digits.len() != 4 || !digits.bytes().all(|b| b.is_ascii_digit()) {
                return None;
            }
            digits.parse().ok().map(CardNumber)
        }

        pub fn get(self) -> u32 {
            self.0
        }
    }
}
// endregion

fn main() {
    // region: use
    use card::CardNumber;
    println!("{:?}", CardNumber::parse(" LIB-0042 "));
    println!("{:?}", CardNumber::parse("LIB-42"));
    println!("{:?}", CardNumber::parse("LIB-0042").map(CardNumber::get));
    // endregion
}
