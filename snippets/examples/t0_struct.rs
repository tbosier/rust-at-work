// region: struct
#[derive(Debug)]
struct Bin {
    location: String,
    capacity: u32,
    on_hand: u32,
}

impl Bin {
    fn new(location: &str, capacity: u32) -> Self {
        Bin {
            location: location.to_string(),
            capacity,
            on_hand: 0,
        }
    }

    fn space_left(&self) -> u32 {
        self.capacity - self.on_hand
    }

    fn put_away(&mut self, qty: u32) -> u32 {
        let stored = qty.min(self.space_left());
        self.on_hand += stored;
        stored
    }
}
// endregion

fn main() {
    // region: use
    let mut bin = Bin::new("A-04-2", 50);
    let stored = bin.put_away(30);
    println!("{}: stored {stored}, {} left", bin.location, bin.space_left());
    let stored = bin.put_away(30);
    println!("stored {stored}, {} left", bin.space_left());
    println!("{bin:?}");
    // endregion
}
