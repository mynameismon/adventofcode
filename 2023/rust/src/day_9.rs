use crate::utils::read_lines;

fn find_extrapolation(nums: &Vec<i32>) -> i32 {
    let nums_shifted = &nums[1..];
    let nums_rest = nums.split_last().unwrap().1;
    let diff_array: Vec<i32> = nums_shifted
        .iter()
        .zip(nums_rest)
        .map(|(x, y)| x - y)
        .collect();
    let zeroes: Vec<i32> = diff_array.clone().into_iter().filter(|x| *x != 0).collect();
    if zeroes.is_empty() {
        nums.last().unwrap() + 0
    } else {
        let rec_call: i32 = find_extrapolation(&diff_array);
        nums.last().unwrap() + rec_call
    }
}

fn part_1() -> i32 {
    let lines: Vec<String> = read_lines("../inputs/d9.txt");

    lines
        .into_iter()
        .map(|x| {
            x.split_whitespace()
                .map(|y| y.parse::<i32>().unwrap())
                .collect()
        })
        .map(|x| find_extrapolation(&x))
        .sum::<i32>()
}

fn part_2() -> i32 {
    let lines: Vec<String> = read_lines("../inputs/d9.txt");

    lines
        .into_iter()
        .map(|x| {
            x.split_whitespace()
                .map(|y| y.parse::<i32>().unwrap())
                .rev()
                .collect()
        })
        .map(|x| find_extrapolation(&x))
        .sum::<i32>()
}

pub fn solve() {
    println!("Answer to part 1: {}", part_1());
    println!("Answer to part 2: {}", part_2());
}
