use aoc::parse_regex::parse_lines;
use itertools::{iproduct, Itertools};
use regex::Regex;
use std::collections::HashMap;
use std::mem;
use std::ops::{Add, Sub};

fn main() {
    // let scanners = read_input(include_str!("../../puzzle_inputs/day_19_test.txt"));
    let scanners = read_input(include_str!("../../puzzle_inputs/day_19.txt"));

    let scanners = align_all(scanners);

    println!("19a: {} (362)", solve_19a(&scanners));
    println!("19b: {} (12204)", solve_19b(&scanners));
}

fn solve_19a(scanners: &[Scanner]) -> usize {
    let marker_beacons = scanners.len();
    let all_beacons = scanners
        .iter()
        .flat_map(|scanner| scanner.0.iter())
        .unique()
        .count();
    all_beacons - marker_beacons
}

fn solve_19b(scanners: &[Scanner]) -> i64 {
    let marker_beacons: Vec<&Beacon> = scanners
        .iter()
        .map(|scanner| scanner.0.first().unwrap())
        .collect();
    iproduct!(marker_beacons.iter(), marker_beacons.iter())
        .map(|(&marker1, &marker2)| {
            let t = marker2 - marker1;
            t.0.abs() + t.1.abs() + t.2.abs()
        })
        .max()
        .unwrap()
}

/// Align all the scanners to scanner[0], returning the result.
fn align_all(mut scanners: Vec<Scanner>) -> Vec<Scanner> {
    let n_scanners = scanners.len();
    let mut unsolved = scanners.split_off(1); // we need to connect these
    let mut solved = Vec::new(); // we have checked these against all others
    let mut processing = scanners; // we need to check these

    while let Some(scanner1) = processing.pop() {
        let mut still_unsolved = Vec::new();
        while let Some(scanner2) = unsolved.pop() {
            match align(&scanner1, scanner2) {
                Ok(scanner2) => processing.push(scanner2),
                Err(scanner2) => still_unsolved.push(scanner2),
            }
        }
        mem::swap(&mut unsolved, &mut still_unsolved);
        solved.push(scanner1);
        assert_eq!(
            solved.len() + processing.len() + unsolved.len(),
            n_scanners,
            "Lost track of a beacon."
        );
    }
    solved
}

/// Ok(scanner2) if they can be aligned, Err(scanner2) otherwise.
fn align(scanner1: &Scanner, scanner2: Scanner) -> Result<Scanner, Scanner> {
    for rot in all_right_handed_rotations() {
        // for rot in ROTATIONS.iter() {
        // TODO: rename to scanner2
        let scanner2_rot: Scanner = scanner2.rotate(&rot);

        let mut translations: HashMap<Translation, usize> = HashMap::new();
        for (beacon1, beacon2) in iproduct!(scanner1.0.iter(), scanner2_rot.0.iter()) {
            let translation: Translation = beacon1 - beacon2;
            match translations.entry(translation.clone()).or_default() {
                &mut 11 => return Ok(scanner2_rot.translate(&translation)),
                n_translations => *n_translations += 1,
            }
        }
    }
    Err(scanner2)
}

/// A becon emitting informaton.
#[derive(Debug, PartialEq, Eq, Hash)]
struct Beacon(i64, i64, i64);

/// Adding a Translation to a Beacon yields another Beacon
impl Add<&Translation> for &Beacon {
    type Output = Beacon;

    fn add(self, rhs: &Translation) -> Self::Output {
        Beacon(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

/// Subtraction two Beacon gives a Translation.
impl Sub for &Beacon {
    type Output = Translation;

    fn sub(self, rhs: Self) -> Self::Output {
        Translation(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

/// A set of detected beacons.
#[derive(Debug)]
struct Scanner(Vec<Beacon>);

impl Scanner {
    /// Rotate all the beacons in a scanner.
    fn rotate(&self, rot: &Rotation) -> Scanner {
        let coord = |b: &Beacon, axis: i8| match axis {
            -1 => -b.0,
            -2 => -b.1,
            -3 => -b.2,
            1 => b.0,
            2 => b.1,
            3 => b.2,
            _ => panic!("Unexpected axis in rotation: {axis}"),
        };
        let rotate = |b: &Beacon| Beacon(coord(b, rot.0), coord(b, rot.1), coord(b, rot.2));
        Scanner(self.0.iter().map(rotate).collect())
    }

    /// Tranlate all the beacons in a scanner.
    fn translate(&self, translation: &Translation) -> Scanner {
        Scanner(self.0.iter().map(|beacon| beacon + translation).collect())
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Translation(i64, i64, i64);

/// An axis-aligned rotation: -1=-x, 1=x, -2=-y, 2=y, -3=-z, 3=z
type Rotation = (i8, i8, i8);

/// An iterator over all 24 axis-aligned right-handed rotations
fn all_right_handed_rotations() -> impl Iterator<Item = Rotation> {
    let evens = [(1, 2, 3), (2, 3, 1), (3, 1, 2)].into_iter();
    let odds = [(2, 1, 3), (1, 3, 2), (3, 2, 1)].into_iter();
    let odds = odds.flat_map(|(i, j, k)| [(-i, j, k), (i, -j, k), (i, j, -k), (-i, -j, -k)]);
    let evens = evens.flat_map(|(i, j, k)| [(i, j, k), (i, -j, -k), (-i, j, -k), (-i, -j, k)]);
    evens.chain(odds)
}

fn parse_beacon(s: &str) -> Scanner {
    let (_, s) = s.split_once("\n").unwrap();
    let re = Regex::new(r"(\-?\d+),(\-?\d+),(\-?\d+)").unwrap();

    let mut beacons = vec![Beacon(0, 0, 0)];
    beacons.extend(parse_lines(&re, s).map(|(x, y, z)| Beacon(x, y, z)));
    Scanner(beacons)
}

fn read_input(input: &str) -> Vec<Scanner> {
    input.split("\n\n").map(parse_beacon).collect()
}
