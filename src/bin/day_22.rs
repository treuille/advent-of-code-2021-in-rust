use aoc::parse_regex::parse_lines;
use itertools::{iproduct, izip, Itertools};
use regex::Regex;
use std::iter::Iterator;
use std::ops;

fn main() {
    let input = include_str!("../../puzzle_inputs/day_22.txt");
    let steps = parse_input(input);
    println!("22a: {} (652209)", solve_22a(&steps));
    println!("22b: {:?} (1217808640648260)", solve_22b(steps));
}

fn solve_22a(steps: &[Step]) -> isize {
    let bound = Cube([-50..51, -50..51, -50..51]);
    solve(Step::clamp(steps, &bound))
}

fn solve_22b(steps: Vec<Step>) -> isize {
    solve(steps)
}

fn solve(steps: Vec<Step>) -> isize {
    let total_volume = |cubes: Vec<Cube>| cubes.iter().map(Cube::volume).sum();
    match Cube::split(steps.iter().map(|step| &step.cube)) {
        // Try to solve recursively
        Some(sub_bounds) => sub_bounds
            .iter()
            .map(|sub_bound| solve(Step::clamp(&steps, sub_bound)))
            .sum(),

        // Otherwise, solve iteratively
        None => total_volume(steps.iter().fold(Vec::new(), Cube::apply_step)),
    }
}

type Range = ops::Range<isize>;

#[derive(Clone)]
struct Cube([Range; 3]);

impl Cube {
    /// Returns the voluem of this cube.
    fn volume(&self) -> isize {
        self.0.iter().map(|range| range.end - range.start).product()
    }

    ///  Clamp's this to lie within the given bound bound.
    fn clamp(&self, Cube(bounds): &Cube) -> Option<Self> {
        let Cube(ranges) = self;
        let mut clamped_ranges = [0..0, 0..0, 0..0];
        for (clamped_range, range, bound) in izip!(clamped_ranges.iter_mut(), ranges, bounds) {
            *clamped_range = match (range.start, range.end) {
                (_, j) if j <= bound.start => return None,
                (i, j) if i <= bound.start && j <= bound.end => bound.start..j,
                (i, j) if i <= bound.start && j > bound.end => bound.clone(),
                (i, j) if i <= bound.end && j <= bound.end => i..j,
                (i, j) if i <= bound.end && j > bound.end => i..bound.end,
                (i, _) if i > bound.end => return None,
                (i, j) => unimplemented!("Impossible range: {i}..={j}"),
            }
        }
        Some(Cube(clamped_ranges))
    }

    /// Are these two cubes disjoint?
    fn disjoint(&self, other: &Self) -> bool {
        let range_disjoint = |(r1, r2): (&Range, &Range)| r1.end <= r2.start || r2.end <= r1.start;
        self.0.iter().zip(other.0.iter()).any(range_disjoint)
    }

    /// Does this cube contain the other?
    fn contains(&self, other: &Self) -> bool {
        let range_contains = |(r1, r2): (&Range, &Range)| r2.start >= r1.start && r2.end <= r1.end;
        self.0.iter().zip(other.0.iter()).all(range_contains)
    }

    /// Subtracts the other cube from this, returning the remaining fragments.
    fn subtract(&self, other: &Self) -> Vec<Self> {
        if self.disjoint(other) {
            return vec![Cube(self.0.clone())];
        }
        let mut coords = self.0.iter().zip(other.0.iter()).map(|(r1, r2)| {
            [r1.start, r1.end, r2.start, r2.end]
                .into_iter()
                .unique()
                .sorted()
                .tuple_windows()
                .map(|(start, end)| start..end)
        });
        let coords_x = coords.next().unwrap();
        let coords_y = coords.next().unwrap();
        let coords_z = coords.next().unwrap();
        iproduct!(coords_x, coords_y, coords_z)
            .map(|(range_x, range_y, range_z)| Cube([range_x, range_y, range_z]))
            .filter(|sub_cube| self.contains(sub_cube) && !other.contains(sub_cube))
            .collect()
    }

    fn apply_step(cubes: Vec<Self>, step: &Step) -> Vec<Self> {
        cubes
            .iter()
            .flat_map(|cube| cube.subtract(&step.cube))
            .chain(step.additive.then(|| step.cube.clone()))
            .collect()
    }

    /// Computes a cube that tightly bounds the input cubes
    fn bounding<'a>(mut cubes: impl Iterator<Item = &'a Cube>) -> Self {
        let base_cube = cubes.next().unwrap().clone();
        cubes.fold(base_cube, |Cube(ranges_1), Cube(ranges_2)| {
            Cube([0, 1, 2].map(|i| {
                let bound_start = isize::min(ranges_1[i].start, ranges_2[i].start);
                let bound_end = isize::max(ranges_1[i].end, ranges_2[i].end);
                bound_start..bound_end
            }))
        })
    }

    /// Splits this set of cubes two, or None if no such good split can be found.
    fn split<'a, CubeIter>(cubes: CubeIter) -> Option<[Cube; 2]>
    where
        CubeIter: Iterator<Item = &'a Cube> + Clone,
    {
        if cubes.clone().count() <= 40 {
            return None;
        }
        let Cube(bounds) = Self::bounding(cubes.clone());
        let (max_len, axis) = bounds
            .iter()
            .enumerate()
            .map(|(i, r)| (r.end - r.start, i))
            .max()
            .unwrap();
        if max_len <= 10 {
            return None;
        }
        let mut coords: Vec<isize> = cubes
            .flat_map(|Cube(ranges)| [ranges[axis].start, ranges[axis].end])
            .collect();
        coords.sort_unstable();
        let median = coords[coords.len() / 2];
        if median == bounds[axis].start || median == bounds[axis].end {
            return None;
        }
        let mut split_bound_1 = bounds.clone();
        let mut split_bound_2 = bounds.clone();
        split_bound_1[axis].end = median;
        split_bound_2[axis].start = median;
        Some([Cube(split_bound_1), Cube(split_bound_2)])
    }
}

/// A row of the input file.
type Row<'a> = (&'a str, isize, isize, isize, isize, isize, isize);

struct Step {
    additive: bool,
    cube: Cube,
}

impl Step {
    /// Parse out a Step from a row of the input file.
    fn from_row((mode, min_x, max_x, min_y, max_y, min_z, max_z): Row) -> Self {
        Self {
            additive: mode == "on",
            cube: Cube([min_x..(max_x + 1), min_y..(max_y + 1), min_z..(max_z + 1)]),
        }
    }

    /// Clamp the steps to within the bounds, discarding those lying outside.
    fn clamp(steps: &[Self], bound: &Cube) -> Vec<Self> {
        steps
            .iter()
            .filter_map(|step| {
                bound.clamp(&step.cube).map(|clamped_cube| Self {
                    additive: step.additive,
                    cube: clamped_cube,
                })
            })
            .collect()
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
