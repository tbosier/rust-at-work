fn shipping_cents(boxes: i64) -> i64 {
    899 + boxes * 150;
}

fn main() {
    println!("{}", shipping_cents(3));
}
