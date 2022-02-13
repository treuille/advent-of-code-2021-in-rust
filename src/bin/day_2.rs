fn main() {
    println!("Puzzle 2a: {:?}", solve_puzzle(advance_a));
    println!("Puzzle 2b: {:?}", solve_puzzle(advance_b));
}

type State = (isize, isize, isize);

fn solve_puzzle<F>(advance: F) -> isize
where
    F: Fn(State, &str, isize) -> State,
{
    let (x, y, _) =
        include_str!("../../puzzle_inputs/day_2.txt")
            .lines()
            .fold((0, 0, 0), |state, line| {
                let (direction, distance) = line.split_once(" ").unwrap();
                let distance: isize = distance.parse().unwrap();
                advance(state, direction, distance)
            });
    x * y
}

fn advance_a((x, y, _): State, direction: &str, distance: isize) -> State {
    match direction {
        "forward" => (x + distance, y, 0),
        "up" => (x, y - distance, 0),
        "down" => (x, y + distance, 0),
        _ => panic!("Invalid direction"),
    }
}

fn advance_b((x, y, aim): State, direction: &str, distance: isize) -> State {
    match direction {
        "forward" => (x + distance, y + distance * aim, aim),
        "up" => (x, y, aim - distance),
        "down" => (x, y, aim + distance),
        _ => panic!("Invalid direction"),
    }
}
