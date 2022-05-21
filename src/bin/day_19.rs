use aoc::parse_regex::parse_lines;
use itertools::iproduct;
use std::collections::{HashMap, HashSet};
// use ndarray::prelude::*;
use regex::Regex;
use std::fmt::{Debug, Error as FormatErr, Formatter};
use std::ops::{Add, Sub};

fn main() {
    println!("rotations: {:?}", ROTATIONS);

    // test the alignment algorithm with the first two scanners
    let scanners = read_input();
    println!("{:?}", scanners[0]);
    let mut scanners = scanners.into_iter();
    let scanner1 = scanners.next().unwrap();
    let scanner2 = scanners.next().unwrap();
    let scanner3 = align(&scanner1, scanner2);
    println!("scanner3: {:?}", scanner3);
    if let Ok(scanner3) = scanner3 {
        let scanner1: HashSet<Beacon> = scanner1.0.into_iter().collect();
        let mut n_matches: usize = 0;
        for beacon3 in scanner3.0.iter() {
            if scanner1.contains(beacon3) {
                n_matches += 1;
                println!("{beacon3:?} -> {n_matches}");
            } else {
                println!("{beacon3:?} XXX");
            }
        }
    }
}

// fn search_for_alignment(mut scanners: Vec<Scanner>) {
//     let n_scanners = scanners.len();
//     let mut unsolved = scanners.split_off(1); // we need to connect these
//     let mut solved = Vec::new(); // we have checked these against all others
//     let mut processing = scanners; // we need to check these
//     println!("just starting");
//     println!("solved: {}", solved.len());
//     println!("processing: {}", processing.len());
//     println!("unsolved: {}\n", unsolved.len());
//     while let Some(scanner1) = processing.pop() {
//         let mut still_unsolved = Vec::new();
//         while let Some(scanner2) = unsolved.pop() {
//             println!("Solving beacon with {} scanners.", scanner2.len());
//             match align(&scanner1, scanner2) {
//                 Ok(scanner2) => processing.push(scanner2),
//                 Err(scanner2) => still_unsolved.push(scanner2),
//             }

//             println!("finished inter while");
//             println!("solved: {}", solved.len());
//             println!("processing: {}", processing.len());
//             println!("unsolved: {}\n", unsolved.len());
//         }
//         mem::swap(&mut unsolved, &mut still_unsolved);
//         solved.push(scanner1);
//         assert_eq!(
//             solved.len() + processing.len() + unsolved.len(),
//             n_scanners,
//             "Lost track of a beacon."
//         );
//         assert!(processing.len() > 0, "Nothing more to process.");

//         println!("finished outer while");
//         println!("solved: {}", solved.len());
//         println!("processing: {}", processing.len());
//         println!("unsolved: {}\n", unsolved.len());
//         panic!("outer while loop");
//     }

//     // let mut unsolved2 = Vec::with_capacity(unsolved.len() - 1);
//     // let candidates = unsolved.drain();
//     // todo!("Move scanners unsolved -> solved");
//     // while let Some(scanner2) = candidates.next() {
//     // }
//     // }
//     // for (scanner1, scanner2) in iproduct!(solved.iter(), .enumerate()) {
//     //     match align(scanner1, scanner2) {
//     //         Ok(scanner2) => {
//     //             solved.push(scanner2);
//     //             break;
//     //         },
//     //         Err(scanner2)
//     //     }
//     //     tood!("What happens in this loop?")
//     // }
//     // }
//     // println!("solved: {}", solved.len());
//     // println!("unsolved: {}", unsolved.len());
// }

/// Ok(scanner2) if they can be aligned, Err(scanner2) otherwise.
fn align(scanner1: &Scanner, scanner2: Scanner) -> Result<Scanner, Scanner> {
    for rot in ROTATIONS.iter() {
        println!("testing rotation: {:?}", rot);

        // TODO: rename to scanner2
        let scanner2_rot: Scanner = scanner2.rotate(rot);

        let mut translations: HashMap<Translation, usize> = HashMap::new();
        for (beacon1, beacon2) in iproduct!(scanner1.0.iter(), scanner2_rot.0.iter()) {
            let translation: Translation = beacon1 - beacon2;
            *translations.entry(translation).or_default() += 1;
        }
        println!("found {} translations", translations.len());
        println!("max entries: {:?}", translations.values().max());
        println!("min entries: {:?}", translations.values().min());
        println!();

        let successful_translations: Vec<Translation> = translations
            .into_iter()
            .filter_map(|(translation, n_translations)| (n_translations >= 12).then(|| translation))
            .collect();

        let translation: &Translation = match successful_translations.len() {
            0 => continue,
            1 => successful_translations.get(0).unwrap(),
            n => panic!("Too many successful translations for {rot:?}: {n}"),
        };
        return Ok(Scanner(
            scanner2_rot
                .0
                .iter()
                .map(|beacon| beacon + translation)
                .collect(),
        ));
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
}

#[derive(Debug, PartialEq, Eq, Hash)]
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
    Scanner(
        parse_lines(&re, s)
            .map(|(x, y, z)| Beacon(x, y, z))
            .collect(),
    )
}

/// Read the input file and turn it into an Array2<u8>
fn read_input() -> Vec<Scanner> {
    let scanners = include_str!("../../puzzle_inputs/day_19_test.txt").split("\n\n");
    scanners.map(parse_beacon).collect()
}
