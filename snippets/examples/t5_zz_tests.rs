// Hidden. Runs `cargo test` in the shelf crate ticket 5 shows (library,
// binary and integration test) and prints the output for the page.
use std::process::Command;

fn cargo_test(dir: &str) -> String {
    let out = Command::new("sh")
        .args(["-c", "cargo test 2>&1"])
        .current_dir(dir)
        .env("CARGO_HOME", "../../.cargo-home")
        .env("CARGO_TERM_COLOR", "never")
        .env_remove("RUST_BACKTRACE")
        .output()
        .expect("cargo runs");
    let text = String::from_utf8_lossy(&out.stdout);
    let mut lines = Vec::new();
    for line in text.lines() {
        let t = line.trim_start();
        if [
            "Compiling ",
            "Finished ",
            "Blocking ",
            "Locking ",
            "Updating ",
        ]
        .iter()
        .any(|p| t.starts_with(p))
        {
            continue;
        }
        // Timings change run to run; pin them.
        let line = match line.find("finished in ") {
            Some(i) => format!("{}finished in 0.00s", &line[..i]),
            None => line.to_string(),
        };
        lines.push(line);
    }
    lines.join("\n").trim_matches('\n').to_string()
}

fn main() {
    println!("{}", cargo_test("../t5_crate"));
}
