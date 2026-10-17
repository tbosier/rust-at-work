use std::collections::HashMap;

struct Book {
    book_id: i64,
    title: String,
}

struct Catalog<'a> {
    by_id: HashMap<i64, &'a Book>,
}

impl<'a> Catalog<'a> {
    fn new(books: &'a [Book]) -> Catalog<'a> {
        Catalog {
            by_id: books.iter().map(|b| (b.book_id, b)).collect(),
        }
    }

    fn title(&self, id: i64) -> Option<&'a str> {
        self.by_id.get(&id).map(|b| b.title.as_str())
    }
}

fn load_catalog<'a>() -> Catalog<'a> {
    let books = vec![Book {
        book_id: 101,
        title: "The Mythical Man-Month".to_string(),
    }];
    Catalog::new(&books)
}

fn main() {
    let catalog = load_catalog();
    println!("{:?}", catalog.title(101));
}
