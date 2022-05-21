use aoc::parse_regex::parse_lines;
use itertools::iproduct;
// use ndarray::prelude::*;
use regex::Regex;

// 1=i -1=-i 2=j -2=-j 3=k -3=-k
type Rotation = (i8, i8, i8);

// all right handed rotations
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
    (-3, -2, -1)
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

fn main() {
    // Figure out the possible rotations
    iproduct!(1..=3, 1..=3, 1..=3)
        .filter(|(i, j, k)| i != j && j != k && i != k)
        .flat_map(|(i, j, k)| {
            [
                (i, j, k),
                (-i, j, k),
                (i, -j, k),
                (-i, -j, k),
                (i, j, -k),
                (-i, j, -k),
                (i, -j, -k),
                (-i, -j, -k),
            ]
        })
        .filter(|(i, j, k)| {
            // [y1 * z2, -z1 * x2, x2 * y2]
            // x = y1 * z2 - z1 * y2
            // y = z1 * x2 - x1 * z2
            // z = x1 * y2 - y1 * x2
            let vs: Vec<Pt> = [i, j, k].into_iter().map(|coord| match coord {
                -1 => (-1, 0, 0),
                -2 => (0, -1, 0),
                -3 => (0, 0, -1),
                1 => (1, 0, 0),
                2 => (0, 1, 0),
                3 => (0, 0, 1),
                _ => unimplemented!("Must be on of these six numbers."),
            }).collect();

            if vs[2].0 != (vs[0].1 * vs[1].2 - vs[0].2 * vs[1].1) {
                return false;
            }
            if vs[2].1 != (vs[0].2 * vs[1].0 - vs[0].0 * vs[1].2) {
                return false;
            }
            if vs[2].2 != (vs[0].0 * vs[1].1 - vs[0].1 * vs[1].0) {
                return false;
            }
            return true;
            // if vs[0] != (y1 * z2 - z1 * y2) {
            // }
            // // y = z1 * x2 - x1 * z2
            // // z = x1 * y2 - y1 * x2
            // println!("({i}, {j}, {k})");
            // println!("x: {:?}", vs[0]);
            // println!("y: {:?}", vs[1]);
            // println!("z: {:?}", vs[2]);
            // todo!("Make sure this is a righ-handed coordinate system.");
        })//.enumerate()
        .for_each(|x| println!("{:?}", x));
    // let mut beacons = read_input().into_iter();
    // let beacons_0 = beacons.next().unwrap();
    // let beacons_1 = beacons.next().unwrap();

    // println!("beacons_0: {beacons_0:?}");
    // println!("beacons_1: {beacons_1:?}");

    // println!("0,1 -> {:?}", align(&beacons_0, beacons_1));

    // let mut unsolved = beacons.split_off(1);
    // let mut processing = beacons;
    // let mut solved = Vec::new();

    // while let Some(beacon1) = processing.pop() {
    //     for beacon2 in unsolved {
    //         match align(&beacon1, beacon2) {
    //             Ok(beacon2) => todo!("found a match"),
    //             Err(beacon2) => todo!("didn't match"),
    //         }
    //     }
    //     todo!("need to reset unsolved");
    // }

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
    println!("rotations: {:?}", ROTATIONS);
}

type Beacon = Vec<Pt>;

type Pt = (i64, i64, i64);

// Size of indices, with n

// fn solve_XXa() -> usize {
//     123
// }

// fn solve_XXb() -> usize {
//     456
// }

// /// Ok(beacon2) if they can be aligned, Err(beacon2) otherwise.
// fn align(beacon1: &Beacon, beacon2: Beacon) -> Result<Beacon, Beacon> {

// }

// fn align_with(beacon1: &Beacon, beacon2: &Beacon, tranform: F) where
// F: Fn(Pt) -> Pt {

// }

fn parse_beacon(s: &str) -> Beacon {
    let s = s.split_once("\n").unwrap().1;
    let re = Regex::new(r"(\-?\d+),(\-?\d+),(\-?\d+)").unwrap();
    parse_lines(&re, s).collect()
    // .map(|(x, y, z)| [x, y, z].into_iter().collect())
    // .collect()
}

/// Read the input file and turn it into an Array2<u8>
fn read_input() -> Vec<Beacon> {
    let beacons = include_str!("../../puzzle_inputs/day_19_test.txt").split("\n\n");
    beacons.map(parse_beacon).collect()
}
