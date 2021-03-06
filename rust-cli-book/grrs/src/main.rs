use anyhow::{Context, Result};
use std::io::BufReader;
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

    grrs::find_matches(f, &args.pattern, &mut std::io::stdout())
}
