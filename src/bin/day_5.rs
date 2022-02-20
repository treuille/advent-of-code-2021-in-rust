use ndarray::Array2;
use regex::Regex;
use std::cmp;

fn main() {
    let mut terrain: Array2<u16> = Array2::default((1000, 1000));
    let re = Regex::new(r"(\d+),(\d+) -> (\d+),(\d+)").unwrap();
    let input = include_str!("../../puzzle_inputs/day_5.txt");
    for line in input.lines() {
        let caps = re.captures(line).unwrap();
        let x1: usize = caps[1].parse().unwrap();
        let y1: usize = caps[2].parse().unwrap();
        let x2: usize = caps[3].parse().unwrap();
        let y2: usize = caps[4].parse().unwrap();
        println!("{line}");
        println!("({}, {}) -> ({}, {})", x1, y1, x2, y2);
        let range: Box<dyn Iterator<Item = (usize, usize)>> = match (x1 == x2, y1 == y2) {
            (true, _) => Box::new((cmp::min(y1, y2)..=cmp::max(y1, y2)).map(|y| (x1, y))),
            (_, true) => Box::new((cmp::min(x1, x2)..=cmp::max(x1, x2)).map(|x| (x, y1))),
            _ => continue,
        };
        for idx in range {
            terrain[idx] += 1;
        }
    }
    println!(
        "answer: {} (6397)",
        terrain.iter().filter(|&x| x > &1).count()
    );
}
