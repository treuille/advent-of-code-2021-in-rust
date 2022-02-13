fn main() {
    let (lines, more_ones) = process_input();
    println!("Puzzle 3a: {:?} 2003336", solve_puzzle_a(&more_ones));
    println!("Puzzle 3b: {:?}", solve_puzzle_b(&lines, &more_ones));
}

fn solve_puzzle_a(more_ones: &[bool]) -> usize {
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

fn solve_puzzle_b(_lines: &[String], _more_ones: &[bool]) -> usize {
    0
}

fn process_input() -> (Vec<String>, Vec<bool>) {
    // List of all the lines in the input file.
    let lines: Vec<String> = include_str!("../../puzzle_inputs/day_3.txt")
        .lines()
        .map(String::from)
        .collect();
    let n_columns = lines[0].len();

    // How many ones there are in each column.
    let ones: Vec<usize> = lines.iter().fold(
        vec![0; n_columns],
        // iter::repeat(0).take().collect(),
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

    // Whether there are more ones in each column.
    let more_ones: Vec<bool> = ones.iter().map(|ones| ones * 2 >= lines.len()).collect();
    (lines, more_ones)
}
