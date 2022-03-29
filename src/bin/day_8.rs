use splr::Certificate;
use std::clone::Clone;
use std::collections::HashMap;
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

/// One line of the puzzle
struct Entry {
    /// Map from Literals to thier index in self.clauses
    literals: HashMap<Proposition, i32>,

    /// The clauses of this entry in conjunctive normal form.
    clauses: Vec<Vec<i32>>,
}

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

        myself
    }

    /// Adds a clause to this entry
    fn add_clause(&mut self, clause: &[Proposition]) {
        let clause_indices = clause.iter().map(|p| self.get_index(p)).collect();
        self.clauses.push(clause_indices);
    }

    /// Returns a vector of propositions which solves this entry.
    fn solve(self) -> Vec<Proposition> {
        if let Ok(Certificate::SAT(soln)) = Certificate::try_from(self.clauses) {
            // Invert the literals table
            let mut indices: HashMap<i32, Proposition> =
                HashMap::from_iter(self.literals.into_iter().map(|(k, v)| (v, k)));
            soln.into_iter()
                .filter_map(|index| indices.remove(&index))
                .collect()
        } else {
            panic!("Not satisfied.")
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
        // There cannot be two arrows to or from any element in the domain
        for x1 in range.clone() {
            for x2 in range.clone() {
                if x1 != x2 {
                    for x3 in range.clone() {
                        self.add_clause(&[to_proposition(x3, x1), to_proposition(x3, x2)]);
                        self.add_clause(&[to_proposition(x1, x3), to_proposition(x2, x3)]);
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
        .map(|line| {
            let (patterns, output) = line.split_once("|").unwrap();
            let patterns: Vec<&str> = patterns.split_whitespace().collect();
            let output: Vec<&str> = output.split_whitespace().collect();

            // Setup the SAT puzzle.
            let mut entry = Entry::new();
            for (pattern, chars) in patterns.iter().enumerate() {
                let pattern = pattern as u8;
                let mut potential_digits = Vec::new();
                for &digit in len_to_digits[&chars.len()].iter() {
                    potential_digits.push(Proposition::PatternIsDigit { pattern, digit });
                    let segments = DIGIT_SEGMENTS[digit as usize];
                    for wire in chars.chars() {
                        let mut implied_segments =
                            vec![Proposition::PatternIsNotDigit { pattern, digit }];
                        for segment in segments.chars() {
                            implied_segments.push(Proposition::WireIsSegment { wire, segment });
                        }
                        entry.add_clause(&implied_segments);
                    }
                }
                entry.add_clause(&potential_digits);
            }

            // Solve the SAT puzzle to create a mapping from characters to digits
            let soln = entry.solve();
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
