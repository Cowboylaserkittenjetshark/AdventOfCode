use clap::Parser;
use itertools::{chain, Itertools};
use std::{
    fs::File,
    io::{BufRead, BufReader},
    iter,
    path::PathBuf,
    process::exit,
};

fn main() {
    part_1();
    part_2();
}

fn part_1() {
    let input = BufReader::new(File::open(Args::parse().input).unwrap_or_else(|err| die(err)));
    let mut safe_count = 0;

    'report_layer: for line in input.lines() {
        if let Ok(report) = line {
            let levels = report
                .split_whitespace()
                .map(|s| s.parse::<i32>().unwrap_or_else(|e| die(e)));
            let mut orientation: LevelOrientation = LevelOrientation::None;
            for (a, b) in levels.tuple_windows() {
                let local_orientation;

                if a < b {
                    local_orientation = LevelOrientation::Inc;
                } else if a > b {
                    local_orientation = LevelOrientation::Dec;
                } else {
                    continue 'report_layer;
                }

                if let LevelOrientation::None = orientation {
                    orientation = local_orientation;
                } else {
                    if local_orientation != orientation {
                        continue 'report_layer;
                    }
                }

                let diff = a.abs_diff(b);
                if diff < 1 || diff > 3 {
                    continue 'report_layer;
                }
            }
        } else {
            die("Invalid input file");
        }
        safe_count += 1;
    }
    println!("Safe reports: {safe_count}");
}

fn part_2() {
    let input = BufReader::new(File::open(Args::parse().input).unwrap_or_else(|err| die(err)));
    let mut safe_count = 0;

    for line in input.lines() {
        if let Ok(report) = line {
            let levels: Vec<i32> = report
                .split_whitespace()
                .map(|s| s.parse().unwrap_or_else(|e| die(e)))
                .collect();
            safe_count += chain!(
                iter::once(chain!(&levels[..], &levels[0..0])),
                (0..levels.len()).map(|i| chain!(&levels[0..i], &levels[i + 1..]))
            )
            .any(|r| {
                let mut orientation = 0;
                r.tuple_windows().all(|(a, b)| {
                    let diff = b - a;
                    let lo = diff.signum();
                    let diff = diff.abs();
                    if diff < 1 || diff > 3 || lo == 0 || (lo != orientation && orientation != 0) {
                        false
                    } else {
                        orientation = lo;
                        true
                    }
                })
            }) as i32;
        } else {
            die("Invalid input file");
        }
    }
    println!("Safe reports (damped): {safe_count}");
}

#[derive(PartialEq, Eq, Clone)]
enum LevelOrientation {
    None,
    Inc,
    Dec,
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
