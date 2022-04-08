use std::collections::HashMap;

fn main() {
    let (lines, n_columns) = load_input();
    println!("Puzzle 3a: {} (2003336)", solve_puzzle_a(&lines, n_columns));
    println!("Puzzle 3b: {} (1877139)", solve_puzzle_b(&lines, n_columns));
}

fn solve_puzzle_a(lines: &[String], n_columns: usize) -> usize {
    let mut gamma = 0;
    let mut epsilon = 0;
    for i in 0..n_columns {
        match more_ones(lines, n_columns - 1 - i) {
            true => gamma |= 1 << i,
            false => epsilon |= 1 << i,
        }
    }
    gamma * epsilon
}

fn solve_puzzle_b(lines: &[String], n_columns: usize) -> usize {
    let oxy_map = HashMap::from([(false, '0'), (true, '1')]);
    let co2_map = HashMap::from([(false, '1'), (true, '0')]);
    let [oxygen, co2] = [oxy_map, co2_map].map(|map| {
        let mut prefix = String::new();
        let mut remaining_lines: Vec<&String> = lines.iter().collect();
        for col_number in 0..n_columns {
            prefix.push(*map.get(&more_ones(&remaining_lines, col_number)).unwrap());
            remaining_lines = remaining_lines
                .into_iter()
                .filter(|line| line.starts_with(&prefix))
                .collect();
            if remaining_lines.len() == 1 {
                let binary_string = remaining_lines.first().unwrap();
                return usize::from_str_radix(binary_string, 2).unwrap();
            }
        }
        panic!("Unable to filter down to a single result.");
    });
    oxygen * co2
}

/// Loads the input and a the number of  characters in each column.
fn load_input() -> (Vec<String>, usize) {
    let lines: Vec<String> = include_str!("../../puzzle_inputs/day_03.txt")
        .lines()
        .map(String::from)
        .collect();
    let n_columns = lines[0].len();
    (lines, n_columns)
}

/// Returns true if there are more 1s than 0s in the specified column.
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
