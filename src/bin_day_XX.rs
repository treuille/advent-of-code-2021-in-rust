fn main() {
    for (i, line) in read_input().into_iter().enumerate() {
        println!("{i}: \"{line}\"");
    }
    println!("XXa: {} (123)", solve_XXa());
    println!("XXb: {} (456)", solve_XXb());
}

fn solve_XXa() -> usize {
    123
}

fn solve_XXb() -> usize {
    456
}

/// Read the input file and turn it into an Array2<u8>
fn read_input() -> Vec<&'static str> {
    include_str!("../../puzzle_inputs/day_XX.txt")
        .lines()
        .collect()
}
