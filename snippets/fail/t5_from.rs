#[derive(Debug)]
enum SlipError {
    Empty,
}

fn days(raw: &str) -> Result<u32, SlipError> {
    if raw.is_empty() {
        return Err(SlipError::Empty);
    }
    let n = raw.parse::<u32>()?;
    Ok(n)
}

fn main() {
    println!("{:?}", days("14"));
}
