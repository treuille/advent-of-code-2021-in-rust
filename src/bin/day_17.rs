use std::ops::RangeInclusive;

const TARGET_XS: RangeInclusive<isize> = 269..=292;
const TARGET_YS: RangeInclusive<isize> = -68..=-44;

fn main() {
    // for (i, line) in read_input().into_iter().enumerate() {
    //     println!("{i}: \"{line}\"");
    // }

    println!("17a: {} (123)", solve_17a());
    println!("17b: {} (456)", solve_17b());
}

fn solve_17a() -> usize {
    123
}

fn solve_17b() -> usize {
    456
}

// /// Read the input file and turn it into an Array2<u8>
// fn read_input() -> Vec<&'static str> {
//     include_str!("../../puzzle_inputs/day_17.txt")
//         .lines()
//         .collect()
// }
