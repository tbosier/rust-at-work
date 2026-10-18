// region: open
use rusqlite::Connection;
// endregion
// region: readonly
use rusqlite::OpenFlags;
// endregion
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // region: open
    let db = Connection::open("data/kettle_creek.db")?;
    let n: i64 = db.query_row("SELECT count(*) FROM suppliers", [], |row| row.get(0))?;
    println!("{n} suppliers");
    // endregion
    println!("---");
    // Run the next two in a scratch folder, so the stray file lands there.
    let dir = std::env::temp_dir().join("snip_t1_open");
    std::fs::create_dir_all(&dir)?;
    std::env::set_current_dir(&dir)?;
    let _ = std::fs::remove_file("data_kettle_creek.db");
    // region: typo
    let db = Connection::open("data_kettle_creek.db")?; // typo: _ for /
    let result = db.prepare("SELECT supplier_id, name FROM suppliers");
    println!("{:?}", result.err().map(|e| e.to_string()));
    // endregion
    println!("---");
    // region: readonly
    let flags = OpenFlags::SQLITE_OPEN_READ_ONLY;
    let result = Connection::open_with_flags("data/kettle-creek.db", flags); // typo: - for _
    println!("{:?}", result.err().map(|e| e.to_string()));
    // endregion
    let _ = std::fs::remove_file("data_kettle_creek.db");
    Ok(())
}
