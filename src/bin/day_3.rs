// use std::ops::Deref;
use std::collections::HashMap;

fn main() {
    let (lines, n_columns) = load_input();
    println!("Puzzle 3a: {} (2003336)", solve_puzzle_a(&lines, n_columns));
    println!("Puzzle 3b: {} (1877139)", solve_puzzle_b(&lines, n_columns));
}

fn solve_puzzle_a(lines: &[String], n_columns: usize) -> usize {
    let (mut gamma, mut epsilon) = (0, 0);
    for i in 0..n_columns {
        if more_ones(lines, n_columns - 1 - i) {
            gamma |= 1 << i;
        } else {
            epsilon |= 1 << i;
        }
    }
    // println!("gamma={gamma}");
    // println!("epsilon={epsilon}");
    gamma * epsilon
}

fn solve_puzzle_b(lines: &[String], n_columns: usize) -> usize {
    // let more_zeros: Vec<bool> = more_ones.iter().map(|b| !b).collect();

    let oxy_map = HashMap::from([(false, '0'), (true, '1')]);
    let co2_map = HashMap::from([(false, '1'), (true, '0')]);
    let [oxygen, co2] = [oxy_map, co2_map].map(|map| {
        let mut prefix = String::new();
        let mut remaining_lines: Vec<&String> = lines.iter().collect();
        // println!("Initializing remaining lines.");
        // remaining_lines
        //     .iter()
        //     .enumerate()
        //     .for_each(|(i, line)| println!("{i:02} -> {line}"));
        for col_number in 0..n_columns {
            prefix.push(*map.get(&more_ones(&remaining_lines, col_number)).unwrap());
            // match  {
            //     true => prefix.push('1'),
            //     false => prefix.push('0'),
            // }
            // println!("prefix = {prefix}");
            remaining_lines = remaining_lines
                .into_iter()
                .filter(|line| line.starts_with(&prefix))
                .collect();
            // remaining_lines
            //     .iter()
            //     .enumerate()
            //     .for_each(|(i, line)| println!("{i:02} -> {line}"));
            if remaining_lines.len() == 1 {
                let binary_string = remaining_lines.first().unwrap();
                return usize::from_str_radix(binary_string, 2).unwrap();
            }
        }
        panic!("Unable to filter down to a single result.");
    });
    // println!("oxygen = {oxygen:?}");
    // println!("co2 = {co2:?}");
    oxygen * co2

    // scan(String::new(), |prefix: &mut String, col: usize| {
    //     let more_ones = more_ones( , col_number: usize)
    //     println!("{x:?} {i}");
    //     Some(())
    // })
    // .for_each(|x| println!("{x:?}"));

    // panic!("Stopping here.");

    // println!("more_ones: {more_ones:?}");
    // println!("more_zeros: {more_zeros:?}");
    // let [a, b] = [more_ones, more_zeros.deref()].map(|selector| {
    //     let mut prefix = String::from("");
    //     for b in selector {
    //         prefix.push(match b {
    //             false => '0',
    //             true => '1',
    //         });
    //         let matching_lines: Vec<&String> = lines
    //             .iter()
    //             .filter(|line| line.starts_with(&prefix))
    //             .collect();
    //         println!("{b} -> {prefix} ({})", matching_lines.len());
    //         if matching_lines.len() > 1 {
    //             continue;
    //         }
    //         let binary_string = matching_lines.into_iter().next().unwrap();
    //         let decimal = usize::from_str_radix(binary_string, 2).unwrap();
    //         println!("** {b} -> {prefix} -> {binary_string} -> {decimal}");
    //         let binary_string = String::from_iter(binary_string.chars().rev());
    //         let decimal = usize::from_str_radix(&binary_string, 2).unwrap();
    //         println!("** {b} -> {prefix} -> {binary_string} -> {decimal}");
    //         return decimal;
    //     }
    //     panic!("Couldn't find a matching string.");
    //     // while remaining_lines.len() > 1 {

    //     // }
    // });
    // println!("a={a} b={b}");
    // a * b
}

/// Loads the input and a the number of  characters in each column.
fn load_input() -> (Vec<String>, usize) {
    let lines: Vec<String> = include_str!("../../puzzle_inputs/day_3.txt")
        // let lines: Vec<String> = include_str!("../../puzzle_inputs/day_3_test.txt")
        .lines()
        .map(String::from)
        .collect();
    let n_columns = lines[0].len();
    (lines, n_columns)
}

// Returns true if there are more 1s than 0s in the specified column.
fn more_ones<S>(lines: &[S], col_number: usize) -> bool
where
    S: AsRef<str>,
{
    let n_ones = lines
        .iter()
        .filter(|s| s.as_ref().chars().nth(col_number) == Some('1'))
        .count();
    n_ones * 2 >= lines.len()
}

// // How many ones there are in each column.
// let ones: Vec<usize> = input.iter().fold(
//     vec![0; n_columns],
//     // iter::repeat(0).take().collect(),
//     |ones: Vec<usize>, line: &String| {
//         ones.iter()
//             .zip(line.chars())
//             .map(|(count, c)| match c {
//                 '0' => *count,
//                 '1' => *count + 1,
//                 _ => panic!("Character must be a 0 or 1."),
//             })
//             .collect()
//     },
// );

// println!("ones: {ones:?}");

// // Whether there are more ones in each column.
// let more_ones: Vec<bool> = ones.iter().map(|ones| ones * 2 >= input.len()).collect();
// (input, more_ones)
// }
