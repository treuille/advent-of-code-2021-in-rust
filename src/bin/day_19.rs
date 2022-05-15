use aoc::parse_regex::parse_lines;
use ndarray::prelude::*;
use regex::Regex;

fn main() {
    for (i, beacon) in read_input().into_iter().enumerate() {
        println!("beacon {i}:\n{beacon:?}");
    }
}

type Beacon = Vec<Pt>;

type Pt = Array1<i64>;

// fn solve_XXa() -> usize {
//     123
// }

// fn solve_XXb() -> usize {
//     456
// }

fn parse_beacon(s: &str) -> Beacon {
    let s = s.split_once("\n").unwrap().1;
    let re = Regex::new(r"(\-?\d+),(\-?\d+),(\-?\d+)").unwrap();
    parse_lines(&re, s)
        .map(|(x, y, z)| [x, y, z].into_iter().collect())
        .collect()
}

/// Read the input file and turn it into an Array2<u8>
fn read_input() -> Vec<Beacon> {
    let beacons = include_str!("../../puzzle_inputs/day_19_test.txt").split("\n\n");
    beacons.map(parse_beacon).collect()
}
