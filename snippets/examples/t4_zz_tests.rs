// Hidden. Runs `cargo test` in the failing card crate ticket 4 shows,
// prints the output for the page, and checks that every
// <pre data-file="..."> in any section matches the file it claims to show.
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

fn unescape(s: &str) -> String {
    s.replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&quot;", "\"")
        .replace("&amp;", "&")
}

fn check_shown_files() -> Result<(), String> {
    let mut n = 0;
    let dir = std::fs::read_dir("../../src/sections").map_err(|e| e.to_string())?;
    let mut paths: Vec<_> = dir.filter_map(|e| e.ok()).map(|e| e.path()).collect();
    paths.sort();
    for p in paths {
        let page = std::fs::read_to_string(&p).map_err(|e| e.to_string())?;
        n += check_page(&page)?;
    }
    eprintln!("{n} shown files match");
    Ok(())
}

fn check_page(page: &str) -> Result<usize, String> {
    let mut rest = page;
    let mut n = 0;
    while let Some(i) = rest.find("<pre data-file=\"") {
        rest = &rest[i + 16..];
        let (path, after) = rest.split_once('"').ok_or("unclosed data-file")?;
        let body_start = after.find('>').ok_or("unclosed pre tag")? + 1;
        let body_end = after.find("</pre>").ok_or("unclosed pre")?;
        let shown = unescape(&after[body_start..body_end]);
        let real =
            std::fs::read_to_string(format!("../{path}")).map_err(|e| format!("{path}: {e}"))?;
        if shown.trim() != real.trim() {
            return Err(format!("page text for {path} differs from the file"));
        }
        n += 1;
        rest = &after[body_end..];
    }
    Ok(n)
}

fn main() {
    if let Err(e) = check_shown_files() {
        eprintln!("{e}");
        std::process::exit(1);
    }
    println!("{}", cargo_test("../t4_failing"));
}
