use clap::Parser;
use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
    process::exit,
};
use GuardHeading::{EAST, NORTH, SOUTH, WEST};

fn main() {
    part_1();
    part_2();
}

fn part_1() {
    let input = BufReader::new(File::open(Args::parse().input).unwrap_or_else(|err| die(err)));

    let mut x_dim = None;
    let y_dim;
    let mut x = 0;
    let mut y = 0;
    let mut obstacle_coords = HashSet::new();
    let mut guard_coords = None;
    let mut guard_heading = NORTH;
    let mut positions = HashSet::new();

    for line in input.lines().map(|res| res.unwrap_or_else(|e| die(e))) {
        if x_dim.is_none() {
            x_dim = Some(line.len());
        }
        for c in line.chars() {
            if c == '#' {
                obstacle_coords.insert((x, y));
            } else if c == '^' {
                if guard_coords.is_none() {
                    guard_coords = Some((x, y));
                } else {
                    die("Invalid input: multiple guard characters");
                }
            }
            x += 1;
        }
        x = 0;
        y += 1;
    }
    let x_dim = x_dim.unwrap_or_else(|| die("Invalid input: zero length"));
    y_dim = y;

    let (mut gx, mut gy) = guard_coords.unwrap_or_else(|| die("Invalid Input: no guard character"));
    positions.insert((gx, gy));
    while !(match guard_heading {
        NORTH => gy == 0,
        EAST => gx == x_dim - 1,
        SOUTH => gy == y_dim - 1,
        WEST => gx == 0,
    }) {
        let potential_obstacle = match guard_heading {
            NORTH => (gx, gy - 1),
            EAST => (gx + 1, gy),
            SOUTH => (gx, gy + 1),
            WEST => (gx - 1, gy),
        };
        if obstacle_coords.contains(&potential_obstacle) {
            guard_heading.turn_right();
        } else {
            (gx, gy) = potential_obstacle;
            positions.insert(potential_obstacle);
        }
    }
    println!("No. positions: {}", positions.len());
}

// Wonderfully efficient!
// ❯ time cargo run -- input.txt
//     Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.03s
//      Running `target/debug/day-6 input.txt`
// No. positions: 5067
// Possible loops: 1793
// cargo run -- input.txt  115.16s user 0.83s system 99% cpu 1:56.62 total
//
// ❯ time cargo run --release -- input.txt
//     Finished `release` profile [optimized] target(s) in 0.02s
//      Running `target/release/day-6 input.txt`
// No. positions: 5067
// Possible loops: 1793
// cargo run --release -- input.txt  7.35s user 0.75s system 99% cpu 8.151 total

fn part_2() {
    let input = BufReader::new(File::open(Args::parse().input).unwrap_or_else(|err| die(err)));

    let mut x_dim = None;
    let y_dim;
    let mut x = 0;
    let mut y = 0;
    let mut obstacle_coords = HashSet::new();
    let mut guard_coords = None;

    for line in input.lines().map(|res| res.unwrap_or_else(|e| die(e))) {
        if x_dim.is_none() {
            x_dim = Some(line.len());
        }
        for c in line.chars() {
            if c == '#' {
                obstacle_coords.insert((x, y));
            } else if c == '^' {
                if guard_coords.is_none() {
                    guard_coords = Some((x, y));
                } else {
                    die("Invalid input: multiple guard characters");
                }
            }
            x += 1;
        }
        x = 0;
        y += 1;
    }
    let x_dim = x_dim.unwrap_or_else(|| die("Invalid input: zero length"));
    y_dim = y;

    let guard_coords = guard_coords.unwrap_or_else(|| die("Invalid Input: no guard character"));

    let mut loops = 0;
    for oy in 0..y_dim {
        'ox_coord: for ox in 0..x_dim {
            if obstacle_coords.contains(&(ox, oy)) {
                continue;
            }
            let (mut gx, mut gy) = guard_coords;
            let mut guard_heading = NORTH;
            let mut positions = HashSet::new();
            positions.insert((gx, gy, guard_heading));
            while !(match guard_heading {
                NORTH => gy == 0,
                EAST => gx == x_dim - 1,
                SOUTH => gy == y_dim - 1,
                WEST => gx == 0,
            }) {
                let potential_obstacle = match guard_heading {
                    NORTH => (gx, gy - 1),
                    EAST => (gx + 1, gy),
                    SOUTH => (gx, gy + 1),
                    WEST => (gx - 1, gy),
                };
                if obstacle_coords.contains(&potential_obstacle) || potential_obstacle == (ox, oy) {
                    guard_heading.turn_right();
                } else {
                    (gx, gy) = potential_obstacle;
                }
                if positions.contains(&(gx, gy, guard_heading)) {
                    loops += 1;
                    continue 'ox_coord;
                } else {
                    positions.insert((gx, gy, guard_heading));
                }
            }
        }
    }
    println!("Possible loops: {loops}");
}

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
enum GuardHeading {
    NORTH,
    EAST,
    SOUTH,
    WEST,
}

impl GuardHeading {
    fn turn_right(&mut self) {
        *self = match self {
            Self::NORTH => Self::EAST,
            Self::EAST => Self::SOUTH,
            Self::SOUTH => Self::WEST,
            Self::WEST => Self::NORTH,
        };
    }
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
