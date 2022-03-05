use splr::Certificate;
use std::clone::Clone;
use std::collections::HashMap;
use std::ops::Neg;

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
#[derive(Debug, PartialEq, Eq)]
enum Proposition {
    /// True if `pattern` represents digit `digit`.
    PatternIsDigit { pattern: u8, digit: u8 },

    /// True if `wire` maps to `segment`.
    WireIsSegment { wire: char, segment: char },
}

impl Proposition {
    const MAX_INDEX: i32 = 149;

    /// The the index representing a proposition.
    fn to_index(&self) -> i32 {
        match *self {
            Proposition::PatternIsDigit { pattern, digit } => {
                let pattern = pattern as i32;
                let digit = digit as i32;
                10 * pattern + digit + 1
            }
            Proposition::WireIsSegment { wire, segment } => {
                let a = 'a' as i32;
                let wire = (wire as i32) - a;
                let segment = (segment as i32) - a;
                7 * wire + segment + 101
            }
        }
    }

    /// The the index representing the negation of a proposition.
    fn negation_to_index(&self) -> i32 {
        self.to_index().neg()
    }

    /// Converts a positive index back into a proposition
    fn from_index(index: i32) -> Self {
        if index <= 0 {
            panic!("Indices must be positive.");
        } else if index <= 100 {
            Proposition::PatternIsDigit {
                pattern: ((index - 1) / 10) as u8,
                digit: ((index - 1) % 10) as u8,
            }
        } else if index <= Proposition::MAX_INDEX {
            let a = 'a' as i32;
            Proposition::WireIsSegment {
                wire: ((((index - 101) / 7) + a) as u8) as char,
                segment: ((((index - 101) % 7) + a) as u8) as char,
            }
        } else {
            panic!("Index {} is too high.", index);
        }
    }
}

struct Entry {
    /// The clauses of this entry in conjunctive normal form.
    clauses: Vec<Vec<i32>>,
}

/// One line of the puzzle
impl Entry {
    fn new() -> Self {
        let mut myself = Self {
            clauses: Vec::new(),
        };
        // Each pattern must represent *exactly* one digit.
        myself.create_bijection(0..10, |pattern, digit| Proposition::PatternIsDigit {
            pattern,
            digit,
        });

        // Each wire must represent *exactly* one segment.
        myself.create_bijection('a'..='g', |wire, segment| Proposition::WireIsSegment {
            wire,
            segment,
        });

        myself
    }

    fn create_bijection<T, R, F>(&mut self, range: R, to_proposition: F)
    where
        T: PartialEq + Eq + Copy,
        R: Iterator<Item = T> + Clone,
        F: Fn(T, T) -> Proposition,
    {
        // There must be at least one arrow from every element in the domain.
        for x in range.clone() {
            self.clauses.push(
                range
                    .clone()
                    .map(|y| to_proposition(x, y).to_index())
                    .collect(),
            );
        }

        // There cannot be two arrows from any element in the domain
        for x in range.clone() {
            for y1 in range.clone() {
                for y2 in range.clone() {
                    if y1 != y2 {
                        self.clauses.push(vec![
                            to_proposition(x, y1).negation_to_index(),
                            to_proposition(x, y2).negation_to_index(),
                        ]);
                    }
                }
            }
        }

        // There cannot be two arrows into any element in the domain
        for x1 in range.clone() {
            for x2 in range.clone() {
                if x1 != x2 {
                    for y in range.clone() {
                        self.clauses.push(vec![
                            to_proposition(x1, y).negation_to_index(),
                            to_proposition(x2, y).negation_to_index(),
                        ]);
                    }
                }
            }
        }
    }

    /// Returns a vector of propositions which solves this entry.
    fn solve(&self) -> Vec<Proposition> {
        match Certificate::try_from(self.clauses.clone()).unwrap() {
            Certificate::UNSAT => {
                panic!("Not satisfied.");
            }
            Certificate::SAT(soln) => soln
                .iter()
                .filter_map(|&index| {
                    if index > 0 {
                        Some(Proposition::from_index(index))
                    } else {
                        None
                    }
                })
                .collect(),
        }
    }
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
    include_str!("../../puzzle_inputs/day_8.txt")
        .lines()
        .for_each(|line| {
            let (patterns, output) = line.split_once("|").unwrap();
            println!("patterns: {patterns}");
            println!("output: {output}");
            let blah: Vec<_> = output.split_whitespace().collect();
            println!("blah: {blah:?}");

            let entry = Entry::new();
            for (pattern, chars) in patterns.split_whitespace().enumerate() {
                println!("{pattern} -> {chars}");
                for digit in 0..10 {}
            }
            //
            // let soln = entry.solve();
            // println!("There are {} solutions.", soln.len());
            // for prop in soln {
            //     println!("{prop:?}");
            // }
            // panic!("Just playing around.");

            panic!("First iteration.");
        });
    //.fold(0, |sum, line| {
    //    println!("{}", line);

    //    // For each pattern:
    //    // 1. Each `pattern` has to be a `digit` 0-9
    //    // 2. No `pattern` can be two `digit`s

    //    // For each wire
    //    // 1. Each wire has to be one segment
    //    // 2. No wire can be two segments
    //    // 3. If the `pattern` is `digit`, and the pattern contains `wire,`
    //    //    then `wire` must be a segment in `digit`
    //    //    aka pattern is digit => (for each wire in pattern)
    //    //        (for all segments s1, s2, ... in digit) wire is s1 || wire is s2
    //    //    aka for each wire in pattern
    //    //           for all segments s NOT IN digit
    //    //              pattern is digit => wire IS NOT segment
    //    //    aka for each wire in pattern
    //    //           for all segments s NOT IN digit
    //    //              pattern IS NOT digit || wire IS NOT segment
    //    //

    //    // let count = output
    //    //     .split_whitespace()
    //    //     .filter(|&s| {
    //    //         let x = s.len();
    //    //         x == 2 || x == 3 || x == 4 || x == 7
    //    //     })
    //    //     .count();
    //    // sum + count
    //});

    // println!("count: {count}");
    // let digits_to_segments = get_digits_to_segemets();
    // for digit in 0u8..=9u8 {
    //     println!("{digit} -> {}", digits_to_segments[&digit].len());
    // }

    //let segements_to_digits: HashMap<&[char], u8> =
    //    HashMap::from_iter(digits_to_segments.iter().map(|(&k, &v)| (v, k)));
    //println!("segements_to_digits: {segements_to_digits:?}");
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /// Test that convert from propositions to indices and back works.
    fn propositions_to_indices() {
        for pattern in 0u8..10u8 {
            for digit in 0u8..10u8 {
                let prop = Proposition::PatternIsDigit { pattern, digit };
                assert_eq!(prop, Proposition::from_index(prop.to_index()));
            }
        }

        for wire in 'a'..='g' {
            for segment in 'a'..='g' {
                let prop = Proposition::WireIsSegment { wire, segment };
                assert_eq!(prop, Proposition::from_index(prop.to_index()));
            }
        }
    }

    #[test]
    /// Test that convert from indices to propositions and back works.
    fn indices_to_propositions() {
        for index in 1..=Proposition::MAX_INDEX {
            let prop = Proposition::from_index(index);
            assert_eq!(index, prop.to_index());
            assert_eq!(index.neg(), prop.negation_to_index());
        }
    }
}
