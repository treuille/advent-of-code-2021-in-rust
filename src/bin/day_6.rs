// use ndarray::Array2;
// use regex::Regex;
// use std::cmp::{Ord, Ordering};
// use std::iter;

// type DynIter<Item> = Box<dyn Iterator<Item = Item>>;

fn main() {
    let mut fish: Vec<usize> = include_str!("../../puzzle_inputs/day_6.txt")
        .trim()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();
    println!("{:?}", fish);

    for day in 1..=256 {
        let n_fish = fish.len();
        for i in 0..n_fish {
            fish[i] = match fish[i] {
                0 => {
                    fish.push(8);
                    6
                }
                n => n.checked_sub(1).unwrap(),
            }
        }
        println!("After day {}: {}", day, fish.len());
    }
    // println!("Puzzle 5a: {} (6397)", solve(true));
    // println!("Puzzle 5b: {} (22335)", solve(false));
}
