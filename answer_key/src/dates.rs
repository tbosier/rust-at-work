use std::fmt;

/// A calendar date that is known to exist.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Date {
    year: u16,
    month: u8,
    day: u8,
}

#[derive(Debug, PartialEq, Eq)]
pub enum DateError {
    Missing,
    UnknownFormat,
    NoSuchDay,
}

impl fmt::Display for DateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DateError::Missing => write!(f, "missing"),
            DateError::UnknownFormat => write!(f, "not YYYY-MM-DD or MM/DD/YYYY"),
            DateError::NoSuchDay => write!(f, "no such date"),
        }
    }
}

fn is_leap(year: u16) -> bool {
    (year % 4 == 0 && year % 100 != 0) || year % 400 == 0
}

fn days_in_month(year: u16, month: u8) -> u8 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 if is_leap(year) => 29,
        2 => 28,
        _ => 0,
    }
}

impl Date {
    pub fn new(year: u16, month: u8, day: u8) -> Result<Date, DateError> {
        if day == 0 || day > days_in_month(year, month) {
            return Err(DateError::NoSuchDay);
        }
        Ok(Date { year, month, day })
    }

    /// `YYYY-MM-DD` (ISO) or `M/D/YYYY` (US, one or two digits for M and D).
    pub fn parse(raw: &str) -> Result<Date, DateError> {
        let s = raw.trim();
        if s.is_empty() {
            return Err(DateError::Missing);
        }
        let parts: Vec<&str> = if s.contains('-') {
            s.split('-').collect()
        } else {
            s.split('/').collect()
        };
        let digits = |t: &str, lo: usize, hi: usize| {
            (lo..=hi).contains(&t.len()) && t.bytes().all(|b| b.is_ascii_digit())
        };
        let (y, m, d) = match parts.as_slice() {
            [y, m, d]
                if s.contains('-') && digits(y, 4, 4) && digits(m, 2, 2) && digits(d, 2, 2) =>
            {
                (y, m, d)
            }
            [m, d, y]
                if s.contains('/') && digits(m, 1, 2) && digits(d, 1, 2) && digits(y, 4, 4) =>
            {
                (y, m, d)
            }
            _ => return Err(DateError::UnknownFormat),
        };
        // Every piece is 1 to 4 ASCII digits, so these parses cannot fail.
        Date::new(y.parse().unwrap(), m.parse().unwrap(), d.parse().unwrap())
    }

    /// `2026-09` for this date's month.
    pub fn month_key(&self) -> String {
        format!("{:04}-{:02}", self.year, self.month)
    }
}

impl fmt::Display for Date {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:04}-{:02}-{:02}", self.year, self.month, self.day)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_both_formats() {
        assert_eq!(Date::parse("2026-08-14").unwrap().to_string(), "2026-08-14");
        assert_eq!(Date::parse("08/14/2026").unwrap().to_string(), "2026-08-14");
        assert_eq!(Date::parse("8/4/2026").unwrap().to_string(), "2026-08-04");
    }

    #[test]
    fn knows_the_calendar() {
        assert_eq!(Date::parse("02/29/2025"), Err(DateError::NoSuchDay));
        assert!(Date::parse("02/29/2024").is_ok());
        assert_eq!(Date::parse("2100-02-29"), Err(DateError::NoSuchDay));
        assert!(Date::parse("2000-02-29").is_ok());
        assert_eq!(Date::parse("16/07/2025"), Err(DateError::NoSuchDay));
        assert_eq!(Date::parse("2026-04-31"), Err(DateError::NoSuchDay));
    }

    #[test]
    fn refuses_other_shapes() {
        assert_eq!(Date::parse(""), Err(DateError::Missing));
        assert_eq!(Date::parse("Aug 14 2026"), Err(DateError::UnknownFormat));
        assert_eq!(Date::parse("2026/08/14"), Err(DateError::UnknownFormat));
        assert_eq!(Date::parse("2026-8-14"), Err(DateError::UnknownFormat));
        assert_eq!(Date::parse("+8/14/2026"), Err(DateError::UnknownFormat));
    }
}
