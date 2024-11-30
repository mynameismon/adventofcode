use std::fs::read_to_string;
use std::env::current_dir;

fn read_lines (filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

pub fn part_1 () {
    println!("Lines read: {:?}", read_lines("../inputs/d1_p1.txt"));
}

pub fn part_2 () {
    unimplemented!();
}
