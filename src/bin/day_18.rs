fn main() {
    let s1 = SnailfishNumber::new("[[[[4,3],4],4],[7,[[8,4],9]]]");
    let s2 = SnailfishNumber::new("[1,1]");

    println("s1: {s1:?}");
    println("s2: {s2:?}");
    println("sum: {}", s1 + s2);
}

// fn solve_XXa() -> usize {
//     123
// }

// fn solve_XXb() -> usize {
//     456
// }

// /// Read the input file and turn it into an Array2<u8>
// fn read_input() -> Vec<&'static str> {
//     include_str!("../../puzzle_inputs/day_XX.txt")
//         .lines()
//         .collect()
// }
