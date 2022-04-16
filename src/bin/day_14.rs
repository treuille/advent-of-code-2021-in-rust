fn main() {
    for (i, line) in read_input().into_iter().enumerate() {
        println!("{i}: \"{line}\"");
    }
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
fn read_input() -> Vec<&'static str> {
    include_str!("../../puzzle_inputs/day_10.txt")
        .lines()
        .collect()
}
