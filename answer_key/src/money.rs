use std::fmt;
use std::iter::Sum;
use std::ops::{Add, AddAssign, Neg, Sub};

/// An amount of money in whole cents. Never a float.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Cents(pub i64);

impl Cents {
    /// Convert a dollar amount read from a REAL column. The database stores
    /// dollars as floats, so round to the nearest cent exactly once, here.
    pub fn from_dollars(dollars: f64) -> Cents {
        Cents((dollars * 100.0).round() as i64)
    }

    /// For spreadsheet cells, which hold numbers as f64 anyway.
    pub fn as_dollars(self) -> f64 {
        self.0 as f64 / 100.0
    }
}

impl Add for Cents {
    type Output = Cents;
    fn add(self, other: Cents) -> Cents {
        Cents(self.0 + other.0)
    }
}

impl Sub for Cents {
    type Output = Cents;
    fn sub(self, other: Cents) -> Cents {
        Cents(self.0 - other.0)
    }
}

impl Neg for Cents {
    type Output = Cents;
    fn neg(self) -> Cents {
        Cents(-self.0)
    }
}

impl AddAssign for Cents {
    fn add_assign(&mut self, other: Cents) {
        self.0 += other.0;
    }
}

impl Sum for Cents {
    fn sum<I: Iterator<Item = Cents>>(iter: I) -> Cents {
        iter.fold(Cents(0), |a, b| a + b)
    }
}

/// Prints like `1234.50` or `-45.00`: what finance's import expects.
impl fmt::Display for Cents {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let sign = if self.0 < 0 { "-" } else { "" };
        let abs = self.0.unsigned_abs();
        write!(f, "{sign}{}.{:02}", abs / 100, abs % 100)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum AmountError {
    Missing,
    NotANumber,
    TooManyDecimals,
}

impl fmt::Display for AmountError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AmountError::Missing => write!(f, "missing"),
            AmountError::NotANumber => write!(f, "not a number"),
            AmountError::TooManyDecimals => write!(f, "more than two decimal places"),
        }
    }
}

/// Parse text like `$1,204.50`, `1204.5`, `(45.00)` or `-45.00` into cents,
/// without ever going through a float.
pub fn parse_amount(raw: &str) -> Result<Cents, AmountError> {
    let mut s = raw.trim();
    if s.is_empty() {
        return Err(AmountError::Missing);
    }
    let mut negative = false;
    if let Some(inner) = s.strip_prefix('(').and_then(|t| t.strip_suffix(')')) {
        negative = true;
        s = inner.trim();
    } else if let Some(rest) = s.strip_prefix('-') {
        negative = true;
        s = rest;
    }
    let s = s.strip_prefix('$').unwrap_or(s).replace(',', "");
    let (whole, frac) = match s.split_once('.') {
        Some((w, f)) => (w, f),
        None => (s.as_str(), ""),
    };
    let all_digits = |t: &str| t.bytes().all(|b| b.is_ascii_digit());
    if whole.is_empty() || !all_digits(whole) || !all_digits(frac) {
        return Err(AmountError::NotANumber);
    }
    if frac.len() > 2 {
        return Err(AmountError::TooManyDecimals);
    }
    let dollars: i64 = whole.parse().map_err(|_| AmountError::NotANumber)?;
    let cents: i64 = match frac.len() {
        0 => 0,
        1 => frac.parse::<i64>().unwrap() * 10,
        _ => frac.parse().unwrap(),
    };
    let total = dollars
        .checked_mul(100)
        .and_then(|d| d.checked_add(cents))
        .ok_or(AmountError::NotANumber)?;
    Ok(Cents(if negative { -total } else { total }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn displays_two_decimals_and_sign() {
        assert_eq!(Cents(120450).to_string(), "1204.50");
        assert_eq!(Cents(-4500).to_string(), "-45.00");
        assert_eq!(Cents(-5).to_string(), "-0.05");
        assert_eq!(Cents(0).to_string(), "0.00");
    }

    #[test]
    fn parses_the_formats_millbrook_uses() {
        assert_eq!(parse_amount("1204.50"), Ok(Cents(120450)));
        assert_eq!(parse_amount("$1,204.50"), Ok(Cents(120450)));
        assert_eq!(parse_amount(" 1,204.50 "), Ok(Cents(120450)));
        assert_eq!(parse_amount("1204.5"), Ok(Cents(120450)));
        assert_eq!(parse_amount("1204"), Ok(Cents(120400)));
        assert_eq!(parse_amount("(45.00)"), Ok(Cents(-4500)));
        assert_eq!(parse_amount("-45.00"), Ok(Cents(-4500)));
    }

    #[test]
    fn rejects_what_it_cannot_read() {
        assert_eq!(parse_amount(""), Err(AmountError::Missing));
        assert_eq!(parse_amount("TBD"), Err(AmountError::NotANumber));
        assert_eq!(parse_amount("12.345"), Err(AmountError::TooManyDecimals));
        assert_eq!(parse_amount("."), Err(AmountError::NotANumber));
        assert_eq!(parse_amount("1.2.3"), Err(AmountError::NotANumber));
        assert_eq!(
            parse_amount("99999999999999999999"),
            Err(AmountError::NotANumber)
        );
        assert_eq!(
            parse_amount("92233720368547759"),
            Err(AmountError::NotANumber)
        );
    }
}
