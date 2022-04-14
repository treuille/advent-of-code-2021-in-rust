use regex::Regex;
use std::collections::{HashMap, HashSet};

#[allow(dead_code)]
const TEST_INPUT: &str = "
dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc
";

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

fn main() {
    let puzzle = read_input(TEST_INPUT);
    println!("puzzle: {puzzle:?}");
    println!("10a: {} (123)", solve_10a());
    println!("10b: {} (456)", solve_10b());
}

fn solve_10a() -> usize {
    123
}

fn solve_10b() -> usize {
    456
}

/// Read the input file and turn it into an Array2<u8>
fn read_input(input: &str) -> HashMap<&str, HashSet<&str>> {
    let re = Regex::new(r"([[:alpha:]]+)-([[:alpha:]]+)").unwrap();
    let mut puzzle: HashMap<&str, HashSet<&str>> = HashMap::new();
    for (i, line) in input.trim().lines().enumerate() {
        println!("- {i} {line}");
        let caps = re.captures(line).unwrap();
        let cave_a = caps.get(1).unwrap().as_str();
        let cave_b = caps.get(2).unwrap().as_str();
        puzzle.entry(cave_a).or_default().insert(cave_b);
        puzzle.entry(cave_b).or_default().insert(cave_a);
    }
    puzzle
}
