use std::os::unix::process::CommandExt;
use std::process::Command;

// region: main
use clap::Parser;
use std::process::ExitCode;

#[derive(Parser)]
#[command(name = "fines")]
struct Cli {
    /// The library database
    #[arg(long, default_value = "library.db")]
    db: String,
}

fn run(cli: &Cli) -> Result<usize, String> {
    if !std::path::Path::new(&cli.db).exists() {
        return Err(format!("database not found: {}", cli.db));
    }
    // A stand-in for the real work: this example is about exit codes,
    // so it only checks the file exists and reports a made-up count.
    Ok(7)
}

fn real_main() -> ExitCode {
    let cli = Cli::parse(); // bad arguments: clap prints why and exits with 2
    match run(&cli) {
        Ok(n) => {
            println!("wrote {n} fines");
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("error: {e}");
            ExitCode::FAILURE // exit status 1
        }
    }
}
// endregion

// When verify.py runs this with no arguments, it re-runs itself with the
// arguments a user might type and reports what a shell would see.
fn main() -> ExitCode {
    if std::env::var_os("T5_EXIT_CHILD").is_some() {
        return real_main();
    }
    harness().expect("harness");
    ExitCode::SUCCESS
}

fn harness() -> Result<(), Box<dyn std::error::Error>> {
    let me = std::env::current_exe()?;
    std::fs::write("library.db", b"")?;
    for args in [
        vec!["--db", "library.db"],
        vec!["--db", "missing.db"],
        vec!["--month", "2026-08"],
    ] {
        let out = Command::new(&me)
            .arg0("fines")
            .env("T5_EXIT_CHILD", "1")
            .args(&args)
            .output()?;
        println!("$ fines {}", args.join(" "));
        print!("{}", String::from_utf8(out.stdout)?);
        for line in String::from_utf8(out.stderr)?.lines() {
            println!("{line}");
        }
        println!("$ echo $?\n{}\n", out.status.code().unwrap_or(-1));
    }
    std::fs::remove_file("library.db")?;
    Ok(())
}
