#![allow(dead_code)]

use aoc::parse_regex::parse_lines;
use itertools::{iproduct, Itertools};
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::iter::Iterator;
use std::ops::RangeInclusive;

const TEST_INPUT_1: &str = "
on x=10..12,y=10..12,z=10..12
on x=11..13,y=11..13,z=11..13
off x=9..11,y=9..11,z=9..11
on x=10..10,y=10..10,z=10..10";

const TEST_INPUT_2: &str = "
on x=-20..26,y=-36..17,z=-47..7
on x=-20..33,y=-21..23,z=-26..28
on x=-22..28,y=-29..23,z=-38..16
on x=-46..7,y=-6..46,z=-50..-1
on x=-49..1,y=-3..46,z=-24..28
on x=2..47,y=-22..22,z=-23..27
on x=-27..23,y=-28..26,z=-21..29
on x=-39..5,y=-6..47,z=-3..44
on x=-30..21,y=-8..43,z=-13..34
on x=-22..26,y=-27..20,z=-29..19
off x=-48..-32,y=26..41,z=-47..-37
on x=-12..35,y=6..50,z=-50..-2
off x=-48..-32,y=-32..-16,z=-15..-5
on x=-18..26,y=-33..15,z=-7..46
off x=-40..-22,y=-38..-28,z=23..41
on x=-16..35,y=-41..10,z=-47..6
off x=-32..-23,y=11..30,z=-14..3
on x=-49..-5,y=-3..45,z=-29..18
off x=18..30,y=-20..-8,z=-3..13
on x=-41..9,y=-7..43,z=-33..15
on x=-54112..-39298,y=-85059..-49293,z=-27449..7877
on x=967..23432,y=45373..81175,z=27513..53682";

// a: Cubes -> Clamp -> Remap -> Solve -> Count
// a: Cubes -> Remap -> Solve -> Count

fn main() {
    // let clamp = |i| isize::max(-50, isize::min(50, i));
    let input = include_str!("../../puzzle_inputs/day_22.txt");
    let cubes = parse_input(input);
    println!("answer: {}", solve_22a(&cubes));
}

fn solve_22a(cubes: &[Cube]) -> usize {
    let mut grid: HashSet<Pt> = HashSet::new();
    for cube in cubes {
        // let (mode, min_x, max_x, min_y, max_y, min_z, max_z): Row = row;
        // println!("mode: {mode}");
        // if let Some(Cube { xs, ys, zs }) =
        //     Cube::from_coords(min_x, max_x, min_y, max_y, min_z, max_z)
        // {
        //     println!("trims to: {xs:?} {ys:?} {zs:?}");
        println!("cube: {cube:?}");
        if let Some(cube) = cube.clamp() {
            println!("cube: {cube:?}");
            match cube.additive {
                true => iproduct!(cube.xs, cube.ys, cube.zs).for_each(|pt| {
                    grid.insert(pt);
                }),
                false => iproduct!(cube.xs, cube.ys, cube.zs).for_each(|pt| {
                    grid.remove(&pt);
                }),
            }
        }
        println!();
    }
    grid.len()
}

type Row<'a> = (&'a str, isize, isize, isize, isize, isize, isize);

// #[derive(PartialEq, Eq, Hash)]
#[derive(Debug)]
struct Cube {
    additive: bool,
    xs: RangeInclusive<isize>,
    ys: RangeInclusive<isize>,
    zs: RangeInclusive<isize>,
}

type Pt = (isize, isize, isize);

impl Cube {
    fn from_row((mode, min_x, max_x, min_y, max_y, min_z, max_z): Row) -> Self {
        Self {
            additive: mode == "on",
            xs: min_x..=max_x,
            ys: min_y..=max_y,
            zs: min_z..=max_z,
        }
    }

