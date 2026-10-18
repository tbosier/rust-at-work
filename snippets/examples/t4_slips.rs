// Renewal slips: a shelf mark and a loan length, both typed by hand.

// The duration parser from the wiki, so this file stands alone.
#[path = "t4_duration.rs"]
#[allow(dead_code)]
mod duration;
use duration::parse_duration;

// region: shelf
mod shelf {
    use std::fmt;

    #[derive(Debug, PartialEq, Eq)]
    pub enum ShelfError {
        Format(String),
        NoSuchAisle(char),
        NoSuchShelf { aisle: char, number: u32 },
    }

    impl fmt::Display for ShelfError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                ShelfError::Format(s) => write!(f, "{s:?} isn't written like B-12"),
                ShelfError::NoSuchAisle(a) => write!(f, "there's no aisle {a}"),
                ShelfError::NoSuchShelf { aisle, number } => {
                    write!(f, "aisle {aisle} has no shelf {number}")
                }
            }
        }
    }

    impl std::error::Error for ShelfError {}

    /// A shelf that exists in the building. The fields are private, so
    /// `Shelf::new` is the only way to make one.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Shelf {
        aisle: char,
        number: u32,
    }

    /// How many shelves an aisle has. D is the short one by the door.
    fn shelves_in(aisle: char) -> Option<u32> {
        match aisle {
            'A' | 'B' | 'C' => Some(40),
            'D' => Some(12),
            _ => None,
        }
    }

    impl Shelf {
        /// Checks the aisle, then the number against that aisle's size.
        pub fn new(aisle: char, number: u32) -> Result<Shelf, ShelfError> {
            let most = shelves_in(aisle).ok_or(ShelfError::NoSuchAisle(aisle))?;
            if number == 0 || number > most {
                return Err(ShelfError::NoSuchShelf { aisle, number });
            }
            Ok(Shelf { aisle, number })
        }

        /// Text like `B-12`: one letter, a dash, a number.
        pub fn parse(raw: &str) -> Result<Shelf, ShelfError> {
            let bad = || ShelfError::Format(raw.to_string());
            let (aisle, number) = raw.trim().split_once('-').ok_or_else(bad)?;
            let aisle: char = aisle.parse().map_err(|_| bad())?;
            let number: u32 = number.parse().map_err(|_| bad())?;
            Shelf::new(aisle, number)
        }

        pub fn aisle(&self) -> char {
            self.aisle
        }

        pub fn number(&self) -> u32 {
            self.number
        }
    }
}
use shelf::Shelf;
// endregion

// region: reject
/// A slip that passed every check.
struct Renewal {
    slip: String,
    shelf: Shelf,
    days: u32,
}

/// A slip that didn't, with what's needed to fix it at the source.
struct Reject {
    slip: String,
    field: &'static str,
    value: String,
    reason: String,
}

impl Reject {
    fn new(slip: &str, field: &'static str, value: &Option<String>, reason: String) -> Reject {
        Reject {
            slip: slip.to_string(),
            field,
            value: value.clone().unwrap_or_default(),
            reason,
        }
    }
}

/// One row as it came out of the database: any column might be NULL.
struct RawSlip {
    slip: Option<String>,
    shelf: Option<String>,
    asked_for: Option<String>,
}

/// Checks the fields in order and stops at the first problem.
fn check(raw: &RawSlip) -> Result<Renewal, Reject> {
    let slip = raw.slip.as_deref().unwrap_or("").trim();
    if slip.is_empty() {
        return Err(Reject::new("", "slip", &raw.slip, "missing".to_string()));
    }
    let shelf = Shelf::parse(raw.shelf.as_deref().unwrap_or(""))
        .map_err(|e| Reject::new(slip, "shelf", &raw.shelf, e.to_string()))?;
    let days = parse_duration(raw.asked_for.as_deref().unwrap_or(""))
        .map_err(|e| Reject::new(slip, "asked_for", &raw.asked_for, e.to_string()))?;
    Ok(Renewal {
        slip: slip.to_string(),
        shelf,
        days,
    })
}

/// Builds a test row. `String::from` is a function, passed as a value.
fn slip_row(slip: Option<&str>, shelf: Option<&str>, asked_for: Option<&str>) -> RawSlip {
    RawSlip {
        slip: slip.map(String::from),
        shelf: shelf.map(String::from),
        asked_for: asked_for.map(String::from),
    }
}
// endregion

fn main() {
    // region: shelf_use
    for text in ["B-12", " D-12 ", "D-13", "E-1", "B12", "B-0"] {
        match Shelf::parse(text) {
            Ok(s) => println!("{text:?}: aisle {}, shelf {}", s.aisle(), s.number()),
            Err(e) => println!("{text:?}: {e}"),
        }
    }
    // endregion
    println!("---");
    // region: reject_use
    // In the ticket these rows come from one SELECT.
    let slips = vec![
        slip_row(Some("R-01"), Some("A-3"), Some("14d")),
        slip_row(Some("R-02"), Some("D-30"), Some("2w")),
        slip_row(Some("R-03"), Some("C-7"), Some("a fortnight")),
        slip_row(None, Some("B-1"), Some("7 days")),
        slip_row(Some("R-05"), Some("B-40"), Some(" 3 weeks ")),
        slip_row(Some("R-06"), None, Some("1w")),
    ];
    let mut accepted = Vec::new();
    let mut rejected = Vec::new();
    for row in &slips {
        match check(row) {
            Ok(renewal) => accepted.push(renewal),
            Err(reject) => rejected.push(reject),
        }
    }
    println!("{} accepted, {} rejected", accepted.len(), rejected.len());
    for r in &accepted {
        println!(
            "  {}: shelf {}-{}, {} days",
            r.slip,
            r.shelf.aisle(),
            r.shelf.number(),
            r.days
        );
    }
    for r in &rejected {
        println!(
            "  {:<5} {:<9} {:<14} {}",
            r.slip,
            r.field,
            format!("{:?}", r.value),
            r.reason
        );
    }
    // endregion
}
