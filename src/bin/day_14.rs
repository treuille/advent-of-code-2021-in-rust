use itertools::{self, Itertools};
use std::collections::HashMap;

fn main() {
    let (mut polymer, xform) = read_input(include_str!("../../puzzle_inputs/day_14.txt"));

    for i in 1..=40 {
        polymer = apply(&xform, polymer);
        match i {
            10 => println!("14a: {} (5656)", solution_for(&polymer)),
            40 => println!("14b: {} (12271437788530)", solution_for(&polymer)),
            _ => (),
        }
    }
}

/// A pair is a length-2 string.
type Pair = (char, char);

/// A polymer is a count of length-2 strings.
type Polymer = HashMap<Pair, usize>;

/// A transform converts a polymer into another.
type Transform = HashMap<Pair, [Pair; 2]>;

/// Read the input file, returning the initial polymer and the tranformation object.
fn read_input(input: &str) -> (Polymer, Transform) {
    // The input consists of two section: the polymer string, and a list of rules.
    let (polymer, rules) = input.trim().split_once("\n\n").unwrap();

    // Parse the polymer string
    let (mut polymer, pairs) = (Polymer::new(), polymer.chars().tuple_windows());
    pairs.for_each(|pair| *polymer.entry(pair).or_default() += 1);

    // Parse the set of rules into a transformation.
    let xform = rules
        .lines()
        .map(|line| {
            let (ab, c) = line.split_once(" -> ").unwrap();
            let (a, b): Pair = ab.chars().tuple_windows().next().unwrap();
            let c = c.chars().next().unwrap();
            ((a, b), [(a, c), (c, b)]) // ab -> ac and cb
        })
        .collect();

    // All done
    (polymer, xform)
}

/// Consumes the polymer, applies the transformation, and returns the result.
fn apply(xform: &Transform, input: Polymer) -> Polymer {
    let mut output = Polymer::new();
    for (pair, count) in input {
        if let Some(new_pairs) = xform.get(&pair) {
            for pair_2 in new_pairs {
                *output.entry(*pair_2).or_default() += count
            }
        }
    }
    output
}

/// Returns the difference between the most and least common character.
fn solution_for(poly: &Polymer) -> usize {
    let mut counts: HashMap<char, usize> = HashMap::new();
    for (&(char_1, char_2), count) in poly.iter() {
        *counts.entry(char_1).or_default() += count;
        *counts.entry(char_2).or_default() += count;
    }

    let twice_most_common = counts.values().max().unwrap();
    let twice_least_common = counts.values().min().unwrap();
    (twice_most_common - twice_least_common) / 2
}