    fn clamp(&self) -> Option<Self> {
        let clamped_ranges: Vec<RangeInclusive<isize>> = [&self.xs, &self.ys, &self.zs]
            .into_iter()
            .flat_map(|range| match (*range.start(), *range.end()) {
                (_, j) if j < -50 => None,
                (i, j) if i < -50 && j <= 50 => Some(-50..=j),
                (i, j) if i < -50 && j > 50 => Some(-50..=50),
                (i, j) if i <= 50 && j <= 50 => Some(i..=j),
                (i, j) if i <= 50 && j > 50 => Some(i..=50),
                (i, _) if i > 50 => None,
                (i, j) => unimplemented!("Impossible range: {i}..={j}"),
            })
            .collect();
        (clamped_ranges.len() == 3).then(|| Cube {
            additive: self.additive,
            xs: clamped_ranges[0].clone(),
            ys: clamped_ranges[1].clone(),
            zs: clamped_ranges[2].clone(),
        })
    }
}

struct Remap {
    xs: Vec<isize>,
    ys: Vec<isize>,
    zs: Vec<isize>,
}

impl Remap {
    fn from_cubes(cubes: &[Cube]) -> (Self, Vec<Cube>) {
        let mut xs: Vec<isize> = Vec::new();
        let mut ys: Vec<isize> = Vec::new();
        let mut zs: Vec<isize> = Vec::new();
        for cube in cubes {
            xs.push(*cube.xs.start());
            ys.push(*cube.ys.start());
            zs.push(*cube.zs.start());
            xs.push(*cube.xs.end());
            ys.push(*cube.ys.end());
            zs.push(*cube.zs.end());
        }
        let xs: Vec<isize> = xs.iter().copied().unique().sorted().collect();
        let ys: Vec<isize> = ys.iter().copied().unique().sorted().collect();
        let zs: Vec<isize> = zs.iter().copied().unique().sorted().collect();
        let x_map: HashMap<isize, usize> = xs.iter().enumerate().map(|(k, &v)| (v, k)).collect();
        let y_map: HashMap<isize, usize> = ys.iter().enumerate().map(|(k, &v)| (v, k)).collect();
        let z_map: HashMap<isize, usize> = zs.iter().enumerate().map(|(k, &v)| (v, k)).collect();
        let remapped_cubes = cubes
            .iter()
            .map(|cube| Cube {
                additive: cube.additive,
                xs: (x_map[cube.xs.start()] as isize)..=(x_map[cube.xs.end()] as isize),
                ys: (y_map[cube.ys.start()] as isize)..=(y_map[cube.ys.end()] as isize),
                zs: (z_map[cube.zs.start()] as isize)..=(z_map[cube.zs.end()] as isize),
            })
            .collect();
        (Remap { xs, ys, zs }, remapped_cubes)
    }
}

// fn solve_XXa() -> usize {
//     123
// }

// fn solve_XXb() -> usize {
//     456
// }

// type Row<'a> = (&'a str, isize, isize);
// type Row<'a> = (&'a str, isize, isize, isize, isize);

// fn parse_input(input: &str) -> Vec<Cube> {
//     let mut regex = String::from(r"(on|off)");
//     regex += r" x=(\-?\d+)..(\-?\d+)";
//     regex += r",y=(\-?\d+)..(\-?\d+)";
//     regex += r",z=(\-?\d+)..(\-?\d+)";
//     let re = Regex::new(regex.as_str()).unwrap();
//     // regex += r" z=(\-?\d+)..(\-?\d+)";

//     parse_lines(re, input).map(Cube::from_row).collect()
// }

// fn solve_XXa() -> usize {
//     123
// }

// fn solve_XXb() -> usize {
//     456
// }

// type Row<'a> = (&'a str, isize, isize);
// type Row<'a> = (&'a str, isize, isize, isize, isize);

fn parse_input(input: &str) -> Vec<Cube> {
    let mut regex = String::from(r"(on|off)");
    regex += r" x=(\-?\d+)..(\-?\d+)";
    regex += r",y=(\-?\d+)..(\-?\d+)";
    regex += r",z=(\-?\d+)..(\-?\d+)";
    let re = Regex::new(regex.as_str()).unwrap();
    // regex += r" z=(\-?\d+)..(\-?\d+)";

    parse_lines(re, input).map(Cube::from_row).collect()
}
