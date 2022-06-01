use std::collections::HashSet;

const DEMO_INPUT: &str = "
..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..##
#..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###
.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#.
.#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#.....
.#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#..
...####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.....
..##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

#..#.
#....
##..#
..#..
..###
";

fn main() {
    println!("result: {}", read_input(DEMO_INPUT));

    // for (i, line) in read_input().into_iter().enumerate() {
    //     println!("{i}: \"{line}\"");
    // }
    // println!("XXa: {} (123)", solve_XXa());
    // println!("XXb: {} (456)", solve_XXb());
}

// fn solve_XXa() -> usize {
//     123
// }

// fn solve_XXb() -> usize {
//     456
// }

// type Pt = (i64, i64);
// type Image = HashSet<Pt>;
// type LookupTable = [bool; 512];

/// Read the input file and turn it into an Array2<u8>
fn read_input(_input: &str) -> bool {
    false
    // include_str!("../../puzzle_inputs/day_XX.txt")
    //     .lines()
    //     .collect()
}
