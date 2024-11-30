use crate::utils::read_lines;

fn transpose<T: std::fmt::Debug> (matrix: Vec<Vec<T>>) -> Vec<Vec<T>> {
    assert!(!matrix.is_empty());

    let len = matrix[0].len();
    let mut iters: Vec<_> = matrix.into_iter()
                                  .map(|x| x.into_iter())
                                  .collect();

    (0..len).map(|_| {
        iters.iter_mut()
             .map(|n| n.next().unwrap())
             .collect::<Vec<T>>()
    }).collect::<Vec<Vec<T>>>()
}

fn find_horizontal (block: &Vec<Vec<char>>) -> u32 {
    let len = block.len();

    let mut result = (0..len - 1).filter(|&i| {
        for delta in 0..=i {
            if (i + delta + 1) < len {
                if block[i - delta] != block[i + delta + 1] {
                    return false
                }
            }
        }
        true
    });

    match result.next() {
        Some(x) => (x + 1).try_into().unwrap(),
        None => 0
    }
}

fn find_horizontal_smudged (block: &Vec<Vec<char>>) -> u32 {
    let len = block.len();

    let mut result = (0..len - 1).filter(|&i| {
        let mut diff_count = 0;
        for delta in 0..=i {
            if (i + delta + 1) < len {
                diff_count += block[i - delta].iter()
                                              .zip(block[i + delta + 1].iter())
                                              .filter(|(&x, &y)| x != y)
                                              .count();

                if diff_count > 1 {
                    return false
                }
            }
        }
        diff_count == 1
    });

    match result.next() {
        Some(x) => (x + 1).try_into().unwrap(),
        None => 0
    }
}

fn part_1 () -> u32 {
    let lines: Vec<String> = read_lines("../inputs/d13.txt");

    let blocks = lines.split(|x| x.len() < 2);

    blocks.map(|x| x.into_iter().map(|x| x.chars().collect::<Vec<_>>()).collect())
          .map(|x| 100 * find_horizontal(&x) + find_horizontal(&transpose(x)))
          .sum::<u32>()
}

fn part_2 () -> u32 {
    let lines: Vec<String> = read_lines("../inputs/d13.txt");

    let blocks = lines.split(|x| x.len() < 2);

    blocks.map(|x| x.into_iter().map(|x| x.chars().collect::<Vec<_>>()).collect())
          .map(|x| 100 * find_horizontal_smudged(&x) + find_horizontal_smudged(&transpose(x)))
          .sum::<u32>()
}

pub fn solve () {
    println!("Answer to part 1: {}", part_1());
    println!("Answer to part 2: {}", part_2());
}
