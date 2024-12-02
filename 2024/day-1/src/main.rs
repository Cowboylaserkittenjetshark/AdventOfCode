use clap::Parser;
use std::{
    cmp::{max, min},
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    iter::zip,
    path::PathBuf,
    process::exit,
};

fn main() {
    let input = BufReader::new(File::open(Args::parse().input).unwrap_or_else(|err| die(err)));

    let mut left: Vec<u32> = Vec::with_capacity(500);
    let mut right: Vec<u32> = Vec::with_capacity(500);

    for line in input.lines() {
        if let Ok(line) = line {
            let mut nums = line
                .split_whitespace()
                .map(|string| string.parse::<u32>().unwrap_or_else(|e| die(e)));
            left.push(nums.next().unwrap());
            right.push(nums.next().unwrap());
        } else {
            die("Invalid input file")
        };
    }

    left.sort_unstable();
    right.sort_unstable();
    let sorted = zip(&left, &right);
    let distance: u32 = sorted.map(|(l, r)| max(l, r) - min(l, r)).sum();
    println!("{distance}");

    let mut counts: HashMap<u32, u32> = HashMap::new();
    // for id in left {
    //     counts.entry(id).or_insert(0);
    // }
    for id in right {
        counts
            .entry(id)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }
    // println!("{}", counts.into_values().sum::<u32>());
    // let score: u32 = counts.iter().map(|(key, value)| key * value).sum();
    let score: u32 = left
        .iter()
        .map(|k| *k * *counts.entry(*k).or_default())
        .sum();
    println!("Similarity Score: {score}");
}

fn die<T: ToString>(err: T) -> ! {
    eprintln!("Error: {}", err.to_string());
    exit(1);
}

#[derive(Parser, Debug)]
struct Args {
    /// Path to the input file
    input: PathBuf,
}
