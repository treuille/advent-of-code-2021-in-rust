use regex::Regex;
use std::collections::{HashMap, HashSet};

fn main() {
    let puzzle = read_input(PUZZLE_INPUT);
    println!("12a: {} (5178)", solve_12a(&puzzle));
    println!("12b: {} (130094)", solve_12b(&puzzle));
}

fn read_input(input: &str) -> Puzzle {
    let re = Regex::new(r"([[:alpha:]]+)-([[:alpha:]]+)").unwrap();
    let mut puzzle: Puzzle = Puzzle::new();
    for line in input.trim().lines() {
        let caps = re.captures(line).unwrap();
        let a = Cave::from_str(caps.get(1).unwrap().as_str());
        let b = Cave::from_str(caps.get(2).unwrap().as_str());
        puzzle.entry(a.clone()).or_default().insert(b.clone());
        puzzle.entry(b.clone()).or_default().insert(a.clone());
    }
    puzzle
}

fn solve_12a(puzzle: &Puzzle) -> usize {
    Explorer::new(puzzle, false).explore(Cave::Start)
}

fn solve_12b(puzzle: &Puzzle) -> usize {
    Explorer::new(puzzle, true).explore(Cave::Start)
}

/// The types of caves in the puzzle.
#[derive(PartialEq, Eq, Hash, Clone)]
enum Cave {
    Start,
    End,
    Small(String),
    Big(String),
}

impl Cave {
    fn from_str(cave: &str) -> Self {
        let is_small = |cave: &str| cave.chars().all(|c| ('a'..='z').contains(&c));
        match cave {
            "start" => Self::Start,
            "end" => Self::End,
            cave if is_small(cave) => Self::Small(cave.to_string()),
            cave => Self::Big(cave.to_string()),
        }
    }
}

/// The puzzle input is a network of interconnected Caves.
type Puzzle = HashMap<Cave, HashSet<Cave>>;

struct Explorer<'a> {
    /// The puzzle we're exploring.
    puzzle: &'a Puzzle,

    /// Which caves we're exploring.
    visited: HashSet<Cave>,

    /// Whether we've visitd this small cave twice
    visited_twice: Option<Cave>,

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
        let mut paths = 0;
        for next_cave in self.puzzle.get(&cave.clone()).unwrap() {
            match next_cave {
                Cave::Start => (),
                Cave::End => paths += 1,
                Cave::Big(_) => paths += self.explore(next_cave.clone()),
                Cave::Small(_) => {
                    if !self.visited.contains(&next_cave) {
                        self.visited.insert(next_cave.clone());
                        paths += self.explore(next_cave.clone());
                        self.visited.remove(&next_cave);
                    } else if self.can_visit_twice && self.visited_twice.is_none() {
                        self.visited_twice = Some(next_cave.clone());
                        paths += self.explore(next_cave.clone());
                        self.visited_twice = None;
                    }
                }
            }
        }
        paths
    }
}

#[allow(dead_code)]
const PUZZLE_INPUT: &str = "
start-qs
qs-jz
start-lm
qb-QV
QV-dr
QV-end
ni-qb
VH-jz
qs-lm
qb-end
dr-fu
jz-lm
start-VH
QV-jz
VH-qs
lm-dr
dr-ni
ni-jz
lm-QV
jz-dr
ni-end
VH-dr
VH-ni
qb-HE
";
