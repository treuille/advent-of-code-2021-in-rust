use splr::Certificate;
use std::clone::Clone;
use std::collections::HashMap;
use std::iter;
use std::ops::Neg;

/// All the potential propositions in this puzzle.
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum Proposition {
    /// True if `pattern` represents digit `digit`.
    PatternIsDigit { pattern: u8, digit: u8 },

    /// Negation of PatternIsDigit
    PatternIsNotDigit { pattern: u8, digit: u8 },

    /// True if `wire` maps to `segment`.
    WireIsSegment { wire: char, segment: char },

    /// Negation of WireIsSegment
    WireIsNotSegment { wire: char, segment: char },
}

struct Entry {
    /// Map from Literals to thier index in self.clauses
    literals: HashMap<Proposition, i32>,

    /// The clauses of this entry in conjunctive normal form.
    clauses: Vec<Vec<i32>>,
}

/// One line of the puzzle
impl Entry {
    fn new() -> Self {
        let mut myself = Self {
            literals: HashMap::new(),
            clauses: Vec::new(),
        };

        // Each pattern must represent *exactly* one digit.
        myself.create_bijection(0..10, |pattern, digit| Proposition::PatternIsNotDigit {
            pattern,
            digit,
        });

        // Each wire must represent *exactly* one segment.
        myself.create_bijection('a'..='g', |wire, segment| Proposition::WireIsNotSegment {
            wire,
            segment,
        });

        // Each wire must represent at least one segment.. DELETE THIS
        for wire in 'a'..='g' {
            myself.add_clause(
                &('a'..='g')
                    .map(|segment| Proposition::WireIsSegment { wire, segment })
                    .collect::<Vec<_>>(),
            );
        }

        myself
    }

    /// Adds a clause to this entry
    fn add_clause(&mut self, clause: &[Proposition]) {
        let clause_indices = clause.iter().map(|p| self.get_index(p)).collect();
        self.clauses.push(clause_indices);
    }

    /// Returns a vector of propositions which solves this entry.
    fn solve(self) -> Vec<Proposition> {
        match Certificate::try_from(self.clauses).unwrap() {
            Certificate::UNSAT => {
                panic!("Not satisfied.");
            }
            Certificate::SAT(soln) => {
                // Invert the literals table
                let mut indices: HashMap<i32, Proposition> =
                    HashMap::from_iter(self.literals.into_iter().map(|(k, v)| (v, k)));
                soln.into_iter()
                    .filter_map(|index| indices.remove(&index))
                    .collect()
            }
        }
    }

    /// Converts a literal to an i32 index.
    fn get_index(&mut self, literal: &Proposition) -> i32 {
        match literal {
            &Proposition::PatternIsNotDigit { pattern, digit } => self
                .get_index(&Proposition::PatternIsDigit { pattern, digit })
                .neg(),
            &Proposition::WireIsNotSegment { wire, segment } => self
                .get_index(&Proposition::WireIsSegment { wire, segment })
                .neg(),
            literal => {
                let next_index = self.literals.len() + 1;
                *self
                    .literals
                    .entry(literal.clone())
                    .or_insert(next_index as i32)
            }
        }
    }

    fn create_bijection<T, R, F>(&mut self, range: R, to_proposition: F)
    where
        T: PartialEq + Eq + Copy,
        R: Iterator<Item = T> + Clone,
        F: Fn(T, T) -> Proposition,
    {
        // There cannot be two arrows from any element in the domain
        for x in range.clone() {
            for y1 in range.clone() {
                for y2 in range.clone() {
                    if y1 != y2 {
                        self.add_clause(&[to_proposition(x, y1), to_proposition(x, y2)]);
                    }
                }
            }
        }

        // There cannot be two arrows into any element in the domain
        for x1 in range.clone() {
            for x2 in range.clone() {
                if x1 != x2 {
                    for y in range.clone() {
                        self.add_clause(&[to_proposition(x1, y), to_proposition(x2, y)]);
                    }
                }
            }
        }
    }
}

/// Which segments are shown for each segment.
const DIGIT_SEGMENTS: [&str; 10] = [
    "abcefg", "cf", "acdeg", "acdfg", "bcdf", "abdfg", "abdefg", "acf", "abcdefg", "abcdfg",
];

/// Sort the characters in a string.
fn sort_chars(s: &str) -> String {
    let mut s: Vec<char> = s.chars().collect();
    s.sort_unstable();
    s.iter().collect()
}

fn main() {
    let digits: Vec<Vec<u8>> = solve_for_digits();
    println!("digits: {digits:?}");

    println!("Puzzle 8a: {} (387)", solve_8a(&digits));
    println!("Puzzle 8b: {} (986034)", solve_8b(&digits));
}

fn solve_8a(digits: &[Vec<u8>]) -> usize {
    digits
        .iter()
        .flatten()
        .filter(|&&x| x == 1 || x == 4 || x == 7 || x == 8)
        .count()
}

fn solve_8b(digits: &[Vec<u8>]) -> usize {
    digits
        .iter()
        .map(|digits| {
            digits
                .iter()
                .fold(0usize, |result, &digit| result * 10 + (digit as usize))
        })
        .sum()
}

fn solve_for_digits() -> Vec<Vec<u8>> {
    let len_to_digits: HashMap<usize, Vec<u8>> = HashMap::from_iter((2..=7).map(|n_segments| {
        (
            n_segments,
            DIGIT_SEGMENTS
                .iter()
                .enumerate()
                .filter_map(|pair| match pair {
                    (i, s) if s.len() == n_segments => Some(i as u8),
                    _ => None,
                })
                .collect(),
        )
    }));

    include_str!("../../puzzle_inputs/day_8.txt")
        .lines()
        .enumerate()
        .map(|(line_no, line)| {
            let (patterns, output) = line.split_once("|").unwrap();
            let patterns: Vec<&str> = patterns.split_whitespace().collect();
            let output: Vec<&str> = output.split_whitespace().collect();
            println!("{line_no} patterns: {patterns:?}");
            println!("{line_no} output: {output:?}");

            // Setup the SAT puzzle.
            let mut entry = Entry::new();
            for (pattern, chars) in patterns.iter().enumerate() {
                let pattern = pattern as u8;
                println!("{pattern} -> {chars}");
                let mut potential_digits = Vec::new();
                for &digit in len_to_digits[&chars.len()].iter() {
                    potential_digits.push(Proposition::PatternIsDigit { pattern, digit });
                    let segments = DIGIT_SEGMENTS[digit as usize];
                    for wire in chars.chars() {
                        entry.add_clause(&Vec::from_iter(
                            segments
                                .chars()
                                .map(|segment| Proposition::WireIsSegment { wire, segment })
                                .chain(iter::once({
                                    Proposition::PatternIsNotDigit { pattern, digit }
                                })),
                        ));
                    }
                }
                entry.add_clause(&potential_digits);
            }

            // Solve the SAT puzzle.
            let soln = entry.solve();
            for prop in soln.iter() {
                println!("{prop:?}");
            }

            // Create a mapping from characters to digits
            let digit_map: HashMap<String, u8> =
                HashMap::from_iter(soln.iter().filter_map(|prop| match prop {
                    Proposition::PatternIsDigit { pattern, digit } => {
                        Some((sort_chars(patterns[*pattern as usize]), *digit))
                    }
                    _ => None,
                }));
            output
                .iter()
                .map(|chars| digit_map[&sort_chars(chars)])
                .collect()
        })
        .collect()
}
