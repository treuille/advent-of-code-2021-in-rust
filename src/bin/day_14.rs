use itertools::{self, Itertools};
use std::collections::HashMap;

fn main() {
    let (polymer, xform) = read_input(include_str!("../../puzzle_inputs/day_14.txt"));

    // for (i, line) in read_input().into_iter().enumerate() {
    //     println!("{i}: \"{line}\"");
    // }
    println!("14a: {} (123)", solve_14a());
    println!("14b: {} (456)", solve_14b());
}

/// A pair is a lenght two string.
type Pair = (char, char);

/// A polymer consists of a counf for a set of length-2 strings.
type Polymer = HashMap<Pair, usize>;

/// A transform converts a polymer into another.
#[derive(Debug)]
struct Transform(HashMap<Pair, Vec<(Pair, usize)>>);

/// Read the input file and turn it into an Array2<u8>
fn read_input(input: &str) -> (Polymer, Transform) {
    // The input consists of two section: the polymer string, and a list of rules.
    let (polymer, rules) = input.split_once("\n\n").unwrap();

    // Parse the polymer string
    let (mut polymer, pairs) = (Polymer::new(), polymer.chars().tuple_windows());
    pairs.for_each(|pair| *polymer.entry(pair).or_default() += 1);

    // Parse the set of rules into a transformation.
    let xform = Transform(
        rules
            .lines()
            .map(|line| {
                let (from, to) = line.split_once(" -> ").unwrap();
                let from: Pair = from.chars().tuple_windows().next().unwrap();
                let to = to.chars().next().unwrap();
                (from, vec![((from.0, to), 1), ((from.1, to), 1)])
            })
            .collect(),
    );

    // All done
    (polymer, xform)
}

fn solve_14a() -> usize {
    123
}

fn solve_14b() -> usize {
    456
}

// impl Transform {
//     fn apply(&self, polymer: Polymer) -> Polymer {}
// }
