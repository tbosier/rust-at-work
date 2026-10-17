mod duration {
    fn parse_days(raw: &str) -> Option<u32> {
        raw.trim().strip_suffix('d')?.parse().ok()
    }
}

fn main() {
    println!("{:?}", duration::parse_days("14d"));
}
