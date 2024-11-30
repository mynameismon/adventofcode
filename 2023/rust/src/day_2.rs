use std::cmp;
use crate::utils;

#[derive(Debug)]
struct Game {
    game_no: u32,
    green : u32,
    red: u32,
    blue: u32,
}


fn calc_ball_sum(game_line: &mut str) -> Game {
    let mut game = Game {
        game_no: 0,
        green: 0,
        red: 0,
        blue: 0
    };

    let mut game_meta = game_line.split(": ").fuse();


    let first = game_meta.next();
    game.game_no = first.unwrap()
                        .split(" ")
                        .last()
                        .expect("No space found")
                        .parse::<u32>().expect("No integer found");

    let rolls = game_meta.next().unwrap().split("; ");
    for roll in rolls {
        let cubes = roll.split(", ");
        for cube in cubes {
            let mut tok = cube.split(" ").fuse();
            let cube_num = tok.next().unwrap().parse::<u32>().expect("Integer not found");
            let cube_colour = tok.next().unwrap();

            match cube_colour {
                "red" => game.red = cmp::max(cube_num, game.red),
                "green" => game.green = cmp::max(cube_num, game.green),
                "blue" => game.blue = cmp::max(cube_num, game.blue),
                _ => unimplemented!()
            }
        }
    }

    game
}

impl Game {
    fn is_valid_game (&self) -> bool {
        self.red <= 12 &&
        self.green <= 13 &&
        self.blue <= 14
    }
}

fn part_1 () -> u32 {
    let lines: Vec<String> = utils::read_lines("../inputs/d2_p1.txt");

    lines.into_iter()
         .map(|mut x| calc_ball_sum(&mut x))
         .filter(|x| x.is_valid_game())
         .map(|x| x.game_no)
         .sum()
}

fn part_2 () -> u32 {
        let lines: Vec<String> = utils::read_lines("../inputs/d2_p1.txt");

    lines.into_iter()
         .map(|mut x| calc_ball_sum(&mut x))
         .map(|x: Game| x.red * x.blue * x.green)
         .sum()
}

pub fn solve () {
    println!("Answer to part 1: {}", part_1());
    println!("Answer to part 2: {}", part_2());
}
