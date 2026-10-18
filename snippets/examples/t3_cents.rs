use snippets::library;
use std::collections::HashMap;
use std::error::Error;
// region: display
use std::fmt;
// endregion
// region: add
use std::ops::Add;
// endregion

// region: newtype
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
struct Cents(i64);

impl Cents {
    /// The one place a float becomes money: round to the nearest cent, once.
    fn from_dollars(dollars: f64) -> Cents {
        Cents((dollars * 100.0).round() as i64)
    }
}
// endregion

// region: display
impl fmt::Display for Cents {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}.{:02}", self.0 / 100, self.0 % 100)
    }
}
// endregion

// region: add
impl Add for Cents {
    type Output = Cents;

    fn add(self, other: Cents) -> Cents {
        Cents(self.0 + other.0)
    }
}
// endregion

fn main() -> Result<(), Box<dyn Error>> {
    let db = library()?;
    let mut stmt = db.prepare("SELECT loan_id, amount FROM fines")?;
    let fines: Vec<(i64, f64)> = stmt
        .query_map([], |row| Ok((row.get(0)?, row.get(1)?)))?
        .collect::<Result<_, _>>()?;
    // region: use
    let mut owed: HashMap<i64, Cents> = HashMap::new();
    for (loan_id, amount) in &fines {
        let total = owed.entry(*loan_id).or_default();
        *total = *total + Cents::from_dollars(*amount);
    }
    println!("loan 1 owes {}", owed[&1]);
    println!("loan 4 owes {}", owed[&4]);
    println!("same amount? {}", owed[&1] == owed[&4]);
    println!("loan 7 owes {}", owed[&7]);
    // endregion
    println!("---");
    // region: negative
    println!("{}", Cents(-150));
    println!("{}", Cents(-5));
    // endregion
    Ok(())
}
