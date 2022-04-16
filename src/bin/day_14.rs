fn main() {
    for (i, line) in read_input().into_iter().enumerate() {
        println!("{i}: \"{line}\"");
    }
    println!("14a: {} (123)", solve_14a());
    println!("14b: {} (456)", solve_14b());
}

fn solve_14a() -> usize {
    123
}

fn solve_14b() -> usize {
    456
}

/// Read the input file and turn it into an Array2<u8>
fn read_input() -> Vec<&'static str> {
    include_str!("../../puzzle_inputs/day_14.txt")
        .lines()
        .collect()
}
