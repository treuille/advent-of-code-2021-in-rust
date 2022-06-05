#![allow(dead_code, unused_imports, clippy::needless_collect)]

use aoc::parse_regex::parse_lines;
use itertools::{iproduct, Itertools};
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::iter::Iterator;
use std::ops::Range;
use std::rc::Rc;

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

const MY_INPUT_1: &str = "
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
on x=-41..9,y=-7..43,z=-33..15";

// b todo:
// - keep a mutable list of cubes
// - update that list whenever we need to

fn main() {
    // let input = TEST_INPUT_2;
    let input = TEST_INPUT_1;
    // let input = include_str!("../../puzzle_inputs/day_22.txt");
    let instructions = parse_input(input);
    println!("22a: {}", solve_22a(&instructions));
    println!("22b: {:?}", solve_22b(instructions));

    // let cubes = vec![
    // k
    //     Cube {
    //         additive: true,
    //         xs: 1..=10,
    //         ys: 1..=10,
    //         zs: 1..=10,
    //     },
    //     Cube {
    //         additive: true,
    //         xs: 20..=30,
    //         ys: 20..=30,
    //         zs: 20..=30,
    //     },
    // ];
    // println!("22b: {}", solve_22b(&cubes));
}

fn solve_22a(steps: &[Step]) -> usize {
    let steps: Vec<Step> = steps.iter().filter_map(Step::clamp).collect();
    let mut grid: HashSet<Pt> = HashSet::new();
    let ignore = |_: bool| ();
    for step in steps {
        match step {
            Step::On(cube) => cube.pts().for_each(|pt| ignore(grid.insert(pt))),
            Step::Off(cube) => cube.pts().for_each(|pt| ignore(grid.remove(&pt))),
        }
    }
    grid.len()
}

type Cubes = Vec<Cube>;

fn solve_22b(steps: Vec<Step>) {
    let volume = |cubes: &Cubes| -> isize { cubes.iter().map(Cube::volume).sum() };
    let steps: Vec<Step> = steps.iter().filter_map(Step::clamp).collect();
    let mut cubes: Cubes = Cubes::new();
    println!("Just starting: {}", cubes.len());
    for (i, step) in steps.into_iter().enumerate() {
        println!("\nstep: {} cubes: {}", i, cubes.len());
        match step {
            Step::On(cube) => {
                cubes = cube.subtract_from(cubes);
                cubes.push(cube);
                println!("-> on");
            }
            Step::Off(cube) => {
                cubes = cube.subtract_from(cubes);
                println!("-> off");
            }
        }
        let all_disjoint = Cube::all_disjoint(&cubes);
        assert!(all_disjoint);
        println!(
            "cubes: {} volume: {} all_disjoint: {} -> {:?}",
            cubes.len(),
            volume(&cubes),
            all_disjoint,
            cubes,
        );
    }
    todo!("nothing")
}

/// A voxel in 3-space.
type Pt = [isize; 3];

/// A row of the input file.
type Row<'a> = (&'a str, isize, isize, isize, isize, isize, isize);

#[derive(PartialEq, Debug)]
struct Cube([Range<isize>; 3]);

impl Cube {
    fn pts(&self) -> impl Iterator<Item = Pt> {
        let Cube([xs, ys, zs]) = self;
        iproduct!(xs.clone(), ys.clone(), zs.clone()).map(|(x, y, z)| [x, y, z])
    }

    fn volume(&self) -> isize {
        self.0.iter().map(|range| range.end - range.start).product()
    }

    fn clamp(&self) -> Option<Self> {
        let mut clamped_ranges = [0..0, 0..0, 0..0];
        for (clamped_range, range) in clamped_ranges.iter_mut().zip(self.0.iter()) {
            *clamped_range = match (range.start, range.end) {
                (_, j) if j <= -50 => return None,
                (i, j) if i <= -50 && j <= 51 => -50..j,
                (i, j) if i <= -50 && j > 51 => -50..51,
                (i, j) if i <= 51 && j <= 51 => i..j,
                (i, j) if i <= 51 && j > 51 => i..51,
                (i, _) if i > 51 => return None,
                (i, j) => unimplemented!("Impossible range: {i}..={j}"),
            }
        }
        Some(Cube(clamped_ranges))
    }

    /// Are these two cubes disjoint?
    fn disjoint(&self, other: &Self) -> bool {
        self.0
            .iter()
            .zip(other.0.iter())
            .all(|(range_1, range_2)| range_1.end <= range_2.start || range_2.end <= range_1.start)
    }

