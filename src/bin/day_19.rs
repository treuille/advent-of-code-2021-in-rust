use aoc::parse_regex::parse_lines;
use itertools::{iproduct, Itertools};
use regex::Regex;
use std::collections::HashMap;
use std::fmt::{Debug, Error as FormatErr, Formatter};
use std::mem;
use std::ops::{Add, Sub};

fn main() {
    // // let scanners = read_input(include_str!("../../puzzle_inputs/day_19_test.txt"));
    // let scanners = read_input(include_str!("../../puzzle_inputs/day_19.txt"));

    // let scanners = align_all(scanners);

    // println!("19a: {} (362)", solve_19a(&scanners));
    // println!("19b: {} (12204)", solve_19b(&scanners));
    //
    for (i, rot) in ROTATIONS.iter().enumerate() {
        println!("{i}: {rot:?}");
    }

    // // even permutation
    // 0: Rot(x, y, z)
    // 12: Rot(y, z, x)
    // : Rot(z, x, y)

    // // odd permutations
    // Rot(-x, z, y)
    // 8: Rot(-y, x, z)
    // Rot(-z, y, x)

    let evens = [(1, 2, 3), (2, 3, 1), (3, 1, 2)].into_iter();
    let odds = [(2, 1, 3), (1, 3, 2), (3, 2, 1)].into_iter();
    let evens = evens.flat_map(|(i, j, k)| [(i, j, k), (i, -j, -k), (-i, j, -k), (-i, -j, k)]);
    let odds = odds.flat_map(|(i, j, k)| [(-i, j, k), (i, -j, k), (i, j, -k), (-i, -j, -k)]);
    for (i, even) in evens.chain(odds).enumerate() {
        println!("{i} -> {even:?}");
    }
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
    for rot in ROTATIONS.iter() {
        // TODO: rename to scanner2
        let scanner2_rot: Scanner = scanner2.rotate(rot);

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
        Scanner(self.0.iter().map(|beacon| rot.apply(beacon)).collect())
    }

    /// Tranlate all the beacons in a scanner.
    fn translate(&self, translation: &Translation) -> Scanner {
        Scanner(self.0.iter().map(|beacon| beacon + translation).collect())
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Translation(i64, i64, i64);

#[derive(Clone, Copy)]
enum SignedAxis {
    NegX,
    NegY,
    NegZ,
    PosX,
    PosY,
    PosZ,
}

impl Debug for SignedAxis {
    fn fmt<'a>(&self, formatter: &mut Formatter<'a>) -> Result<(), FormatErr> {
        match self {
            SignedAxis::NegX => formatter.write_str("-x"),
            SignedAxis::NegY => formatter.write_str("-y"),
            SignedAxis::NegZ => formatter.write_str("-z"),
            SignedAxis::PosX => formatter.write_str("x"),
            SignedAxis::PosY => formatter.write_str("y"),
            SignedAxis::PosZ => formatter.write_str("z"),
        }
    }
}

// 1=i NegX=-i 2=j NegY=-j 3=k NegZ=-k
struct Rotation(SignedAxis, SignedAxis, SignedAxis);

// Rotates the point along the origin by the given amount
impl Rotation {
    fn apply(&self, beacon: &Beacon) -> Beacon {
        let coord = |axis: SignedAxis| match axis {
            SignedAxis::NegX => -beacon.0,
            SignedAxis::NegY => -beacon.1,
            SignedAxis::NegZ => -beacon.2,
            SignedAxis::PosX => beacon.0,
            SignedAxis::PosY => beacon.1,
            SignedAxis::PosZ => beacon.2,
        };
        Beacon(coord(self.0), coord(self.1), coord(self.2))
    }
}

impl Debug for Rotation {
    fn fmt<'a>(&self, formatter: &mut Formatter<'a>) -> Result<(), FormatErr> {
        formatter.write_fmt(format_args!(
            "Rot({:?}, {:?}, {:?})",
            self.0, self.1, self.2
        ))
    }
}

/// All right handed rotations.
const ROTATIONS: [Rotation; 24] = [
    Rotation(SignedAxis::PosX, SignedAxis::PosY, SignedAxis::PosZ),
    Rotation(SignedAxis::NegX, SignedAxis::NegY, SignedAxis::PosZ),
    Rotation(SignedAxis::NegX, SignedAxis::PosY, SignedAxis::NegZ),
    Rotation(SignedAxis::PosX, SignedAxis::NegY, SignedAxis::NegZ),
    Rotation(SignedAxis::NegX, SignedAxis::PosZ, SignedAxis::PosY),
    Rotation(SignedAxis::PosX, SignedAxis::NegZ, SignedAxis::PosY),
    Rotation(SignedAxis::PosX, SignedAxis::PosZ, SignedAxis::NegY),
    Rotation(SignedAxis::NegX, SignedAxis::NegZ, SignedAxis::NegY),
    Rotation(SignedAxis::NegY, SignedAxis::PosX, SignedAxis::PosZ),
    Rotation(SignedAxis::PosY, SignedAxis::NegX, SignedAxis::PosZ),
    Rotation(SignedAxis::PosY, SignedAxis::PosX, SignedAxis::NegZ),
    Rotation(SignedAxis::NegY, SignedAxis::NegX, SignedAxis::NegZ),
    Rotation(SignedAxis::PosY, SignedAxis::PosZ, SignedAxis::PosX),
    Rotation(SignedAxis::NegY, SignedAxis::NegZ, SignedAxis::PosX),
    Rotation(SignedAxis::NegY, SignedAxis::PosZ, SignedAxis::NegX),
    Rotation(SignedAxis::PosY, SignedAxis::NegZ, SignedAxis::NegX),
    Rotation(SignedAxis::PosZ, SignedAxis::PosX, SignedAxis::PosY),
    Rotation(SignedAxis::NegZ, SignedAxis::NegX, SignedAxis::PosY),
    Rotation(SignedAxis::NegZ, SignedAxis::PosX, SignedAxis::NegY),
    Rotation(SignedAxis::PosZ, SignedAxis::NegX, SignedAxis::NegY),
    Rotation(SignedAxis::NegZ, SignedAxis::PosY, SignedAxis::PosX),
    Rotation(SignedAxis::PosZ, SignedAxis::NegY, SignedAxis::PosX),
    Rotation(SignedAxis::PosZ, SignedAxis::PosY, SignedAxis::NegX),
    Rotation(SignedAxis::NegZ, SignedAxis::NegY, SignedAxis::NegX),
];

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
