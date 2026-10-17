use shelf::duration::parse_duration;

fn main() {
    for raw in std::env::args().skip(1) {
        match parse_duration(&raw) {
            Ok(days) => println!("{raw}: {days} days"),
            Err(e) => eprintln!("{raw}: {e}"),
        }
    }
}
