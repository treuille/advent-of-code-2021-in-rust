#![allow(dead_code)]

use aoc::parse_regex::parse_lines;
use itertools::iproduct;
use regex::Regex;
use std::collections::HashSet;
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

type Pt = (isize, isize, isize);

fn clamp(r: RangeInclusive<isize>) -> Option<RangeInclusive<isize>> {
    match r.into_inner() {
        (_, j) if j < -50 => None,
        (i, j) if i < -50 && j <= 50 => Some(-50..=j),
        (i, j) if i < -50 && j > 50 => Some(-50..=50),
        (i, j) if i <= 50 && j <= 50 => Some(i..=j),
        (i, j) if i <= 50 && j > 50 => Some(i..=50),
        (i, _) if i > 50 => None,
        (i, j) => unimplemented!("Impossible range: {i}..={j}"),
    }
}

fn main() {
    let mut grid: HashSet<Pt> = HashSet::new();
    // let clamp = |i| isize::max(-50, isize::min(50, i));
    let input = include_str!("../../puzzle_inputs/day_22.txt");
    for row in parse_input(input) {
        let (mode, min_x, max_x, min_y, max_y, min_z, max_z): Row = row;
        println!("mode: {mode}");
        if let Some(Cube { xs, ys, zs }) =
            Cube::from_coords(min_x, max_x, min_y, max_y, min_z, max_z)
        {
            println!("trims to: {xs:?} {ys:?} {zs:?}");
            match mode {
                "on" => iproduct!(xs, ys, zs).for_each(|pt| {
                    grid.insert(pt);
                }),
                "off" => iproduct!(xs, ys, zs).for_each(|pt| {
                    grid.remove(&pt);
                }),
                _ => panic!("Unexpected mode: \"{mode}\""),
            }
        }
    }
    println!("answer: {}", grid.len());
}

#[derive(PartialEq, Eq, Hash)]
struct Cube {
    xs: RangeInclusive<isize>,
    ys: RangeInclusive<isize>,
    zs: RangeInclusive<isize>,
}

impl Cube {
    fn from_coords(
        min_x: isize,
        max_x: isize,
        min_y: isize,
        max_y: isize,
        min_z: isize,
        max_z: isize,
    ) -> Option<Self> {
        println!("xs: {min_x}..={max_x}");
        println!("ys: {min_y}..={max_y}");
        println!("zs: {min_z}..={max_z}");
        if let Some(xs) = clamp(min_x..=max_x) {
            if let Some(ys) = clamp(min_y..=max_y) {
                if let Some(zs) = clamp(min_z..=max_z) {
                    Some(Cube { xs, ys, zs })
                } else {
                    println!("unable to trim: z");
                    None
                }
            } else {
                println!("unable to trim: y");
                None
            }
        } else {
            println!("unable to trim: x");
            None
        }
    }
}

struct CubeSet(HashSet<Cube>);

// fn split_x(self, x: isize) -> Self {
//     cube_set
//         .into_iter()
//         .flat_map(|Cube { xs, ys, zs }| match xs.clone().into_inner() {
//             (i, j) if i < x && j < x => vec![Cube { xs, ys, zs }],
//             (i, j) if i == x && j < x => unimplemented!("i == x && j < x"),
//             (i, j) if i > x && j < x => unimplemented!("i > x && j < x"),

//             (i, j) if i < x && j == x => vec![Cube { xs, ys, zs }],
//             (i, j) if i == x && j == x => vec![Cube { xs, ys, zs }],
//             (i, j) if i > x && j == x => unimplemented!("i > x && j == x"),

//             (i, j) if i < x && j > x => {
//                 vec![
//                     Cube {
//                         xs: i..=x,
//                         ys: ys.clone(),
//                         zs: zs.clone(),
//                     },
//                     Cube {
//                         xs: x..=j,
//                         ys: ys.clone(),
//                         zs: zs.clone(),
//                     },
//                 ]
//             }
//             (i, j) if i == x && j > x => vec![Cube { xs, ys, zs }],
//             (i, j) if i > x && j > x => vec![Cube { xs, ys, zs }],
//             (i, j) => unimplemented!("what {i} {j}"),
//         })
//         .collect()
// }

impl CubeSet {
    fn add(self, cube: Cube) -> Self {
        //  TODO: Start implementing here.
        todo!("CubeSet::add");
        // let cube_set = self
        //     .split_x(*cube.xs.start())
        //     .split_x(*cube.xs.end())
        //     .split_y(*cube.ys.start())
        //     .split_y(*cube.ys.end())
        //     .split_z(*cube.zs.start())
        //     .split_z(*cube.zs.end())
        // let cube_set = self;
    }

    fn split_x(self, x: isize) -> Self {
        CubeSet(
            self.0
                .into_iter()
                .flat_map(|cube| match cube.xs.clone().into_inner() {
                    (i, j) if i < x && j > x => vec![
                        Cube {
                            xs: i..=x,
                            ys: cube.ys.clone(),
                            zs: cube.zs.clone(),
                        },
                        Cube {
                            xs: x..=j,
                            ys: cube.ys.clone(),
                            zs: cube.zs.clone(),
                        },
                    ],
                    _ => vec![cube],
                })
                .collect(),
        )
    }

    fn split_y(self, y: isize) -> Self {
        CubeSet(
            self.0
                .into_iter()
                .flat_map(|cube| match cube.ys.clone().into_inner() {
                    (i, j) if i < y && j > y => vec![
                        Cube {
                            xs: cube.xs.clone(),
                            ys: i..=y,
                            zs: cube.zs.clone(),
                        },
                        Cube {
                            xs: cube.xs.clone(),
                            ys: y..=j,
                            zs: cube.zs.clone(),
                        },
                    ],
                    _ => vec![cube],
                })
                .collect(),
        )
    }

    fn split_z(self, z: isize) -> Self {
        CubeSet(
            self.0
                .into_iter()
                .flat_map(|cube| match cube.zs.clone().into_inner() {
                    (i, j) if i < z && j > z => vec![
                        Cube {
                            xs: cube.xs.clone(),
                            ys: cube.ys.clone(),
                            zs: i..=z,
                        },
                        Cube {
                            xs: cube.xs.clone(),
                            ys: cube.ys.clone(),
                            zs: z..=j,
                        },
                    ],
                    _ => vec![cube],
                })
                .collect(),
        )
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
type Row<'a> = (&'a str, isize, isize, isize, isize, isize, isize);

fn parse_input(input: &str) -> impl Iterator<Item = Row> {
    let mut regex = String::from(r"(on|off)");
    regex += r" x=(\-?\d+)..(\-?\d+)";
    regex += r",y=(\-?\d+)..(\-?\d+)";
    regex += r",z=(\-?\d+)..(\-?\d+)";
    println!("regex 1: {}", regex);
    let re = Regex::new(regex.as_str()).unwrap();
    // regex += r" z=(\-?\d+)..(\-?\d+)";
    println!("regex 2: {}", regex);

    parse_lines(re, input)
}
