use anyhow::{Context, Result};
use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;
use structopt::StructOpt;

/// Search for pattern in a file and display the lines that contain it.
#[derive(Debug)]
#[derive(StructOpt)]
struct Cli {
    /// The pattern to look for
    pattern: String,

    /// The path to the file to read
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() -> Result<()> {
    let args = Cli::from_args();
    let f = File::open(&args.path)
        .with_context(|| format!("Could not open file {:?}", &args.path))?;
    let f = BufReader::new(f);

    find_matches(f, &args.pattern, &mut std::io::stdout())
}

fn find_matches(content: impl BufRead, pattern: &str, mut writer: impl Write) -> Result<()> {
    for line in content.lines() {
        let line = line.context("Could not read line")?;
        if line.contains(pattern) {
            writeln!(writer, "{}", line).context("Could not output line")?;
        }
    }
    Ok(())
}

#[test]
fn find_a_match() {
    let mut result = Vec::new();

    find_matches(
        std::io::Cursor::new("lorem ipsum\ndolor sit amet"),
        "lorem",
        &mut result,
    ).expect("find_matches failed unexpectedly");
    assert_eq!(result, b"lorem ipsum\n");
}
