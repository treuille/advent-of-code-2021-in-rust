use std::ops::Deref;

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

fn solve_puzzle_b(lines: &[String], more_ones: &[bool]) -> usize {
    let more_zeros: Vec<bool> = more_ones.iter().map(|b| !b).collect();
    println!("more_ones: {more_ones:?}");
    println!("more_zeros: {more_zeros:?}");
    let [a, b] = [more_ones, more_zeros.deref()].map(|selector| {
        let mut prefix = String::from("");
        for b in selector {
            prefix.push(match b {
                false => '0',
                true => '1',
            });
            let matching_lines: Vec<&String> = lines
                .iter()
                .filter(|line| line.starts_with(&prefix))
                .collect();
            println!("{b} -> {prefix} ({})", matching_lines.len());
            if matching_lines.len() > 1 {
                continue;
            }
            let binary_string = matching_lines.into_iter().next().unwrap();
            let decimal = usize::from_str_radix(binary_string, 2).unwrap();
            println!("** {b} -> {prefix} -> {binary_string} -> {decimal}");
            let binary_string = String::from_iter(binary_string.chars().rev());
            let decimal = usize::from_str_radix(&binary_string, 2).unwrap();
            println!("** {b} -> {prefix} -> {binary_string} -> {decimal}");
            return decimal;
        }
        panic!("Couldn't find a matching string.");
        // while remaining_lines.len() > 1 {

        // }
    });
    println!("a={a} b={b}");
    a * b
}

fn process_input() -> (Vec<String>, Vec<bool>) {
    // // Test input - consider dedenting with indoc
    // let test_input = r#"
    //     00100
    //     11110
    //     10110
    //     10111
    //     10101
    //     01111
    //     00111
    //     11100
    //     10000
    //     11001
    //     00010
    //     01010"#;
    // println!("'''{test_input}'''");
    // panic!("Testing test input.");
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

    println!("ones: {ones:?}");

    // Whether there are more ones in each column.
    let more_ones: Vec<bool> = ones.iter().map(|ones| ones * 2 >= lines.len()).collect();
    (lines, more_ones)
}
