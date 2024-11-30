use std::fs::read_to_string;

fn read_lines (filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

pub fn part_1 () {
    let lines: Vec<String> = read_lines("../inputs/d1_p1.txt");
    let result: u32 = lines.clone()
                              .into_iter()
                              .map(|x| {
                                  let v : Vec<u32> = x.matches(char::is_numeric).map(|x| x.parse::<u32>().unwrap()).collect();
                                  v.get(0).unwrap() * 10 + v.last().unwrap()
                              })
                              .sum();

    println!("Answer to Part 1: {}", result);
}

pub fn part_1_idiomatic () {
    let lines: Vec<String> = read_lines("../inputs/d1_p1.txt");

    let mut result: u32 = 0;
    for line in lines {
        let digits: Vec<u32> = line.chars()
                                   .filter(|x| x.is_ascii_digit())
                                   .map(|x| x.to_digit(10).unwrap())
                                   .collect();
        result += digits.get(0).unwrap() * 10 + digits.last().unwrap();
    }

    println!("Answer to Part 1: {}", result);
}

pub fn part_2 () {
    unimplemented!();
}
