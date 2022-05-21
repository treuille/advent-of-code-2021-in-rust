use aoc::parse_regex::parse_lines;
// use itertools::iproduct;
// use ndarray::prelude::*;
use regex::Regex;
use std::mem;

fn main() {
    println!("rotations: {:?}", ROTATIONS);

    // test the alignment algorithm with the first two scanners
    let scanners = read_input();
    let mut scanners = scanners.into_iter();
    let scanner1 = scanners.next().unwrap();
    let scanner2 = scanners.next().unwrap();
    let scanner3 = align(&scanner1, scanner2);
    println!("scanner3: {:?}", scanner3);
}

fn search_for_alignment(mut scanners: Vec<Scanner>) {
    let n_scanners = scanners.len();
    let mut unsolved = scanners.split_off(1); // we need to connect these
    let mut solved = Vec::new(); // we have checked these against all others
    let mut processing = scanners; // we need to check these
    println!("just starting");
    println!("solved: {}", solved.len());
    println!("processing: {}", processing.len());
    println!("unsolved: {}\n", unsolved.len());
    while let Some(scanner1) = processing.pop() {
        let mut still_unsolved = Vec::new();
        while let Some(scanner2) = unsolved.pop() {
            println!("Solving beacon with {} scanners.", scanner2.len());
            match align(&scanner1, scanner2) {
                Ok(scanner2) => processing.push(scanner2),
                Err(scanner2) => still_unsolved.push(scanner2),
            }

            println!("finished inter while");
            println!("solved: {}", solved.len());
            println!("processing: {}", processing.len());
            println!("unsolved: {}\n", unsolved.len());
        }
        mem::swap(&mut unsolved, &mut still_unsolved);
        solved.push(scanner1);
        assert_eq!(
            solved.len() + processing.len() + unsolved.len(),
            n_scanners,
            "Lost track of a beacon."
        );
        assert!(processing.len() > 0, "Nothing more to process.");

        println!("finished outer while");
        println!("solved: {}", solved.len());
        println!("processing: {}", processing.len());
        println!("unsolved: {}\n", unsolved.len());
        panic!("outer while loop");
    }

    // let mut unsolved2 = Vec::with_capacity(unsolved.len() - 1);
    // let candidates = unsolved.drain();
    // todo!("Move scanners unsolved -> solved");
    // while let Some(scanner2) = candidates.next() {
    // }
    // }
    // for (scanner1, scanner2) in iproduct!(solved.iter(), .enumerate()) {
    //     match align(scanner1, scanner2) {
    //         Ok(scanner2) => {
    //             solved.push(scanner2);
    //             break;
    //         },
    //         Err(scanner2)
    //     }
    //     tood!("What happens in this loop?")
    // }
    // }
    // println!("solved: {}", solved.len());
    // println!("unsolved: {}", unsolved.len());
    // println!("{:?}", scanners[0]);
}

/// Ok(scanner2) if they can be aligned, Err(scanner2) otherwise.
fn align(scanner1: &Scanner, scanner2: Scanner) -> Result<Scanner, Scanner> {
    Err(scanner2)
}

/// A set of
type Scanner = Vec<Beacon>;

type Beacon = (i64, i64, i64);

// 1=i -1=-i 2=j -2=-j 3=k -3=-k
type Rotation = (i8, i8, i8);

/// All right handed rotations.
const ROTATIONS: [Rotation; 24] = [
    (1, 2, 3),
    (-1, -2, 3),
    (-1, 2, -3),
    (1, -2, -3),
    (-1, 3, 2),
    (1, -3, 2),
    (1, 3, -2),
    (-1, -3, -2),
    (-2, 1, 3),
    (2, -1, 3),
    (2, 1, -3),
    (-2, -1, -3),
    (2, 3, 1),
    (-2, -3, 1),
    (-2, 3, -1),
    (2, -3, -1),
    (3, 1, 2),
    (-3, -1, 2),
    (-3, 1, -2),
    (3, -1, -2),
    (-3, 2, 1),
    (3, -2, 1),
    (3, 2, -1),
    (-3, -2, -1),
];

/// Rotates the point along the origin by the given amount
fn rotate(pt: &Beacon, rot: &Rotation) -> Beacon {
    let coord = |axis: i8| match axis {
        1 => pt.0,
        2 => pt.1,
        3 => pt.2,
        -1 => -pt.0,
        -2 => -pt.1,
        -3 => -pt.2,
        _ => unimplemented!("Axis must be a nonzero integer between -3 and 3"),
    };
    (coord(rot.0), coord(rot.1), coord(rot.2))
}

// fn align_with(scanner1: &Beacon, scanner2: &Beacon, tranform: F) where
// F: Fn(Pt) -> Pt {

// }

fn parse_beacon(s: &str) -> Scanner {
    let (_, s) = s.split_once("\n").unwrap();
    let re = Regex::new(r"(\-?\d+),(\-?\d+),(\-?\d+)").unwrap();
    parse_lines(&re, s).collect()
}

/// Read the input file and turn it into an Array2<u8>
fn read_input() -> Vec<Scanner> {
    let scanners = include_str!("../../puzzle_inputs/day_19_test.txt").split("\n\n");
    scanners.map(parse_beacon).collect()
}
