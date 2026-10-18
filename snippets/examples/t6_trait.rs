use rusqlite::Connection;
use snippets::library;
use std::error::Error;

struct Book {
    book_id: i64,
    title: String,
    author: String,
}

struct Loan {
    loan_id: i64,
    member: String,
    status: String,
}

// region: trait
/// Anything that can be shown as one row of a table.
trait TableRow {
    /// Column names. The same for every row of a type, so no `self`.
    fn headers() -> &'static [&'static str];
    /// This row's cells, in header order.
    fn cells(&self) -> Vec<String>;
}
// endregion

// region: impls
impl TableRow for Book {
    fn headers() -> &'static [&'static str] {
        &["book_id", "title", "author"]
    }
    fn cells(&self) -> Vec<String> {
        vec![
            self.book_id.to_string(),
            self.title.clone(),
            self.author.clone(),
        ]
    }
}

impl TableRow for Loan {
    fn headers() -> &'static [&'static str] {
        &["loan_id", "member", "status"]
    }
    fn cells(&self) -> Vec<String> {
        vec![
            self.loan_id.to_string(),
            self.member.clone(),
            self.status.clone(),
        ]
    }
}
// endregion

// region: generic
fn print_table<T: TableRow>(rows: &[T]) {
    println!("{}", T::headers().join(" | "));
    for row in rows {
        println!("{}", row.cells().join(" | "));
    }
}
// endregion

fn books(db: &Connection) -> rusqlite::Result<Vec<Book>> {
    let mut stmt = db.prepare("SELECT book_id, title, author FROM books")?;
    let rows = stmt.query_map([], |r| {
        Ok(Book {
            book_id: r.get(0)?,
            title: r.get(1)?,
            author: r.get(2)?,
        })
    })?;
    rows.collect()
}

fn loans(db: &Connection) -> rusqlite::Result<Vec<Loan>> {
    let mut stmt = db.prepare("SELECT loan_id, member, status FROM loans")?;
    let rows = stmt.query_map([], |r| {
        Ok(Loan {
            loan_id: r.get(0)?,
            member: r.get(1)?,
            status: r.get(2)?,
        })
    })?;
    rows.collect()
}

fn main() -> Result<(), Box<dyn Error>> {
    let db = library()?;
    // region: use
    let books = books(&db)?;
    let loans = loans(&db)?;
    print_table(&books[..2]);
    println!();
    print_table(&loans[..3]);
    // endregion
    Ok(())
}
