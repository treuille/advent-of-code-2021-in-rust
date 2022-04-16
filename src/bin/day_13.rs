fn main() {
    for (i, line) in read_input().into_iter().enumerate() {
        println!("{i}: \"{line}\"");
    }
    println!("13a: {} (123)", solve_13a());
    println!("13b: {} (456)", solve_13b());
}

fn solve_13a() -> usize {
    123
}

fn solve_13b() -> usize {
    456
}

/// Read the input file and turn it into an Array2<u8>
fn read_input() -> Vec<&'static str> {
    include_str!("../../puzzle_inputs/day_13.txt")
        .lines()
        .collect()
}
