use ndarray::Array2;
use regex::Regex;
use std::cmp::{Ord, Ordering};
use std::iter;

type DynIter<Item> = Box<dyn Iterator<Item = Item>>;

fn main() {
    println!("Puzzle 5a: {} (6397)", solve(true));
    println!("Puzzle 5b: {} (22335)", solve(false));
}

fn solve(skip_diagonals: bool) -> usize {
    let mut terrain: Array2<u16> = Array2::default((1000, 1000));
    let re = Regex::new(r"(\d+),(\d+) -> (\d+),(\d+)").unwrap();
    let input = include_str!("../../puzzle_inputs/day_5.txt");
    for line in input.lines() {
        let caps = re.captures(line).unwrap();
        let x1: usize = caps[1].parse().unwrap();
        let y1: usize = caps[2].parse().unwrap();
        let x2: usize = caps[3].parse().unwrap();
        let y2: usize = caps[4].parse().unwrap();

        if skip_diagonals && (x1 != x2) && (y1 != y2) {
            continue;
        }

        let [x_range, y_range]: [DynIter<usize>; 2] =
            [(x1, x2), (y1, y2)].map(|(a, b)| -> DynIter<usize> {
                match a.cmp(&b) {
                    Ordering::Less => Box::new(a..=b),
                    Ordering::Equal => Box::new(iter::repeat(a)),
                    Ordering::Greater => Box::new((b..=a).rev()),
                }
            });
        for idx in x_range.zip(y_range) {
            terrain[idx] += 1;
        }
    }
    terrain.iter().filter(|&x| x > &1).count()
}
