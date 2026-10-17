use std::process::Command;

fn main() {
    if std::env::args().nth(1).as_deref() == Some("child") {
        // region: panic
        let days: u32 = "TBD".parse().unwrap();
        // endregion
        println!("{days}");
        return;
    }
    // Run this same program as a child so its panic message can be shown.
    let me = std::env::current_exe().expect("own path");
    let out = Command::new(me)
        .arg("child")
        .env_remove("RUST_BACKTRACE")
        .output()
        .expect("child runs");
    print!("{}", String::from_utf8_lossy(&out.stderr));
    println!("exit code: {:?}", out.status.code());
}
