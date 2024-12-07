use clap::Parser;
use itertools::Itertools;
use std::{
    collections::HashMap,
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
    let mut lines = input.lines().map(|res| res.unwrap_or_else(|e| die(e)));
    let mut map: HashMap<usize, Vec<usize>> = HashMap::new();

    while let Some(line) = lines.next() {
        if line == "" {
            break;
        }
        let (pre, post): (usize, usize) = line
            .split('|')
            .map(|s| s.parse().unwrap_or_else(|e| die(e)))
            .collect_tuple()
            .unwrap_or_else(|| die("Invalid input"));
        map.entry(pre)
            .and_modify(|posts| posts.push(post))
            .or_insert(vec![post]);
    }

    let mut middle_sum = 0;

    'line: for line in lines {
        let pages = line
            .trim()
            .split(',')
            .map(|s| s.parse::<usize>().unwrap_or_else(|e| die(e)))
            .collect_vec();
        for i in 0..pages.len() {
            let pre = pages[i];
            if let Some(posts) = map.get(&pre) {
                for post in posts {
                    if let Some(j) = pages.iter().position(|p| p == post) {
                        if i > j {
                            continue 'line;
                        }
                    }
                }
            }
        }
        middle_sum += pages[pages.len() / 2]
    }
    println!("Already sorted: {middle_sum}");
}

// Pain :|
// fn part_2() {
//     let input = BufReader::new(File::open(Args::parse().input).unwrap_or_else(|err| die(err)));
//     let mut lines = input.lines().map(|res| res.unwrap_or_else(|e| die(e)));
//     let mut order: VecDeque<usize> = VecDeque::new();

//     while let Some(line) = lines.next() {
//         if line == "" {
//             break;
//         }
//         let (pre, post): (usize, usize) = line
//             .split('|')
//             .map(|s| s.parse().unwrap_or_else(|e| die(e)))
//             .collect_tuple()
//             .unwrap_or_else(|| die("Invalid input"));
//         let pre_pos = order.iter().position(|elem| *elem == pre);
//         let post_pos = order.iter().rposition(|elem| *elem == post);
//         if let Some(i) = pre_pos {
//             if let Some(j) = post_pos {
//                 if i > j {
//                     order.swap(i, j);
//                 }
//             } else {
//                 order.push_back(post);
//             }
//         } else {
//             order.push_front(pre);
//             if post_pos.is_none() {
//                 order.push_back(post);
//             }
//         }
//     }
//     let mut map = HashMap::with_capacity(order.len());
//     for (i, elem) in order.iter().enumerate() {
//         map.insert(elem, i);
//     }

//     let mut middle_sum = 0;

//     for line in lines {
//         let mut pages = line
//             .trim()
//             .split(',')
//             .map(|s| s.parse::<usize>().unwrap_or_else(|e| die(e)))
//             .collect_vec();
//         if pages.is_sorted_by(|a, b| matches!(cstcmp(a, b, &map), Ordering::Less)) {
//             // pages.sort_by(|a, b| cstcmp(a, b, &map));
//             middle_sum += pages[pages.len() / 2]
//         }
//     }
//     println!("{middle_sum}");
// }

// fn cstcmp(a: &usize, b: &usize, order: &HashMap<&usize, usize, RandomState>) -> Ordering {
//     order.get(a).unwrap_or(&0).cmp(order.get(b).unwrap_or(&0))
// }

fn part_2() {
    let input = BufReader::new(File::open(Args::parse().input).unwrap_or_else(|err| die(err)));
    let mut lines = input.lines().map(|res| res.unwrap_or_else(|e| die(e)));
    let mut map: HashMap<usize, Vec<usize>> = HashMap::new();

    while let Some(line) = lines.next() {
        if line == "" {
            break;
        }
        let (pre, post): (usize, usize) = line
            .split('|')
            .map(|s| s.parse().unwrap_or_else(|e| die(e)))
            .collect_tuple()
            .unwrap_or_else(|| die("Invalid input"));
        map.entry(pre)
            .and_modify(|posts| posts.push(post))
            .or_insert(vec![post]);
    }

    let mut middle_sum = 0;

    for line in lines {
        let mut pages = line
            .trim()
            .split(',')
            .map(|s| s.parse::<usize>().unwrap_or_else(|e| die(e)))
            .collect_vec();
        let mut sorting_needed = false;
        let mut i = 0;
        'sort: while i < pages.len() {
            if let Some(posts) = map.get(&pages[i]) {
                for post in posts {
                    let pre_pos = i;
                    if let Some(post_pos) = pages.iter().position(|elem| elem == post) {
                        if pre_pos > post_pos {
                            let pre_elem = pages.remove(pre_pos);
                            pages.insert(post_pos, pre_elem);
                            sorting_needed = true;
                            i = 0;
                            continue 'sort;
                        }
                    }
                }
            }
            i += 1;
        }
        if sorting_needed {
            middle_sum += pages[pages.len() / 2]
        }
    }
    println!("Sorted: {middle_sum}");
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
