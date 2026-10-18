fn main() {
    // region: bits
    println!("{:>4} = {:08b}", 5u8, 5u8);
    println!("{:>4} = {:08b}", 200u8, 200u8);
    println!("{:>4} = {:08b}", -1i8, -1i8);
    println!("{:>4} = {:08b}", -128i8, -128i8);
    println!("u8  {} to {}", u8::MIN, u8::MAX);
    println!("i8  {} to {}", i8::MIN, i8::MAX);
    println!("i64 {} to {}", i64::MIN, i64::MAX);
    // endregion
    println!("---");
    // region: overflow
    let stock: u8 = 250;
    println!("{:?}", stock.checked_add(5));
    println!("{:?}", stock.checked_add(10));
    println!("{}", stock.wrapping_add(10));
    println!("{}", stock.saturating_add(10));
    // endregion
    println!("---");
    // region: sizes
    use std::mem::size_of;
    println!("u8    {} byte", size_of::<u8>());
    println!("bool  {} byte", size_of::<bool>());
    println!("i32   {} bytes", size_of::<i32>());
    println!("u32   {} bytes", size_of::<u32>());
    println!("char  {} bytes", size_of::<char>());
    println!("i64   {} bytes", size_of::<i64>());
    println!("f64   {} bytes", size_of::<f64>());
    println!("usize {} bytes", size_of::<usize>());
    println!("&i64  {} bytes", size_of::<&i64>());
    // endregion
}
