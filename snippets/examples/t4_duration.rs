// A loan length typed on a renewal slip: "14d", "2w", " 3 weeks ".
// Shown in the guide in regions; also compiled as module `duration`
// of snippets/t5_crate, which runs the tests below.

// region: error
use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub enum DurationError {
    Empty,
    NoNumber(String),
    TooBig(String),
    UnknownUnit(String),
}

impl fmt::Display for DurationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DurationError::Empty => write!(f, "nothing written"),
            DurationError::NoNumber(s) => write!(f, "{s:?} doesn't start with a number"),
            DurationError::TooBig(s) => write!(f, "{s:?} is too long a loan"),
            DurationError::UnknownUnit(u) => write!(f, "unknown unit {u:?}"),
        }
    }
}

impl std::error::Error for DurationError {}
// endregion

// region: parse
/// Days in a loan length like `14d`, `2w`, `3 weeks` or ` 10 days `.
pub fn parse_duration(raw: &str) -> Result<u32, DurationError> {
    let s = raw.trim();
    if s.is_empty() {
        return Err(DurationError::Empty);
    }
    let digits = s.bytes().take_while(|b| b.is_ascii_digit()).count();
    let (number, unit) = s.split_at(digits);
    if number.is_empty() {
        return Err(DurationError::NoNumber(s.to_string()));
    }
    let n: u32 = number
        .parse()
        .map_err(|_| DurationError::TooBig(s.to_string()))?;
    let days_per = match unit.trim() {
        "d" | "day" | "days" => 1,
        "w" | "week" | "weeks" => 7,
        other => return Err(DurationError::UnknownUnit(other.to_string())),
    };
    n.checked_mul(days_per)
        .ok_or_else(|| DurationError::TooBig(s.to_string()))
}
// endregion

// region: tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_every_way_the_slips_say_it() {
        assert_eq!(parse_duration("14d"), Ok(14));
        assert_eq!(parse_duration("2w"), Ok(14));
        assert_eq!(parse_duration(" 3 weeks "), Ok(21));
        assert_eq!(parse_duration("1 day"), Ok(1));
    }

    #[test]
    fn says_what_is_wrong() {
        assert_eq!(parse_duration("   "), Err(DurationError::Empty));
        assert_eq!(
            parse_duration("fortnight"),
            Err(DurationError::NoNumber("fortnight".into()))
        );
        assert_eq!(
            parse_duration("3 months"),
            Err(DurationError::UnknownUnit("months".into()))
        );
    }

    #[test]
    fn edges() {
        assert_eq!(parse_duration("0d"), Ok(0));
        assert_eq!(parse_duration("4294967295d"), Ok(u32::MAX));
        assert!(matches!(
            parse_duration("4294967296d"),
            Err(DurationError::TooBig(_))
        ));
        assert!(matches!(
            parse_duration("613566757w"),
            Err(DurationError::TooBig(_))
        ));
        assert!(matches!(
            parse_duration("-3d"),
            Err(DurationError::NoNumber(_))
        ));
    }
}
// endregion

#[allow(dead_code)]
fn main() {
    // region: demo
    for raw in ["14d", " 3 weeks ", "fortnight", "3 months", ""] {
        match parse_duration(raw) {
            Ok(days) => println!("{raw:?} -> {days} days"),
            Err(e) => println!("{raw:?} -> error: {e}"),
        }
    }
    // endregion
}
