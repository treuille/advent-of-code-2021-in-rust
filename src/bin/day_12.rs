// use regex::Regex;
use std::collections::{HashMap, HashSet};

fn main() {
    let puzzle = read_input(include_str!("../../puzzle_inputs/day_12.txt"));
    println!("12a: {} (5178)", solve_12a(&puzzle));
    println!("12b: {} (130094)", solve_12b(&puzzle));
}

fn read_input(input: &str) -> Puzzle {
    let mut puzzle: Puzzle = Puzzle::new();
    for line in input.trim().lines() {
        let (a, b) = line.split_once("-").unwrap();
        puzzle.entry(a).or_default().insert(b);
        puzzle.entry(b).or_default().insert(a);
    }
    puzzle
}

fn solve_12a(puzzle: &Puzzle) -> usize {
    Explorer::new(puzzle, false).explore("start")
}

fn solve_12b(puzzle: &Puzzle) -> usize {
    Explorer::new(puzzle, true).explore("start")
}

/// Each cave is denoted by a string slice identifier.
type Cave<'a> = &'a str;

/// The puzzle input is a network of interconnected Caves.
type Puzzle<'a> = HashMap<Cave<'a>, HashSet<Cave<'a>>>;

/// Stores the state of our exploration.
struct Explorer<'a> {
    /// The puzzle we're exploring.
    puzzle: &'a Puzzle<'a>,

    /// Which caves we're exploring.
    visited: HashSet<Cave<'a>>,

    /// Whether we've visitd this small cave twice
    visited_twice: Option<Cave<'a>>,

    /// Whether we can visit a small cave twice.
    can_visit_twice: bool,
}

impl<'a> Explorer<'a> {
    fn new(puzzle: &'a Puzzle, can_visit_twice: bool) -> Self {
        Self {
            puzzle,
            visited: HashSet::new(),
            visited_twice: None,
            can_visit_twice,
        }
    }

    fn explore(&mut self, cave: Cave) -> usize {
        let is_small = |cave: &str| cave.chars().all(|c| ('a'..='z').contains(&c));
        let mut paths = 0;
        for next_cave in self.puzzle.get(&cave).unwrap() {
            match *next_cave {
                "start" => (),
                "end" => paths += 1,
                next_cave if is_small(next_cave) => {
                    if !self.visited.contains(next_cave) {
                        self.visited.insert(next_cave);
                        paths += self.explore(next_cave);
                        self.visited.remove(next_cave);
                    } else if self.can_visit_twice && self.visited_twice.is_none() {
                        self.visited_twice = Some(next_cave);
                        paths += self.explore(next_cave);
                        self.visited_twice = None;
                    }
                }
                next_cave => paths += self.explore(next_cave),
            }
        }
        paths
    }
}
