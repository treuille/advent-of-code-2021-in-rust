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
    let ones: Vec<usize> = lines.iter().fold(
        iter::repeat(0).take(lines[0].len()).collect(),
        |ones: Vec<usize>, line: &String| {
            ones.iter()
                .zip(line.chars())
                .map(|(count, c)| match c {
                    '0' => *count,
                    '1' => *count + 1,
                    _ => panic!("Character must be a 0 or 1."),
                })
                .collect()
        },
    );
    let more_ones: Vec<bool> = ones.iter().map(|ones| ones * 2 >= lines.len()).collect();
    (lines, more_ones)
}
