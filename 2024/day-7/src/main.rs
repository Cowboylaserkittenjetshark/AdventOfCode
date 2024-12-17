use clap::Parser;
use itertools::{repeat_n, Itertools};
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
    let mut total_calibration_result = 0;
    'line: for line in input.lines().map(|res| res.unwrap_or_else(|e| die(e))) {
        let mut tokens = line.split_whitespace();
        let test_value = tokens.next().unwrap_or_else(|| die("Invalid input file"));
        let test_value = test_value[..test_value.len() - 1].parse().unwrap();
        let tokens = tokens.map(|s| s.parse::<u64>().unwrap_or_else(|e| die(e)));
        let nums = tokens.collect_vec();
        let op_perms = repeat_n(['+', '*'], nums.len() - 1).multi_cartesian_product();
        for perm in op_perms {
            let mut nums = nums.iter();
            let ops = perm.iter();
            let a = nums.next().unwrap_or_else(|| die("Invalid input file"));
            let res_value = do_op(*a, nums, ops);
            if res_value == test_value {
                total_calibration_result += test_value;
                continue 'line;
            }
        }
    }
    println!("Total calibration result: {total_calibration_result}");
}

fn part_2() {
    let input = BufReader::new(File::open(Args::parse().input).unwrap_or_else(|err| die(err)));
    let mut total_calibration_result = 0;
    'line: for line in input.lines().map(|res| res.unwrap_or_else(|e| die(e))) {
        let mut tokens = line.split_whitespace();
        let test_value = tokens.next().unwrap_or_else(|| die("Invalid input file"));
        let test_value = test_value[..test_value.len() - 1].parse().unwrap();
        let tokens = tokens.map(|s| s.parse::<u64>().unwrap_or_else(|e| die(e)));
        let nums = tokens.collect_vec();
        let op_perms = repeat_n(['+', '*', 'c'], nums.len() - 1).multi_cartesian_product();
        for perm in op_perms {
            let mut nums = nums.iter();
            let ops = perm.iter();
            let a = nums.next().unwrap_or_else(|| die("Invalid input file"));
            let res_value = do_op(*a, nums, ops);
            if res_value == test_value {
                total_calibration_result += test_value;
                continue 'line;
            }
        }
    }
    println!("Total calibration result: {total_calibration_result}");
}

fn do_op(a: u64, mut nums: std::slice::Iter<'_, u64>, mut ops: std::slice::Iter<'_, char>) -> u64 {
    if let Some(b) = nums.next() {
        match ops
            .next()
            .expect("Operator permutation had an unexpected length")
        {
            '+' => do_op(a + b, nums, ops),
            '*' => do_op(a * b, nums, ops),
            'c' => do_op(string_concat_nums(a, *b), nums, ops),
            _ => unreachable!("ops will only contain + and * and c"),
        }
    } else {
        return a;
    }
}

fn string_concat_nums(a: u64, b: u64) -> u64 {
    let a = a.to_string();
    let b = b.to_string();
    let concated = a + b.as_str();
    return concated.parse().unwrap();
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
