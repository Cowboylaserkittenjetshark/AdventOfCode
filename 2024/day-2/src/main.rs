use clap::Parser;
use std::{fs::read_to_string, path::PathBuf, process::exit};

fn main() {
    let file = Args::parse().input;
    let input = read_to_string(file).unwrap_or_else(|e| {
        eprintln!("Error: {e}");
        exit(1)
    });
}

#[derive(Parser, Debug)]
struct Args {
    /// Path to the input file
    input: PathBuf,
}
