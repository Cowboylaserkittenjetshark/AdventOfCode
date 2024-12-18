use clap::Parser;
use itertools::Itertools;
use std::{
    collections::VecDeque,
    fs,
    io::{BufRead, BufReader},
    iter,
    path::PathBuf,
    process::exit,
};

fn main() {
    let input = BufReader::new(fs::File::open(Args::parse().input).unwrap_or_else(|err| die(err)));
    if let Some(Ok(disk_map)) = input.lines().next() {
        let mut disk: Disk = Disk::with_capacity(disk_map.len());
        let mut id = 0;
        for mut chunk in &disk_map
            .chars()
            .map(|c| c.to_digit(10).unwrap_or_else(|| die("Invalid input file")) as usize)
            .chunks(2)
        {
            disk.push_back(Segment::file(
                id,
                chunk.next().unwrap_or_else(|| die("Invalid input file")),
            ));
            id += 1;

            if let Some(free_blocks) = chunk.next() {
                disk.push_back(Segment::free(free_blocks));
            }
        }

        unsafe {
            while let Some(i) = next_free(&disk) {
                let new_seg_id;
                loop {
                    let seg = &disk[disk.len() - 1];
                    if let Some(seg_id) = seg.id {
                        new_seg_id = seg_id;
                        break;
                    } else {
                        disk.pop_back();
                    }
                }

                let disk_len = disk.len();
                if i >= disk_len {
                    break;
                }
                let file_segment = &mut disk[disk_len - 1] as *mut Segment;
                let free_segment = &mut disk[i] as *mut Segment;
                let free_size = (*free_segment).size.clone();
                let file_size = (*file_segment).size.clone();

                (*free_segment).id = Some(new_seg_id);

                if free_size > file_size {
                    (*free_segment).size = file_size;
                    disk.insert(i + 1, Segment::free(free_size - file_size));
                    disk.pop_back();
                } else {
                    (*file_segment).size -= free_size;
                }
            }
        }

        let checksum: usize = disk
            .iter()
            .flat_map(|s| iter::repeat_n(s.id.unwrap(), s.size))
            .enumerate()
            .map(|(i, id)| i * id)
            .sum();
        println!("Checksum: {checksum}");
    } else {
        die("Invalid input file");
    }
}

type Disk = VecDeque<Segment>;

fn next_free(disk: &Disk) -> Option<usize> {
    for (i, segment) in disk.iter().enumerate() {
        if let None = segment.id {
            return Some(i);
        }
    }
    return None;
}

#[derive(Debug)]
struct Segment {
    id: Option<usize>,
    size: usize,
}

impl Segment {
    fn free(size: usize) -> Self {
        Self { id: None, size }
    }

    fn file(id: usize, size: usize) -> Self {
        Self { id: Some(id), size }
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
