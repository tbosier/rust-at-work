#[allow(clippy::useless_vec)]
fn main() {
    // region: refs
    let qty = vec![12, 0, 7, 45];
    let mut big = 0;
    for q in &qty {
        if *q > 10 {
            big += 1;
        }
    }
    println!("{big} big orders out of {}", qty.len());

    let first = &qty[0];
    println!("first doubled: {}", *first * 2);
    // endregion
}
