use std::process::Command;

fn main() {
    if std::env::args().nth(1).as_deref() == Some("child") {
        // region: dbg
        let qty = 40;
        let price_cents = 1_299;
        let total = dbg!(qty * price_cents);
        dbg!(total > 50_000);
        println!("total is {total}");
        // endregion
        return;
    }
    // Run this same program as a child so its stderr can be shown.
    let me = std::env::current_exe().expect("own path");
    let out = Command::new(me).arg("child").output().expect("child runs");
    print!("{}", String::from_utf8_lossy(&out.stderr));
    print!("{}", String::from_utf8_lossy(&out.stdout));
}
