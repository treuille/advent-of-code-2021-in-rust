use itertools::{self, Itertools};
use std::collections::HashMap;

const TEST_INPUT: &str = "
NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C
";

fn main() {
    let (mut polymer, xform) = read_input(include_str!("../../puzzle_inputs/day_14.txt"));
    // let (mut polymer, xform) = read_input(TEST_INPUT);

    // let ref_counts: Vec<HashMap<char, usize>> = [
    //     "NNCB",
    //     "NCNBCHB",
    //     "NBCCNBBBCBHCB",
    //     "NBBBCNCCNBBNBNBBCHBHHBCHB",
    //     "NBBNBNBBCCNBCNCCNBBNBBNBBBNBBNBBCBHCBHHNHCBBCBHCB",
    // ]
    // .into_iter()
    // .map(|s| {
    //     let mut counts = HashMap::new();
    //     s.chars().for_each(|c| *counts.entry(c).or_default() += 1);
    //     counts
    // })
    // .collect();

    // println!("counts 0 : {:?}", ref_counts[0]);
    // println!("counts 1 : {:?}", ref_counts[1]);
    // println!("counts 2 : {:?}", ref_counts[2]);
    // println!("counts 3 : {:?}", ref_counts[3]);
    // println!("counts 4 : {:?}", ref_counts[4]);
    // println!("len : {:?}", ref_counts.len());

    for i in 0..40 {
        // println!("iteration: {i}");
        // if let Some(ref_counts) = ref_counts.get(i) {
        //     println!("polymer: {polymer:?}");
        //     assert_eq!(char_counts(&polymer), *ref_counts);
        // }
        polymer = xform.apply(polymer);
        // println!("polymer: {polymer:?}");
        // let counts = char_counts(&polymer);
        // println!("counts: {counts:?}");
        // panic!("testing");
    }

    let counts = char_counts(&polymer);
    println!("counts: {counts:?}");

    let most_common = counts.values().max().unwrap();
    let least_common = counts.values().min().unwrap();
    println!("answer: {}", most_common - least_common);
    // for (i, line) in read_input().into_iter().enumerate() {
    //     println!("{i}: \"{line}\"");
    // }
    // println!("14a: {} (123)", solve_14a());
    // println!("14b: {} (456)", solve_14b());
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
    let (polymer, rules) = input.trim().split_once("\n\n").unwrap();
    println!("polymer: {polymer}");

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
                (from, vec![((from.0, to), 1), ((to, from.1), 1)])
            })
            .collect(),
    );

    // All done
    (polymer, xform)
}

// fn solve_14a() -> usize {
//     123
// }

// fn solve_14b() -> usize {
//     456
// }

impl Transform {
    // TODO: This should be rewritten with for loops
    fn apply(&self, input: Polymer) -> Polymer {
        let mut output = Polymer::new();
        input.iter().for_each(|(pair_1, count_1)| {
            if let Some(new_pairs) = self.0.get(pair_1) {
                new_pairs.iter().for_each(|(pair_2, count_2)| {
                    *output.entry(*pair_2).or_default() += count_1 * count_2
                });
            }
        });
        output
    }
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
