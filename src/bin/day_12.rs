use regex::Regex;
use std::collections::{HashMap, HashSet};

type Puzzle<'a> = HashMap<&'a str, HashSet<&'a str>>;

#[allow(dead_code)]
const TEST_INPUT_1: &str = "
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
const TEST_INPUT_2: &str = "
fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW
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
    let puzzle: Puzzle = read_input(PUZZLE_INPUT);
    println!("puzzle: {puzzle:?}");

    // for (&cave, _) in puzzle.iter() {
    //     println!("{cave} -> {}", is_small(cave));
    // }
    // panic!("done!");

    // let small_caves_visited: HashSet<&str> = ;
    let paths = explore("start", &mut HashSet::new(), &puzzle);
    println!("founds {paths} paths.");

    // // println!("10a: {} (123)", solve_10a());
    // // println!("10b: {} (456)", solve_10b());
}

fn explore<'a>(
    cave: &'a str,
    small_caves_visited: &mut HashSet<&'a str>,
    puzzle: &Puzzle<'a>,
) -> usize {
    // println!("exploring {cave} ({})", small_caves_visited.len());
    if cave == "end" {
        return 1;
    }
    let is_small: bool = cave.chars().all(|c| ('a'..='z').contains(&c));
    if is_small {
        small_caves_visited.insert(cave);
    }
    let paths = puzzle
        .get(cave)
        .unwrap()
        .iter()
        .filter_map(
            |&next_cave| match &small_caves_visited.contains(next_cave) {
                false => Some(explore(next_cave, small_caves_visited, puzzle)),
                true => None,
            },
        )
        .sum();
    if is_small {
        small_caves_visited.remove(cave);
    }
    paths
}

// fn is_small(cave: &str) -> bool {
//     // cave.chars().all(|c| c >= 'a' && c <= 'z')
//     cave.chars().all(|c| ('a'..='z').contains(&c))
// }

fn solve_10a() -> usize {
    123
}

fn solve_10b() -> usize {
    456
}

/// Read the input file and turn it into an Array2<u8>
fn read_input(input: &str) -> Puzzle {
    let re = Regex::new(r"([[:alpha:]]+)-([[:alpha:]]+)").unwrap();
    let mut puzzle: Puzzle = Puzzle::new();
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
