use ndarray::Array2;
use regex::Regex;
use std::cmp;
use std::cmp::{Ord, Ordering};
use std::iter;

type DynIter<Item> = Box<dyn Iterator<Item = Item>>;

fn main() {
    println!("Puzzle 5a: {} (6397)", solve(rectilinear_lines));
    println!("Puzzle 5b: {} (22335)", solve(all_lines));
}

fn solve<F>(line_iter: F) -> usize
where
    F: Fn(usize, usize, usize, usize) -> DynIter<(usize, usize)>,
{
    let mut terrain: Array2<u16> = Array2::default((1000, 1000));
    let re = Regex::new(r"(\d+),(\d+) -> (\d+),(\d+)").unwrap();
    let input = include_str!("../../puzzle_inputs/day_5.txt");
    for line in input.lines() {
        let caps = re.captures(line).unwrap();
        let x1: usize = caps[1].parse().unwrap();
        let y1: usize = caps[2].parse().unwrap();
        let x2: usize = caps[3].parse().unwrap();
        let y2: usize = caps[4].parse().unwrap();

        for idx in line_iter(x1, y1, x2, y2) {
            terrain[idx] += 1;
        }
    }
    terrain.iter().filter(|&x| x > &1).count()
}

fn rectilinear_lines(x1: usize, y1: usize, x2: usize, y2: usize) -> DynIter<(usize, usize)> {
    match (x1 == x2, y1 == y2) {
        (true, _) => Box::new((cmp::min(y1, y2)..=cmp::max(y1, y2)).map(move |y| (x1, y))),
        (_, true) => Box::new((cmp::min(x1, x2)..=cmp::max(x1, x2)).map(move |x| (x, y1))),
        _ => Box::new(iter::empty()),
    }
}

fn all_lines(x1: usize, y1: usize, x2: usize, y2: usize) -> DynIter<(usize, usize)> {
    let [x_range, y_range]: [DynIter<usize>; 2] =
        [(x1, x2), (y1, y2)].map(|(a, b)| -> DynIter<usize> {
            match a.cmp(&b) {
                Ordering::Less => Box::new(a..=b),
                Ordering::Equal => Box::new(iter::repeat(a)),
                Ordering::Greater => Box::new((b..=a).rev()),
            }
        });
    Box::new(x_range.zip(y_range))
}
