use crate::utils::read_lines;

#[derive(Debug)]
struct Engine {
    engines: String,
    operational: Vec<u32>
}

fn parse_line (line: String) -> Engine {
    let mut groups = line.split(" ");

    Engine {
        engines: groups.next()
                       .unwrap()
                       .to_string(),
        operational: groups.next()
                           .unwrap()
                           .split(",")
                           .map(|x| x.parse::<u32>().unwrap())
                           .collect()
    }
}

fn calculate_combinations (e: Engine) -> u64 {
    let engines = format!["{}.", e.engines];
    let engine_ch: Vec<char> = engines.chars().collect::<Vec<char>>();

    let mut operational = e.operational;
    operational.push(engine_ch.len().try_into().unwrap());

    let mut dp: Vec<Vec<Vec<u64>>> = vec![vec![vec![0; engines.len() + 1]; operational.len() + 1]; engines.len() + 1];

    // The DP solution
    // https://old.reddit.com/r/adventofcode/comments/18ge41g/2023_day_12_solutions/kd04bjg/
    dp[0][0][0] = 1;

    // println!();

    for i in 0..engines.len() {
        for j in 0..operational.len() {
            for k in 0..engines.len() {
                if dp[i][j][k] == 0 {
                    continue;
                }

                // println!("Position = {}, # of Groups = {}, Length of last group = {}; Ans = {}", i, j, k, dp[i][j][k]);


                match engine_ch[i] {
                    '.' => {
                        if k == 0 || k == operational[j - 1].try_into().unwrap() {
                            dp[i + 1][j][0] += dp[i][j][k];
                        }
                    }
                    '#' => {
                        if k == 0 {
                            dp[i + 1][j + 1][k + 1] += dp[i][j][k];
                        } else {
                            dp[i + 1][j][k + 1] += dp[i][j][k];
                        }
                    }
                    '?' => {
                        if k == 0 || k == operational[j - 1].try_into().unwrap() {
                            dp[i + 1][j][0] += dp[i][j][k];
                        }

                        if k == 0 {
                            dp[i + 1][j + 1][k + 1] += dp[i][j][k];
                        } else {
                            dp[i + 1][j][k + 1] += dp[i][j][k];
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    dp[engines.len()][operational.len() - 1][0]
}

fn part_1 () -> u64 {
    let lines: Vec<String> = read_lines("../inputs/d12.txt");

    lines.into_iter()
         .map(|x| parse_line(x))
         .map(|x| calculate_combinations(x))
         .sum::<u64>()
}

fn part_2 () -> u64 {
    let lines: Vec<String> = read_lines("../inputs/d12.txt");

    lines.into_iter()
         .map(|x| parse_line(x))
         .map(|mut x| {
             for _ in [0..4] {
                 x.engines.push('?');
                 x.engines = x.engines.repeat(5);
                 x.engines.pop();
                 x.operational = x.operational.repeat(5);
             }
             x
         })
         .map(|x| calculate_combinations(x))
         .sum::<u64>()
}

pub fn solve () {
    println!("Answer to part 1: {}", part_1());
    println!("Answer to part 2: {}", part_2());
}
