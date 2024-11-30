use std::fmt;

use crate::utils;

#[derive(Debug)]
struct Coord {
    x: u32,
    y: u32,
    z: u32
}

impl fmt::Display for Coord {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
	write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}


#[derive(Debug)]
struct Brick {
    start: Coord,
    end: Coord
}

impl fmt::Display for Brick {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
	write!(f, "{} -> {}", self.start, self.end)
    }
}

fn parse_coord (coords: &str) -> Coord {
    match coords.split(",").collect::<Vec<_>>().as_slice() {
	[x, y, z] => Coord {
	    x: x.parse::<u32>().unwrap(),
	    y: y.parse::<u32>().unwrap(),
	    z: z.parse::<u32>().unwrap(),
	},
	_ => panic!()
    }
}

fn parse_line (line: String) -> Brick {
    let (min, max) = line.split_once("~").unwrap();
    
    Brick {
	start: parse_coord(min),
	end: parse_coord(max)
    }
}

fn part_1() -> u32 {
    let lines: Vec<String> = utils::read_lines("../inputs/d22.txt");

    let mut bricks: Vec<Brick> = lines.into_iter()
	.map(|x| parse_line(x))
        .collect::<Vec<Brick>>();

    bricks.sort_by_key(|x| x.start.z);

    for brick in bricks.iter() {
	println!("{}", brick)
    }
    
    unimplemented!();
}

fn part_2() -> u32 {
    let lines: Vec<String> = utils::read_lines("../inputs/d22.txt");

    unimplemented!();
}

pub fn solve () {
    println!("Answer to part 1: {}", part_1());
    println!("Answer to part 2: {}", part_2());
}
