// use std::error::Error;
use std::num::ParseIntError;
use std::str::FromStr;

fn main() {
    println!("Welcome to day 4");
    load_input();
    // let (lines, n_columns) = load_input();
    // println!("Puzzle 3a: {} (2003336)", solve_puzzle_a(&lines, n_columns));
    // println!("Puzzle 3b: {} (1877139)", solve_puzzle_b(&lines, n_columns));
}

/// Loads the input and a the number of  characters in each column.
fn load_input() {
    let input = include_str!("../../puzzle_inputs/day_4.txt");
    let elts: Vec<_> = input.split_terminator("\n\n").collect();
    let (guesses, boards) = elts.split_first().unwrap();
    println!("guesses: {:?}", guesses);
    let guesses: Vec<usize> = guesses.split(",").map(|s| s.parse().unwrap()).collect();
    println!("guesses: {:?}", guesses);
    let boards: Vec<Board> = boards.iter().map(|s| s.parse().unwrap()).collect();
    println!("first:\n{:?}", boards.first());
    println!("last:\n{:?}", boards.last());
    // for x in elts.iter().take(3).chain(elts.last()) {
    //     println!("###");
    //     println!("{}", x);
    //     println!("!!!");
    // }
}

#[derive(Debug)]
struct Board {
    w: usize,
    h: usize,
    elts: Vec<Vec<usize>>,
}

impl FromStr for Board {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        println!("parsing: {}", s);

        let elts: Vec<Vec<usize>> = s
            .lines()
            .map(|line| {
                line.split_ascii_whitespace()
                    .map(|s| s.parse())
                    .collect::<Result<_, _>>()
            })
            .collect::<Result<_, _>>()?;

        Ok(Self {
            w: elts[0].len(),
            h: elts.len(),
            elts,
        })
    }
}
