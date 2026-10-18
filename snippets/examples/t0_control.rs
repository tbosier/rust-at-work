fn main() {
    // region: if
    let qty = 40;
    let ships_as = if qty >= 100 {
        "pallet"
    } else if qty >= 10 {
        "case"
    } else {
        "single"
    };
    println!("{qty} ships as a {ships_as}");
    // endregion
    println!("---");
    // region: for
    for day in 1..4 {
        print!("{day} ");
    }
    println!();
    for day in 1..=4 {
        print!("{day} ");
    }
    println!();
    for day in (1..=4).rev() {
        print!("{day} ");
    }
    println!();
    for qty in [12, 0, 7, 0, 3] {
        if qty == 0 {
            continue;
        }
        print!("{qty} ");
    }
    println!();
    // endregion
    println!("---");
    // region: while
    let mut stock = 25;
    let mut orders = 0;
    while stock >= 8 {
        stock -= 8;
        orders += 1;
    }
    println!("{orders} orders of 8, {stock} left");

    let mut on_hand = 100;
    let mut week = 0;
    let reorder_week = loop {
        week += 1;
        on_hand -= 30;
        if on_hand < 20 {
            break week;
        }
    };
    println!("reorder in week {reorder_week}");
    // endregion
    println!("---");
    // region: contains
    let day = 17;
    println!("{}", (10..=20).contains(&day));
    println!("{}", (10..17).contains(&day));
    // endregion
}
