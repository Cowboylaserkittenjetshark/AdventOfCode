use clap::Parser;
use itertools::Itertools;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    iter::zip,
    path::PathBuf,
    process::exit,
};

fn main() {
    part_1();
    part_2();
}

fn part_1() {
    let input = BufReader::new(File::open(Args::parse().input).unwrap_or_else(|err| die(err)));
    let mut count = 0;
    let cw: Vec<Vec<char>> = input
        .lines()
        .map(|res| res.unwrap_or_else(|e| die(e)).chars().collect())
        .collect();
    count += cw.iter().map(|row| count_xmas(row)).sum::<usize>();
    count += (0..cw.len())
        .map(|i| count_xmas(cw.iter().map(|row| row[i]).collect_vec().as_ref()))
        .sum::<usize>();
    let row_len = cw[0].len() as isize;
    let col_len = cw.len() as isize;
    let maj_dim = row_len.max(col_len);
    let min_dim = row_len.min(col_len);
    let mut combos = Vec::new();
    if row_len >= 4 && col_len >= 4 {
        for ri in (-1 * (min_dim - 4))..=(maj_dim - 4) {
            let mut ldia = Vec::with_capacity(1);
            let mut rdia = Vec::with_capacity(1);
            for ci in 0..row_len {
                let (a, b) = (ci, ci + ri);
                if (0..row_len).contains(&a) && (0..col_len).contains(&b) {
                    ldia.push(cw[b as usize][a as usize]);
                    rdia.push(cw[b as usize][(row_len - 1 - a) as usize]);
                }
            }
            println!("Left diagonal: {ldia:?}");
            println!("Right diagonal: {rdia:?}");
            combos.push(ldia);
            combos.push(rdia);
        }
    }
    for combo in combos {
        count += count_xmas(&combo);
    }
    println!("Count: {count}");
}

fn part_2() {
    let input = BufReader::new(File::open(Args::parse().input).unwrap_or_else(|err| die(err)));
    let mut count = 0;
    let cw: Vec<Vec<char>> = input
        .lines()
        .map(|res| res.unwrap_or_else(|e| die(e)).chars().collect())
        .collect();
    let row_len = cw[0].len() as isize;
    let col_len = cw.len() as isize;
    if row_len >= 3 && col_len >= 3 {
        let maj_dim = row_len.max(col_len);
        let min_dim = row_len.min(col_len);

        let mut ldia_combos = Vec::new();
        let mut rdia_combos = Vec::new();

        for ri in (-1 * (min_dim - 3))..=(maj_dim - 3) {
            let mut ldia = Vec::with_capacity(1);
            let mut rdia = Vec::with_capacity(1);
            for ci in 0..row_len {
                let (a, b) = (ci, ci + ri);
                if (0..row_len).contains(&a) && (0..col_len).contains(&b) {
                    ldia.push((a as usize, b as usize));
                    rdia.push(((row_len - 1 - a) as usize, b as usize));
                }
            }
            ldia_combos.push(ldia);
            rdia_combos.push(rdia);
        }

        let mut ldia_coords = Vec::new();
        let mut rdia_coords = Vec::new();

        for (ldia_combo, rdia_combo) in zip(ldia_combos, rdia_combos) {
            for i in 1..ldia_combo.len() - 1 {
                let (x, y) = ldia_combo[i];
                if let ('M', 'A', 'S') | ('S', 'A', 'M') =
                    (cw[y + 1][x - 1], cw[y][x], cw[y - 1][x + 1])
                {
                    ldia_coords.push((x, y));
                }
                let (x, y) = rdia_combo[i];
                if let ('M', 'A', 'S') | ('S', 'A', 'M') =
                    (cw[y + 1][x + 1], cw[y][x], cw[y - 1][x - 1])
                {
                    rdia_coords.push((x, y));
                }
            }
        }

        count += ldia_coords
            .iter()
            .filter(|c| rdia_coords.iter().contains(c))
            .count();
    }
    println!("Count: {count}");
}

fn count_xmas(combo: &Vec<char>) -> usize {
    combo
        .iter()
        .tuple_windows::<(&char, &char, &char, &char)>()
        .filter(|&(&a, &b, &c, &d)| {
            (a, b, c, d) == ('X', 'M', 'A', 'S') || (a, b, c, d) == ('S', 'A', 'M', 'X')
        })
        .count()
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
