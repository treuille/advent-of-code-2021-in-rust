fn main() {
    println!("Puzzle 1a: {:?}", solve_puzzle(2));
    println!("Puzzle 1b: {:?}", solve_puzzle(4));
}

fn solve_puzzle(window_size: usize) -> usize {
    include_str!("../../puzzle_inputs/day_01.txt")
        .lines()
        .filter_map(|x| x.parse().ok())
        .collect::<Vec<usize>>()
        .windows(window_size)
        .filter(|pair| pair[0] < pair[1])
        .count()
}
