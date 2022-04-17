use itertools::{self, Itertools};
use std::collections::HashMap;

fn main() {
    let (mut polymer, xform) = read_input(include_str!("../../puzzle_inputs/day_14.txt"));

    for _ in 0..40 {
        polymer = apply(&xform, polymer);
    }

    let counts = char_counts(&polymer);
    let most_common = counts.values().max().unwrap();
    let least_common = counts.values().min().unwrap();
    let answer = most_common - least_common;
    assert_eq!(answer, 12271437788530);
    println!("answer: {}", answer);
}

/// A pair is a lenght two string.
type Pair = (char, char);

/// A polymer consists of a counf for a set of length-2 strings.
type Polymer = HashMap<Pair, usize>;

/// A transform converts a polymer into another.
type Transform = HashMap<Pair, Vec<(Pair, usize)>>;

/// Read the input file and turn it into an Array2<u8>
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
            ((a, b), vec![((a, c), 1), ((c, b), 1)])
        })
        .collect();

    // All done
    (polymer, xform)
}

// fn solve_14a() -> usize {
//     123
// }

// fn solve_14b() -> usize {
//     456
// }

// TODO: This should be rewritten with for loops
fn apply(xform: &Transform, input: Polymer) -> Polymer {
    let mut output = Polymer::new();
    for (pair_1, count_1) in input.iter() {
        if let Some(new_pairs) = xform.get(pair_1) {
            for (pair_2, count_2) in new_pairs {
                *output.entry(*pair_2).or_default() += count_1 * count_2
            }
        }
    }
    output
}

/// Returns a hashtable of the count of each character in this polymer.
fn char_counts(poly: &Polymer) -> HashMap<char, usize> {
    let mut counts: HashMap<char, usize> = HashMap::new();
    for (&(char_1, char_2), count) in poly.iter() {
        *counts.entry(char_1).or_default() += count;
        *counts.entry(char_2).or_default() += count;
    }

    counts
        .drain()
        .map(|(c, count)| (c, (count + 1) / 2))
        .collect()
}
