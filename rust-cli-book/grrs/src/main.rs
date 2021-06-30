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

fn main() {
    let args = Cli::from_args();
    let f = File::open(&args.path)
        .expect("Could not open file");
    let f = BufReader::new(f);

    for line in f.lines() {
        let line = line.unwrap();
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }
}
