use clap::Parser;
use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
    process::exit,
};

fn main() {
    let lines = BufReader::new(File::open(Args::parse().input).unwrap_or_else(|err| die(err)))
        .lines()
        .map(|res| res.unwrap_or_else(|e| die(e)));

    let mut nodes_by_freq: HashMap<char, Vec<(usize, usize)>> = HashMap::new();

    let mut x_dim = 0;
    let mut y_dim = 0;

    for (ri, row) in lines.enumerate() {
        let columns = row.chars().into_iter();
        let mut local_x_dim = 0;

        for (ci, col) in columns.enumerate() {
            local_x_dim = ci;
            if col == '.' {
                continue;
            }
            nodes_by_freq
                .entry(col)
                .and_modify(|coords| {
                    coords.push((ci, ri));
                })
                .or_insert(vec![(ci, ri)]);
        }

        y_dim += 1;
        x_dim = local_x_dim + 1;
    }

    let mut antinodes = HashSet::new();

    for (freq, coords) in nodes_by_freq {
        // let perms = coords
        //     .into_iter()
        //     .map(|(a, b)| (a as isize, b as isize))
        //     .permutations(2);
        // dbg!(perms.size_hint());
        for mut perm in coords
            .into_iter()
            .map(|(a, b)| (a as isize, b as isize))
            .combinations(2)
        {
            perm.sort_unstable_by(|a, b| a.0.cmp(&b.0));

            let aa = perm[0];
            let ab = perm[1];
            let x_dist = ab.0 - aa.0;
            let y_dist;

            if x_dist == 0 {
                y_dist = ab.1.max(aa.1) - ab.1.min(aa.1);
            } else {
                y_dist = ab.1 - aa.1;
            }

            let potential_antinodes = vec![
                (aa.0 - x_dist, aa.1 - y_dist),
                (ab.0 + x_dist, ab.1 + y_dist),
            ];

            for antinode in potential_antinodes {
                if (0..x_dim as isize).contains(&antinode.0) && (0..y_dim).contains(&antinode.1) {
                    eprintln!("Adding ({}, {})", antinode.0, antinode.1);
                    antinodes.insert(antinode);
                }
            }
        }
    }

    println!("Antinodes: {}", antinodes.len());
    // for ri in 0..y_dim {
    //     'x: for ci in 0..x_dim {
    //         for (freq, coords) in nodes_by_freq.iter() {
    //             for coord in coords {
    //                 if coord == &(ci, ri) {
    //                     print!("{freq}");
    //                     continue 'x;
    //                 }
    //             }
    //         }
    //         print!(".");
    //     }
    //     println!();
    // }

    // eprintln!("X: {x_dim}, Y: {y_dim}");
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
