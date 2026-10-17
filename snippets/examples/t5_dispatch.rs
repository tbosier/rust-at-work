use std::mem::size_of;

// region: dyn
trait Describe {
    fn describe(&self) -> String;
}

struct Book {
    title: String,
}

struct Fine {
    cents: i64,
}

impl Describe for Book {
    fn describe(&self) -> String {
        format!("book: {}", self.title)
    }
}

impl Describe for Fine {
    fn describe(&self) -> String {
        format!("fine: {}.{:02}", self.cents / 100, self.cents % 100)
    }
}

fn describe_all(items: &[Box<dyn Describe>]) {
    for item in items {
        println!("{}", item.describe());
    }
}
// endregion

fn main() {
    // region: mixed
    let items: Vec<Box<dyn Describe>> = vec![
        Box::new(Book {
            title: "The Mythical Man-Month".to_string(),
        }),
        Box::new(Fine { cents: 2495 }),
        Box::new(Book {
            title: "A Discipline of Programming".to_string(),
        }),
    ];
    describe_all(&items);
    // endregion
    println!("---");
    // region: sizes
    println!("&Book:          {} bytes", size_of::<&Book>());
    println!("&dyn Describe:  {} bytes", size_of::<&dyn Describe>());
    println!(
        "Box<dyn Describe>: {} bytes",
        size_of::<Box<dyn Describe>>()
    );
    // endregion
}
