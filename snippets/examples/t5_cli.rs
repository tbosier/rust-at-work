// region: cli
use clap::Parser;

/// Fines report for the lending library.
#[derive(Parser, Debug)]
#[command(name = "fines")]
struct Cli {
    /// Shelf to report on: one capital letter and one or two digits, like B12.
    #[arg(long, value_parser = parse_shelf)]
    shelf: String,

    /// Only this member's fines.
    #[arg(long)]
    member: Option<String>,

    /// Where to write the workbook.
    #[arg(long, default_value = "out/fines.xlsx")]
    out: String,
}

/// Accept `B12` or `C7`; refuse anything else, saying why.
fn parse_shelf(s: &str) -> Result<String, String> {
    let mut chars = s.chars();
    let Some(letter) = chars.next() else {
        return Err("a shelf code can't be empty".to_string());
    };
    if !letter.is_ascii_uppercase() {
        return Err(format!("{s:?} should start with a capital letter"));
    }
    let number = chars.as_str();
    if number.is_empty() || number.len() > 2 || !number.bytes().all(|b| b.is_ascii_digit()) {
        return Err(format!("{s:?} needs one or two digits after the letter"));
    }
    Ok(s.to_string())
}
// endregion

fn main() {
    use clap::CommandFactory;
    // region: try
    let args = ["fines", "--shelf", "B12", "--member", "Grace"];
    let cli = Cli::try_parse_from(args).expect("good arguments");
    println!("{cli:?}");
    // endregion
    println!("---");
    // region: bad
    for args in [
        vec!["fines", "--shelf", "b12"],
        vec!["fines", "--shelf", "B123"],
        vec!["fines", "--member", "Grace"],
    ] {
        let err = Cli::try_parse_from(&args).unwrap_err();
        print!("{err}");
        println!("(exit code {})\n", err.exit_code());
    }
    // endregion
    println!("---");
    print!("{}", Cli::command().render_help());
}
