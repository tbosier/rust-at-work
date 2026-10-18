fn main() {
    let qty = vec![12, 0, 7, 45];
    let mut big = 0;
    for q in &qty {
        if q > 10 {
            big += 1;
        }
    }
    println!("{big}");
}
