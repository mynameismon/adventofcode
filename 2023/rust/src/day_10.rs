use std::{char, collections::VecDeque};

use crate::utils::read_lines;

#[derive(Debug, PartialEq, Eq)]
enum Dir {
    Ground,
    Start ,
    VPipe ,
    HPipe ,
    NEBend,
    SEBend,
    SWBend,
    NWBend,
    Unknown
}

impl Dir {
    fn convert_char (ch: char) -> Dir{
        match ch {
            'S' => Dir::Start,
            '.' => Dir::Ground,
            '|' => Dir::VPipe,
            '-' => Dir::HPipe,
            'L' => Dir::NEBend,
            'F' => Dir::SEBend,
            '7' => Dir::SWBend,
            'J' => Dir::NWBend,
            _ => Dir::Unknown
        }
    }

    fn use_pipe (&self, position: Pos) -> Pos {
        match *self {
            Dir::Start => position,
            Dir::Ground => position,
            Dir::VPipe => Pos {
                row: position.row + 1,
                col: position.col
            },
            Dir::HPipe => Pos {
                row: position.row,
                col: position.col + 1
            },
            Dir::NEBend => {
                position.row += 1;
                position.col += 1;
                position
            },
            _ => position
        }
    }
}

struct Pos {
    row: usize,
    col: usize
}

fn part_1 () -> u32 {
    let lines: Vec<Vec<Dir>> = read_lines("../inputs/d10.txt").iter()
                                                              .map(|r| r.chars().map(Dir::convert_char).collect::<Vec<Dir>>())
                                                              .collect();

    let row: usize = lines.iter()
                          .position(|r| r.iter().position(|s| s == &Dir::Start).is_some())
                          .unwrap();

    let col: usize = lines[row].iter()
                               .position(|s| s == &Dir::Start)
                               .unwrap();

    let start: Pos = Pos {
        row,
        col
    };

    let remaining: VecDeque<Pos> = VecDeque::new();

    remaining.push_back(start);

    while !remaining.is_empty() {
        let current: Pos = remaining.pop_front().unwrap();
        match lines[current.row][current.col] {

            _ => {}
        }
    }
}

fn part_2 () -> u32 {
    unimplemented!("Yet to finish part 1");
}

pub fn solve () {
    println!("Answer to part 1: {}", part_1());
    println!("Answer to part 2: {}", part_2());
}
