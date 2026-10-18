use snippets::library;
// region: index
use std::collections::{HashMap, HashSet};
// endregion
use std::error::Error;

struct Book {
    book_id: i64,
    title: String,
}

struct Loan {
    loan_id: i64,
    book_id: i64,
}

fn main() -> Result<(), Box<dyn Error>> {
    let db = library()?;
    let mut stmt = db.prepare("SELECT book_id, title FROM books")?;
    let books: Vec<Book> = stmt
        .query_map([], |r| {
            Ok(Book {
                book_id: r.get(0)?,
                title: r.get(1)?,
            })
        })?
        .collect::<Result<_, _>>()?;
    let mut stmt = db.prepare("SELECT loan_id, book_id FROM loans")?;
    let mut loans: Vec<Loan> = stmt
        .query_map([], |r| {
            Ok(Loan {
                loan_id: r.get(0)?,
                book_id: r.get(1)?,
            })
        })?
        .collect::<Result<_, _>>()?;
    loans.push(Loan {
        loan_id: 8,
        book_id: 999,
    }); // a book nobody catalogued

    // region: index
    let by_id: HashMap<i64, &Book> = books.iter().map(|b| (b.book_id, b)).collect();

    let mut borrowed_before: HashSet<i64> = HashSet::new();
    for loan in &loans {
        let Some(book) = by_id.get(&loan.book_id) else {
            println!(
                "loan {}: no book {} in the catalog",
                loan.loan_id, loan.book_id
            );
            continue;
        };
        let again = !borrowed_before.insert(book.book_id);
        println!(
            "loan {}: {}{}",
            loan.loan_id,
            book.title,
            if again { " (again)" } else { "" }
        );
    }
    // endregion
    println!("---");
    // region: combinators
    let title = |id: i64| {
        by_id
            .get(&id)
            .map(|b| b.title.as_str())
            .unwrap_or("(unknown)")
    };
    println!("{}", title(104));
    println!("{}", title(999));

    let copies: HashMap<i64, u32> = HashMap::from([(101, 2), (103, 1)]);
    let n = |id: i64| copies.get(&id).copied().unwrap_or_default();
    println!("copies of 101: {}, of 105: {}", n(101), n(105));
    // endregion
    Ok(())
}
