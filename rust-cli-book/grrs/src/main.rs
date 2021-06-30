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
    println!("args: {:?}", args);
}
