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

pub fn part_2 () {
    unimplemented!();
}
