// region: collect
use std::fmt;
use std::num::ParseIntError;

#[derive(Debug)]
enum SlipError {
    BadNumber(ParseIntError),
}

impl fmt::Display for SlipError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SlipError::BadNumber(e) => write!(f, "not a number of days: {e}"),
        }
    }
}

impl From<ParseIntError> for SlipError {
    fn from(e: ParseIntError) -> SlipError {
        SlipError::BadNumber(e)
    }
}

fn days(raw: &[&str]) -> Result<Vec<u32>, SlipError> {
    let days = raw
        .iter()
        .map(|s| s.parse::<u32>())
        .collect::<Result<Vec<_>, _>>()?;
    Ok(days)
}
// endregion

fn main() {
    // region: collect_use
    for slips in [["14", "7"], ["14", "two"]] {
        match days(&slips) {
            Ok(d) => println!("{d:?}"),
            Err(e) => println!("error: {e}"),
        }
    }
    // endregion
}
