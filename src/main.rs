use std::path::PathBuf;
use structopt::StructOpt;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Debug, StructOpt)]
struct Cli {
    /// Patterns to search for
    pattern: String,
    /// File path of the search object
    #[structopt(parse(from_os_str))]
    path: PathBuf,
}

fn main() {
    let args = Cli::from_args();
    let content = std::fs::read_to_string(&args.path).expect("could not read file");
}