    /// Does this cube contain the other?
    fn contains(&self, other: &Self) -> bool {
        self.0
            .iter()
            .zip(other.0.iter())
            .all(|(self_range, other_range)| {
                other_range.start >= self_range.start && other_range.end <= self_range.end
            })
    }

    /// Are all these cubes dijoint?
    fn all_disjoint(cubes: &Cubes) -> bool {
        cubes
            .iter()
            .zip(cubes.iter())
            .all(|(cube_1, cube_2)| cube_1 == cube_2 || cube_1.disjoint(cube_2))
    }

    /// Subract this cube from the set of cubes we have here.
    fn subtract_from(&self, cubes: Cubes) -> Cubes {
        cubes
            .iter()
            .flat_map(|cube| {
                let mut coords = self.0.iter().zip(cube.0.iter()).map(|(s_range, c_range)| {
                    [
                        Some(s_range.start),
                        Some(s_range.end),
                        (!s_range.contains(&c_range.start)).then(|| c_range.start),
                        (!s_range.contains(&c_range.end)).then(|| c_range.end),
                    ]
                    .into_iter()
                    .flatten()
                    .sorted()
                    .tuple_windows()
                    .map(|(start, end)| start..end)
                });
                let coords_x = coords.next().unwrap();
                let coords_y = coords.next().unwrap();
                let coords_z = coords.next().unwrap();
                // debug - begin
                println!("s_ranges: {:?}", self.0);
                println!("c_ranges: {:?}", cube.0);

                let coords_x: Vec<Range<isize>> = coords_x.collect();
                let coords_y: Vec<Range<isize>> = coords_y.collect();
                let coords_z: Vec<Range<isize>> = coords_z.collect();

                // TODO: Print these out!
                println!("coords_x: {:?}", coords_x);
                println!("coords_y: {:?}", coords_y);
                println!("coords_z: {:?}", coords_z);
                // debug - end
                iproduct!(
                    coords_x.into_iter(),
                    coords_y.into_iter(),
                    coords_z.into_iter()
                )
                .map(|(range_x, range_y, range_z)| {
                    let sub_cube = Cube([range_x, range_y, range_z]);
                    println!(
                        "sub_cube: {:?} keep: {}",
                        sub_cube.0,
                        cube.contains(&sub_cube) && !self.contains(&sub_cube)
                    );
                    sub_cube
                })
                .filter(|sub_cube| cube.contains(sub_cube) && !self.contains(sub_cube))
            })
            .collect()
    }
}

enum Step {
    On(Cube),
    Off(Cube),
}

impl Step {
    fn from_row((mode, min_x, max_x, min_y, max_y, min_z, max_z): Row) -> Self {
        let cube = Cube([min_x..(max_x + 1), min_y..(max_y + 1), min_z..(max_z + 1)]);
        match mode {
            "on" => Step::On(cube),
            "off" => Step::Off(cube),
            _ => panic!("Unexpected mode: \"{mode}\""),
        }
    }

    fn clamp(&self) -> Option<Self> {
        match self {
            Step::On(cube) => cube.clamp().map(Step::On),
            Step::Off(cube) => cube.clamp().map(Step::Off),
        }
    }
}

fn parse_input(input: &str) -> Vec<Step> {
    let mut regex = String::from(r"(on|off)");
    regex += r" x=(\-?\d+)..(\-?\d+)";
    regex += r",y=(\-?\d+)..(\-?\d+)";
    regex += r",z=(\-?\d+)..(\-?\d+)";
    let re = Regex::new(regex.as_str()).unwrap();
    parse_lines(re, input).map(Step::from_row).collect()
}

#[cfg(test)]
mod test {
    use super::Cube;

    #[test]
    fn contains_works() {
        let c1 = Cube([0..2, 0..2, 0..2]);

        let c2 = Cube([0..1, 0..2, 0..2]);
        assert!(c1.contains(&c2), "{:?} should contain {:?}", c1, c2);

        let c2 = Cube([1..2, 0..2, 0..2]);
        assert!(c1.contains(&c2), "{:?} should contain {:?}", c1, c2);

        let c2 = Cube([0..2, 0..2, 0..2]);
        assert!(c1.contains(&c2), "{:?} should contain {:?}", c1, c2);

        let c2 = Cube([1..3, 0..2, 0..2]);
        assert!(!c1.contains(&c2), "{:?} shouldn't contain {:?}", c1, c2);

        let c2 = Cube([0..2, -1..2, 0..2]);
        assert!(c1.contains(&c2), "{:?} shouldn't contain {:?}", c1, c2);
    }
}
