use std::process::Command;

#[allow(clippy::useless_vec)]
fn main() {
    match std::env::args().nth(1).as_deref() {
        Some("index") => {
            // region: index
            let qty = vec![12, 0, 7];
            let day = 3;
            println!("{}", qty[day]);
            // endregion
            return;
        }
        Some("unwrap") => {
            // region: unwrap
            let qty: u32 = "forty".parse().unwrap();
            // endregion
            println!("{qty}");
            return;
        }
        Some("overflow") => {
            // region: overflow
            let stock: u8 = std::hint::black_box(250);
            let more = stock + 10;
            // endregion
            println!("{more}");
            return;
        }
        _ => {}
    }
    // Run this same program as a child for each case, to show the panic.
    let me = std::env::current_exe().expect("own path");
    for (i, case) in ["index", "unwrap", "overflow"].iter().enumerate() {
        if i > 0 {
            println!("---");
        }
        let out = Command::new(&me)
            .arg(case)
            .env_remove("RUST_BACKTRACE")
            .output()
            .expect("child runs");
        print!("{}", String::from_utf8_lossy(&out.stderr));
        println!("exit code: {:?}", out.status.code());
    }
}
