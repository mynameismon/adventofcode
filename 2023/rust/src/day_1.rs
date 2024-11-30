use crate::utils;

fn parse_integers (input_line: &String) -> Vec<u32> {
    let replacements: [(&str, &str); 10] = [("zero", "zero0zero"),
                                            ("one", "one1one"),
                                            ("two", "two2two"),
                                            ("three", "three3three"),
                                            ("four", "four4four"),
                                            ("five", "five5five"),
                                            ("six", "six6six"),
                                            ("seven", "seven7seven"),
                                            ("eight", "eight8eight"),
                                            ("nine", "nine9nine")];

    let mut replaced: String = input_line.to_string();

    for r in replacements {
        replaced = replaced.replace(r.0, r.1);
    }

    let digits: Vec<u32> = replaced.chars()
                                   .filter(|x| x.is_ascii_digit())
                                   .map(|x| x.to_digit(10).unwrap())
                                   .collect();
    print!("Input Line: {:?}\t", input_line);
    println!("Digits: {:?}", digits);
    digits
}

// #[flux_sig(fn () -> u32)]
pub fn part_1 () -> u32 {
    let lines: Vec<String> = utils::read_lines("../inputs/d1_p1.txt");
    let result: u32 = lines.clone()
                              .into_iter()
                              .map(|x| {
                                  let v : Vec<u32> = x.matches(char::is_numeric).map(|x| x.parse::<u32>().unwrap()).collect();
                                  v.get(0).unwrap() * 10 + v.last().unwrap()
                              })
                              .sum();
    result
}

pub fn part_1_idiomatic () -> u32 {
    let lines: Vec<String> = utils::read_lines("../inputs/d1_p1.txt");

    let mut result: u32 = 0;
    for line in lines {
        let digits: Vec<u32> = line.chars()
                                   .filter(|x| x.is_ascii_digit())
                                   .map(|x| x.to_digit(10).unwrap())
                                   .collect();
        result += digits.get(0).unwrap() * 10 + digits.last().unwrap();
    }

    result
}

pub fn part_2 () -> u32 {
    let lines: Vec<String> = utils::read_lines("../inputs/d1_p1.txt");

    let result: u32 = lines.into_iter()
                           .map(|line| parse_integers(&line))
                           .map(|digits| digits.first().unwrap() * 10 + digits.last().unwrap())
                           .sum::<u32>();
    result
}

pub fn solve () {
    println!("Answer to Part 1: {}", part_1());
    println!("Answer to Part 2: {}", part_2());
}
