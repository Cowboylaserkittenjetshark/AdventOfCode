use clap::Parser;
use regex::Regex;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
    process::exit,
};

fn main() {
    part_1();
    part_2();
}

fn part_1() {
    let input = BufReader::new(File::open(Args::parse().input).unwrap_or_else(|err| die(err)));
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let result = input
        .lines()
        .map(|l| {
            let line = l.unwrap_or_else(|e| die(e));
            re.captures_iter(&line)
                .map(|c| {
                    let (_, nums) = c.extract::<2>();
                    let mut nums = nums.iter().map(|n| n.parse::<i32>().unwrap());
                    nums.next().unwrap() * nums.next().unwrap()
                })
                .sum::<i32>()
        })
        .sum::<i32>();
    println!("{result}");
}

fn part_2() {
    let input = BufReader::new(File::open(Args::parse().input).unwrap_or_else(|err| die(err)));
    let re = Regex::new(r"mul\(\d{1,3},\d{1,3}\)|don't\(\)|do\(\)").unwrap();
    let mut enabled = true;
    let result = input
        .lines()
        .map(|l| {
            let line = l.unwrap_or_else(|e| die(e));
            re.find_iter(&line)
                .map(|m| {
                    if &m.as_str()[0..2] == "do" {
                        enabled = m.as_str() == "do()";
                    } else if enabled {
                        return line[m.range().start + 4..m.range().end - 1]
                            .split(',')
                            .map(|n| n.parse::<i32>().unwrap())
                            .reduce(|a, b| a * b)
                            .unwrap();
                    }
                    0
                })
                .sum::<i32>()
        })
        .sum::<i32>();
    println!("{result}");
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
