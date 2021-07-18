use anyhow::{Context, Result};
use std::env::{self, Args};
use std::fs::File;
use std::io::{self, Read};
use std::process;

fn main() {
    let filename = parse_args(&mut env::args());
    let contents = read_input(&filename).unwrap_or_else(|err| {
        eprintln!("Could not read input: {}", err);
        process::exit(1);
    });

    print!("{}", contents);
}

fn parse_args(args: &mut Args) -> String {
    let mut filename = String::from("-");
    if let Some(f) = args.nth(1) {
        filename = f;
    };
    filename
}

fn read_input(filename: &str) -> Result<String> {
    let stdin; // for holding a reference throughout the scope
    let mut input: Box<dyn Read> =
        if filename == "-" {
            stdin = io::stdin();
            Box::new(stdin.lock())
        } else {
            Box::new(File::open(filename).with_context(|| {
                format!("Could not open file {:?}", &filename)
            })?)
        };

    let mut s = String::new();
    input.read_to_string(&mut s)?;
    Ok(s)
}
