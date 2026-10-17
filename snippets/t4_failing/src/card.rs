/// The number on a library card like `LIB-0042`.
pub fn card_number(raw: &str) -> Option<u32> {
    raw.strip_prefix("LIB-")?.parse().ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plain_card() {
        assert_eq!(card_number("LIB-0042"), Some(42));
    }

    #[test]
    fn card_typed_with_spaces() {
        assert_eq!(card_number(" LIB-0042 "), Some(42));
    }
}
