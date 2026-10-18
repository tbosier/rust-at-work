// region: trap
use snippets::library;
use std::collections::HashMap;
// endregion
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // region: trap
    let db = library()?;
    let mut stmt = db.prepare("SELECT loan_id, amount FROM fines")?;
    let fines: Vec<(i64, f64)> = stmt
        .query_map([], |row| Ok((row.get(0)?, row.get(1)?)))?
        .collect::<Result<_, _>>()?;

    let mut owed: HashMap<i64, f64> = HashMap::new();
    for (loan_id, amount) in &fines {
        *owed.entry(*loan_id).or_default() += amount;
    }
    println!("loan 1 owes {}", owed[&1]);
    println!("loan 4 owes {}", owed[&4]);
    println!("same amount? {}", owed[&1] == owed[&4]);
    println!("loan 7 owes {}", owed[&7]);
    println!("printed nicely: {:.2} and {:.2}", owed[&1], owed[&7]);
    // endregion
    println!("---");
    // region: exact
    println!("{:.30}", 0.1);
    println!("{:.30}", 0.2);
    println!("{:.30}", 0.3);
    println!("{:.30}", 0.1 + 0.2);
    // endregion
    Ok(())
}
