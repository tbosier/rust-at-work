// region: hello
// A line that starts with two slashes is a comment.
fn main() {
    let ordered = 12;
    let per_box = 5;
    let boxes = boxes_needed(ordered, per_box);
    println!("{ordered} bolts need {boxes} boxes");
    println!("shipping costs {} cents", shipping_cents(boxes));
}

fn boxes_needed(items: i64, per_box: i64) -> i64 {
    if items == 0 {
        return 0;
    }
    let full = items / per_box;
    if items % per_box == 0 { full } else { full + 1 }
}

fn shipping_cents(boxes: i64) -> i64 {
    899 + boxes * 150
}
// endregion
