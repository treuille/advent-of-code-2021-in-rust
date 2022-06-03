use aoc::parse_regex::parse_lines;
use itertools::{iproduct, Itertools};
use regex::Regex;
use std::collections::HashMap;

fn main() {
    let input = include_str!("../../puzzle_inputs/day_19.txt");
    let scanners = align_all(parse_input(input));

    println!("19a: {} (362)", solve_19a(&scanners));
    println!("19b: {} (12204)", solve_19b(&scanners));
}

fn solve_19a(scanners: &[Scanner]) -> usize {
    let marker_beacons = scanners.len();
    let all_beacons = scanners.iter().flatten().unique().count();
    all_beacons - marker_beacons
}

fn solve_19b(scanners: &[Scanner]) -> i64 {
    let marker_beacons: Vec<Beacon> = scanners.iter().map(|scanner| scanner[0]).collect();
    let manhattan_dist = |(beacon1, beacon2): (&Beacon, &Beacon)| -> i64 {
        let t = subtract(beacon1, beacon2);
        t.0.abs() + t.1.abs() + t.2.abs()
    };
    let marker_pairs = iproduct!(marker_beacons.iter(), marker_beacons.iter());
    marker_pairs.map(manhattan_dist).max().unwrap()
}

/// Align all the scanners to scanner[0], returning the result.
fn align_all(mut scanners: Vec<Scanner>) -> Vec<Scanner> {
    let mut solved = Vec::new(); // we have checked these against all others
    let mut unsolved = scanners.split_off(1); // we need to connect these
    let mut processing = scanners; // we need to check these

    while let Some(scanner1) = processing.pop() {
        let (aligned, still_unsolved): (Vec<Scanner>, Vec<Scanner>) = unsolved
            .into_iter()
            .map(|scanner2| align(&scanner1, scanner2))
            .partition_result();
        processing.extend(aligned);
        unsolved = still_unsolved;
        solved.push(scanner1);
    }
    solved
}

/// Ok(scanner2) if they can be aligned, Err(scanner2) otherwise.
fn align(scanner1: &Scanner, scanner2: Scanner) -> Result<Scanner, Scanner> {
    for rot in all_right_handed_rotations() {
        let rotated_scanner2 = rotate(&scanner2, &rot);
        let mut translations: HashMap<Translation, usize> = HashMap::new();
        for (b1, b2) in iproduct!(scanner1.iter(), rotated_scanner2.iter()) {
            let translation: Translation = subtract(b1, b2);
            match translations.entry(translation).or_default() {
                &mut 11 => return Ok(translate(&rotated_scanner2, &translation)),
                n_translations => *n_translations += 1,
            }
        }
    }
    Err(scanner2)
}

/// A beacon emitting its location.
type Beacon = (i64, i64, i64);

/// A vector between two beacons.
type Translation = (i64, i64, i64);

/// A set of beacons as seen from a scanner.
type Scanner = Vec<Beacon>;

/// An axis-aligned rotation: -1=-x, 1=x, -2=-y, 2=y, -3=-z, 3=z
type Rotation = (i8, i8, i8);

/// Find the vector from one beacon to another
fn subtract(b1: &Beacon, b2: &Beacon) -> Translation {
    (b1.0 - b2.0, b1.1 - b2.1, b1.2 - b2.2)
}

/// Rotate all the beacons in a scanner.
fn rotate(scanner: &Scanner, rot: &Rotation) -> Scanner {
    let coord = |b: &Beacon, axis: i8| match axis {
        -1 => -b.0,
        -2 => -b.1,
        -3 => -b.2,
        1 => b.0,
        2 => b.1,
        3 => b.2,
        _ => panic!("Unexpected axis in rotation: {axis}"),
    };
    let rotate = |b: &Beacon| (coord(b, rot.0), coord(b, rot.1), coord(b, rot.2));
    scanner.iter().map(rotate).collect()
}

/// Tranlate all the beacons in a scanner.
fn translate(scanner: &Scanner, t: &Translation) -> Scanner {
    let translate = |b: &Beacon| (b.0 + t.0, b.1 + t.1, b.2 + t.2);
    scanner.iter().map(translate).collect()
}

/// An iterator over all 24 axis-aligned right-handed rotations
fn all_right_handed_rotations() -> impl Iterator<Item = Rotation> {
    let evens = [(1, 2, 3), (2, 3, 1), (3, 1, 2)].into_iter();
    let odds = [(2, 1, 3), (1, 3, 2), (3, 2, 1)].into_iter();
    let odds = odds.flat_map(|(i, j, k)| [(-i, j, k), (i, -j, k), (i, j, -k), (-i, -j, -k)]);
    let evens = evens.flat_map(|(i, j, k)| [(i, j, k), (i, -j, -k), (-i, j, -k), (-i, -j, k)]);
    evens.chain(odds)
}

fn parse_scanner(s: &str) -> Scanner {
    let (_, s) = s.split_once("\n").unwrap();
    let re = Regex::new(r"(\-?\d+),(\-?\d+),(\-?\d+)").unwrap();
    [(0, 0, 0)].into_iter().chain(parse_lines(re, s)).collect()
}

fn parse_input(input: &str) -> Vec<Scanner> {
    input.split("\n\n").map(parse_scanner).collect()
}
