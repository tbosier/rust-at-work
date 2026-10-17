mod card {
    #[derive(Debug)]
    pub struct CardNumber(u32);

    impl CardNumber {
        pub fn parse(raw: &str) -> Option<CardNumber> {
            raw.strip_prefix("LIB-")?.parse().ok().map(CardNumber)
        }
    }
}

fn main() {
    let forged = card::CardNumber(9999);
    println!("{forged:?} {:?}", card::CardNumber::parse("LIB-0042"));
}
