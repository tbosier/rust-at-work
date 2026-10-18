use shelf::duration::{DurationError, parse_duration};

#[test]
fn a_whole_batch_of_slips() {
    let slips = ["14d", "2w", "", "3 months", "21 days"];
    let (good, bad): (Vec<_>, Vec<_>) = slips
        .iter()
        .map(|s| parse_duration(s))
        .partition(|r| r.is_ok());
    assert_eq!(good.len(), 3);
    assert_eq!(bad[0], Err(DurationError::Empty));
}
