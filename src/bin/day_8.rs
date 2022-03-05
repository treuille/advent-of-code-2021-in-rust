use splr::Certificate;
use std::collections::{HashMap, HashSet};

// digit -> segments
// 0 -> 6
// 1 -> 2
// 2 -> 5
// 3 -> 5
// 4 -> 4
// 5 -> 5
// 6 -> 6
// 7 -> 3
// 8 -> 7
// 9 -> 6

/// All the potential propositions in this puzzle.
enum Proposition {
    /// True if `pattern` represents digit `digit`.
    PatternIsDigit { pattern: u8, digit: u8 },

    /// True if `wire` maps to `segment`.
    WireIsSegment { wire: char, segment: char },
}

struct Puzzle {}

/// An mapping from puzzles to 
impl Puzzle {
    fn new()
}

// /// Gets a mapping from digits to which segements the encompass.
// fn get_digits_to_segemets() -> HashMap<u8, HashSet<char>> {
//     HashMap::from([
//         (0u8, HashSet::from(['a', 'b', 'c', 'e', 'f', 'g']),
//         (1u8, HashSet::from(['c', 'f']),
//         (2u8, HashSet::from(['a', 'c', 'd', 'e', 'g']),
//         (3u8, HashSet::from(['a', 'c', 'd', 'f', 'g']),
//         (4u8, HashSet::from(['b', 'c', 'd', 'f']),
//         (5u8, HashSet::from(['a', 'b', 'd', 'f', 'g']),
//         (6u8, HashSet::from(['a', 'b', 'd', 'e', 'f', 'g']),
//         (7u8, HashSet::from(['a', 'c', 'f']),
//         (8u8, HashSet::from(['a', 'b', 'c', 'd', 'e', 'f', 'g']),
//         (9u8, HashSet::from(['a', 'b', 'c', 'd', 'f', 'g'])),
//     ])
// }

/// Gets a mapping from digits to which segements the encompass.
fn get_digits_to_segemets() -> HashMap<u8, &'static [char]> {
    HashMap::from([
        (0u8, &['a', 'b', 'c', 'e', 'f', 'g'][..]),
        (1u8, &['c', 'f'][..]),
        (2u8, &['a', 'c', 'd', 'e', 'g'][..]),
        (3u8, &['a', 'c', 'd', 'f', 'g'][..]),
        (4u8, &['b', 'c', 'd', 'f'][..]),
        (5u8, &['a', 'b', 'd', 'f', 'g'][..]),
        (6u8, &['a', 'b', 'd', 'e', 'f', 'g'][..]),
        (7u8, &['a', 'c', 'f'][..]),
        (8u8, &['a', 'b', 'c', 'd', 'e', 'f', 'g'][..]),
        (9u8, &['a', 'b', 'c', 'd', 'f', 'g'][..]),
    ])
}

fn main() {
    let digits_to_segments = get_digits_to_segemets();
    for digit in 0u8..=9u8 {
        println!("{digit} -> {}", digits_to_segments[&digit].len());
    }

    let segements_to_digits: HashMap<&[char], u8> =
        HashMap::from_iter(digits_to_segments.iter().map(|(&k, &v)| (v, k)));

    println!("segements_to_digits: {segements_to_digits:?}");

    // let v: Vec<Vec<i32>> = vec![vec![-1], vec![1, 2], vec![-1, 3], vec![1, -3], vec![-1, 2]];
    // match Certificate::try_from(v).unwrap() {
    //     Certificate::UNSAT => {
    //         println!("Not satisfied.");
    //     }
    //     Certificate::SAT(vec) => {
    //         println!("Satisfied: {:?}", vec);
    //     }
    // };

    let count = include_str!("../../puzzle_inputs/day_8.txt")
        .lines()
        .fold(0, |sum, line| {
            println!("{}", line);
            let (patterns, output) = line.split_once("|").unwrap();
            println!("patterns: {patterns}");
            println!("output: {output}");
            let blah: Vec<_> = output.split_whitespace().collect();
            println!("blah: {blah:?}");
            panic!("First iteration.");

            // For each pattern:
            // 1. Each `pattern` has to be a `digit` 0-9
            // 2. No `pattern` can be two `digit`s

            // For each wire
            // 1. Each wire has to be one segment
            // 2. No wire can be two segments
            // 3. If the `pattern` is `digit`, and the pattern contains `wire,`
            //    then `wire` must be a segment in `digit`
            //    aka pattern is digit => (for each wire in pattern)
            //        (for all segments s1, s2, ... in digit) wire is s1 || wire is s2
            //    aka for each wire in pattern
            //           for all segments s NOT IN digit
            //              pattern is digit => wire IS NOT segment
            //    aka for each wire in pattern
            //           for all segments s NOT IN digit
            //              pattern IS NOT digit || wire IS NOT segment
            //

            // let count = output
            //     .split_whitespace()
            //     .filter(|&s| {
            //         let x = s.len();
            //         x == 2 || x == 3 || x == 4 || x == 7
            //     })
            //     .count();
            // sum + count
        });
    println!("count: {count}");
}

#[allow(dead_code)]
fn puzzle_8a() {
    let count = include_str!("../../puzzle_inputs/day_8.txt")
        .lines()
        .fold(0, |sum, line| {
            println!("{}", line);
            let (_, output) = line.split_once("|").unwrap();
            println!("output: {output}");
            // println!("patterns: {patterns}");
            let blah: Vec<_> = output.split_whitespace().collect();
            println!("blah: {blah:?}");
            let count = output
                .split_whitespace()
                .filter(|&s| {
                    let x = s.len();
                    x == 2 || x == 3 || x == 4 || x == 7
                })
                .count();
            sum + count
        });
    println!("count: {count}");
}
