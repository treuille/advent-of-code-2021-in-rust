use std::iter;

fn main() {
    println!("Puzzle 3a: {:?} 2003336", solve_puzzle());
    // println!("Puzzle 2b: {:?}", solve_puzzle(advance_b));
}

fn solve_puzzle() -> usize {
    let (_, more_ones) = process_input();
    let (mut gamma, mut epsilon) = (0, 0);
    for (i, more_ones) in more_ones.iter().rev().enumerate() {
        if *more_ones {
            gamma |= 1 << i;
        } else {
            epsilon |= 1 << i;
        }
    }
    gamma * epsilon
}

fn process_input() -> (Vec<String>, Vec<bool>) {
    let lines: Vec<String> = include_str!("../../puzzle_inputs/day_3.txt")
        .lines()
        .map(String::from)
        .collect();
    let count_and_ones: Vec<usize> = lines
        .iter()
        .map(|line| format!("1{line}"))
        .fold(None, |ones: Option<Vec<usize>>, line| {
            let all_zeros = iter::repeat(0).take(line.len());
            let ones = ones
                .unwrap_or_else(|| all_zeros.collect())
                .iter()
                .zip(line.chars())
                .map(|(count, c)| match c {
                    '0' => *count,
                    '1' => *count + 1,
                    _ => panic!("Character must be a 0 or 1."),
                })
                .collect();
            Some(ones)
        })
        .expect("File has zero lines.");
    let (count, ones) = (count_and_ones[0], &count_and_ones[1..]);
    let more_ones: Vec<bool> = ones.iter().map(|ones| ones * 2 >= count).collect();
    (lines, more_ones)
}
