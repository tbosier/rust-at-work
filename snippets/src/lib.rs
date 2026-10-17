// region: library
use rusqlite::{Connection, Result};

/// A tiny lending library, built in memory, for trying things out.
pub fn library() -> Result<Connection> {
    let db = Connection::open_in_memory()?;
    db.execute_batch(
        "CREATE TABLE books (
             book_id INTEGER PRIMARY KEY,
             title   TEXT NOT NULL,
             author  TEXT NOT NULL
         );
         CREATE TABLE loans (
             loan_id     INTEGER PRIMARY KEY,
             book_id     INTEGER NOT NULL,
             member      TEXT NOT NULL,
             loaned_on   TEXT NOT NULL,
             returned_on TEXT,
             status      TEXT NOT NULL
         );
         CREATE TABLE fines (
             loan_id INTEGER NOT NULL,
             amount  REAL NOT NULL
         );
         INSERT INTO books VALUES
             (101, 'The Mythical Man-Month', 'Brooks'),
             (102, 'Structure and Interpretation of Computer Programs', 'Abelson'),
             (103, 'The Art of Computer Programming', 'Knuth'),
             (104, 'A Discipline of Programming', 'Dijkstra'),
             (105, 'Notes on the Synthesis of Form', 'Alexander');
         INSERT INTO loans VALUES
             (1, 101, 'Ada',    '2026-08-03', '2026-08-20', 'returned'),
             (2, 102, 'Grace',  '2026-08-11', NULL,         'out'),
             (3, 103, 'Alan',   '2026-09-01', NULL,         'out'),
             (4, 101, 'Grace',  '2026-09-02', '2026-09-15', 'returned'),
             (5, 104, 'Edsger', '2026-09-28', NULL,         'out'),
             (6, 105, 'Ada',    '2026-07-19', NULL,         'lost'),
             (7, 103, 'Grace',  '2026-07-30', '2026-08-29', 'returned');
         INSERT INTO fines VALUES
             (1, 0.10), (1, 0.20), (4, 0.30), (6, 24.95), (7, 0.70), (7, 0.10);",
    )?;
    Ok(db)
}
// endregion
