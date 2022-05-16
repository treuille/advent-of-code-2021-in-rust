use aoc::parse_regex::parse_lines;
use itertools::iproduct;
use ndarray::prelude::*;
use regex::Regex;

fn main() {
    let mut beacons = read_input();
    let mut unsolved = beacons.split_off(1);
    let mut solved = beacons;

    while !unsolved.is_empty() {
        // let mut unsolved2 = Vec::with_capacity(unsolved.len() - 1);
        // let candidates = unsolved.drain();
        todo!("Move beacons unsolved -> solved");
        // while let Some(beacon2) = candidates.next() {
        // }
        // }
        // for (beacon1, beacon2) in iproduct!(solved.iter(), .enumerate()) {
        //     match align(beacon1, beacon2) {
        //         Ok(beacon2) => {
        //             solved.push(beacon2);
        //             break;
        //         },
        //         Err(beacon2)
        //     }
        //     tood!("What happens in this loop?")
        // }
    }
    println!("solved: {}", solved.len());
    println!("unsolved: {}", unsolved.len());
}

type Beacon = Vec<Pt>;

type Pt = Array1<i64>;

// fn solve_XXa() -> usize {
//     123
// }

// fn solve_XXb() -> usize {
//     456
// }

/// Ok(beacon2) if they can be aligned, Err(beacon2) otherwise.
fn align(beacon1: &Beacon, beacon2: Beacon) -> Result<Beacon, Beacon> {
    todo!("align")
}

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
