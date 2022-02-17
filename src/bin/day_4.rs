// use std::collections::HashMap;

fn main() {
    println!("Welcome to day 4");
    load_input();
    // let (lines, n_columns) = load_input();
    // println!("Puzzle 3a: {} (2003336)", solve_puzzle_a(&lines, n_columns));
    // println!("Puzzle 3b: {} (1877139)", solve_puzzle_b(&lines, n_columns));
}

/// Loads the input and a the number of  characters in each column.
fn load_input() -> () {
    let input = include_str!("../../puzzle_inputs/day_4.txt");
    let elts: Vec<_> = input.split("\n\n").collect();
    for x in elts.iter().take(3).chain(elts.last()) {
        println!("###");
        println!("{}", x);
        println!("!!!");
    }
}
