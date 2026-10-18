// region: uses
use rusqlite::Connection;
use snippets::library;
// endregion

// The duration parser from the wiki, so this file stands alone.
#[path = "t4_duration.rs"]
#[allow(dead_code)]
mod duration;
use duration::{DurationError, parse_duration};

// region: apperror
use std::fmt;

#[derive(Debug)]
pub enum RenewalError {
    Db(rusqlite::Error),
    BadSlip {
        slip_id: i64,
        problem: DurationError,
    },
}

impl fmt::Display for RenewalError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RenewalError::Db(e) => write!(f, "database: {e}"),
            RenewalError::BadSlip { slip_id, problem } => write!(f, "slip {slip_id}: {problem}"),
        }
    }
}

impl std::error::Error for RenewalError {}

impl From<rusqlite::Error> for RenewalError {
    fn from(e: rusqlite::Error) -> RenewalError {
        RenewalError::Db(e)
    }
}
// endregion

// region: uses
fn total_days(db: &Connection) -> Result<u64, RenewalError> {
    let mut stmt = db.prepare("SELECT slip_id, asked_for FROM renewal_slips")?;
    let slips = stmt
        .query_map([], |row| {
            Ok((row.get::<_, i64>(0)?, row.get::<_, String>(1)?))
        })?
        .collect::<rusqlite::Result<Vec<_>>>()?;
    let mut total: u64 = 0;
    for (slip_id, asked_for) in slips {
        let days = parse_duration(&asked_for)
            .map_err(|problem| RenewalError::BadSlip { slip_id, problem })?;
        total += u64::from(days);
    }
    Ok(total)
}
// endregion

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // region: run
    let db = library()?;
    db.execute_batch(
        "CREATE TABLE renewal_slips (slip_id INTEGER, asked_for TEXT);
         INSERT INTO renewal_slips VALUES (1, '14d'), (2, '2 weeks'), (3, '7 days');",
    )?;
    println!("{:?}", total_days(&db).map_err(|e| e.to_string()));
    db.execute("INSERT INTO renewal_slips VALUES (4, 'a fortnight')", [])?;
    println!("{:?}", total_days(&db).map_err(|e| e.to_string()));
    db.execute("DROP TABLE renewal_slips", [])?;
    println!("{:?}", total_days(&db).map_err(|e| e.to_string()));
    // endregion
    Ok(())
}
