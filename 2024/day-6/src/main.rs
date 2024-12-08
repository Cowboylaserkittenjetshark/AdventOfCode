use clap::Parser;
use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
    process::exit,
    thread::sleep,
    time::Duration,
};
use GuardHeading::{EAST, NORTH, SOUTH, WEST};

fn main() {
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
        // for y in 0..y_dim {
        //     for x in 0..x_dim {
        //         let coords = (x, y);
        //         if obstacle_coords.contains(&coords) {
        //             print!("#");
        //         } else if coords == (gx, gy) {
        //             let guard_char = match guard_heading {
        //                 NORTH => '^',
        //                 EAST => '>',
        //                 SOUTH => 'v',
        //                 WEST => '<',
        //             };
        //             print!("{guard_char}");
        //         } else {
        //             print!(".");
        //         }
        //     }
        //     println!();
        // }
        // sleep(Duration::from_millis(500));
    }
    println!("No. positions: {}", positions.len());
}
#[derive(Debug)]
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
