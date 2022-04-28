// use aoc::to_tuple::ToTuple;
// use regex::Regex;

fn main() {
    todo!("Redo with the new crate.");
    // println!("Puzzle 2a: {:?}", solve_puzzle(advance_a));
    // println!("Puzzle 2b: {:?}", solve_puzzle(advance_b));
}

// type State = (isize, isize, isize);

// fn solve_puzzle<F>(advance: F) -> isize
// where
//     F: Fn(State, &str, isize) -> State,
// {
//     let re = Regex::new(r"([a-z]+) (\d)").unwrap();
//     let (x, y, _) = re
//         .parse_lines(include_str!("../../puzzle_inputs/day_02.txt"))
//         .fold((0, 0, 0), |state, (dir, dist)| advance(state, dir, dist));
//     x * y
// }

// fn advance_a((x, y, _): State, direction: &str, distance: isize) -> State {
//     match direction {
//         "forward" => (x + distance, y, 0),
//         "up" => (x, y - distance, 0),
//         "down" => (x, y + distance, 0),
//         _ => panic!("Invalid direction"),
//     }
// }

// fn advance_b((x, y, aim): State, direction: &str, distance: isize) -> State {
//     match direction {
//         "forward" => (x + distance, y + distance * aim, aim),
//         "up" => (x, y, aim - distance),
//         "down" => (x, y, aim + distance),
//         _ => panic!("Invalid direction"),
//     }
// }
