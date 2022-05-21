use aoc::parse_regex::parse_lines;
// use itertools::iproduct;
// use ndarray::prelude::*;
use regex::Regex;
use std::mem;

fn main() {
    println!("rotations: {:?}", ROTATIONS);

    // test the alignment algorithm with the first two beacons
    let beacons = read_input();
    let mut beacons = beacons.into_iter();
    let beacon1 = beacons.next().unwrap();
    let beacon2 = beacons.next().unwrap();
    let beacon3 = align(&beacon1, beacon2);
    println!("beacon3: {:?}", beacon3);
}

fn search_for_alignment(mut beacons: Vec<Beacon>) {
    let n_beacons = beacons.len();
    let mut unsolved = beacons.split_off(1); // we need to connect these
    let mut solved = Vec::new(); // we have checked these against all others
    let mut processing = beacons; // we need to check these
    println!("just starting");
    println!("solved: {}", solved.len());
    println!("processing: {}", processing.len());
    println!("unsolved: {}\n", unsolved.len());
    while let Some(beacon1) = processing.pop() {
        let mut still_unsolved = Vec::new();
        while let Some(beacon2) = unsolved.pop() {
            println!("Solving beacon with {} scanners.", beacon2.len());
            match align(&beacon1, beacon2) {
                Ok(beacon2) => processing.push(beacon2),
                Err(beacon2) => still_unsolved.push(beacon2),
            }

            println!("finished inter while");
            println!("solved: {}", solved.len());
            println!("processing: {}", processing.len());
            println!("unsolved: {}\n", unsolved.len());
        }
        mem::swap(&mut unsolved, &mut still_unsolved);
        solved.push(beacon1);
        assert_eq!(
            solved.len() + processing.len() + unsolved.len(),
            n_beacons,
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
    // todo!("Move beacons unsolved -> solved");
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
    // }
    // println!("solved: {}", solved.len());
    // println!("unsolved: {}", unsolved.len());
    // println!("{:?}", beacons[0]);
}

/// Ok(beacon2) if they can be aligned, Err(beacon2) otherwise.
fn align(beacon1: &Beacon, beacon2: Beacon) -> Result<Beacon, Beacon> {
    Err(beacon2)
}
type Beacon = Vec<Pt>;

type Pt = (i64, i64, i64);

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
fn rotate(pt: &Pt, rot: &Rotation) -> Pt {
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

// fn align_with(beacon1: &Beacon, beacon2: &Beacon, tranform: F) where
// F: Fn(Pt) -> Pt {

// }

fn parse_beacon(s: &str) -> Beacon {
    let (_, s) = s.split_once("\n").unwrap();
    let re = Regex::new(r"(\-?\d+),(\-?\d+),(\-?\d+)").unwrap();
    parse_lines(&re, s).collect()
}

/// Read the input file and turn it into an Array2<u8>
fn read_input() -> Vec<Beacon> {
    let beacons = include_str!("../../puzzle_inputs/day_19_test.txt").split("\n\n");
    beacons.map(parse_beacon).collect()
}
