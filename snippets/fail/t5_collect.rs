use std::num::ParseIntError;

#[derive(Debug)]
enum SlipError {
    BadNumber(ParseIntError),
}

impl From<ParseIntError> for SlipError {
    fn from(e: ParseIntError) -> SlipError {
        SlipError::BadNumber(e)
    }
}

fn days(raw: &[&str]) -> Result<Vec<u32>, SlipError> {
    raw.iter().map(|s| s.parse::<u32>()).collect()
}

fn main() {
    println!("{:?}", days(&["14", "7"]));
}
