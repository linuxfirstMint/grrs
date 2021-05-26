use anyhow::{Context, Result};
use std::fs::File;
use std::io::{prelude::*, BufReader};
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

fn main() -> Result<()> {
    let args = Cli::from_args();
    let f = File::open(&args.path)
        .with_context(|| format!("could not read file `{:?}`", &args.path))?;
    let content = BufReader::new(f);
    for line in content.lines() {
        let s = line.unwrap();
        if s.contains(&args.pattern) {
            println!("{:?}", s);
        }
    }
    Ok(())
}
