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
    let (lookup_table, image) = read_input(DEMO_INPUT.trim());
    println!("result: {}", false);

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

type Pt = (i64, i64);
type Image = HashSet<Pt>;
type LookupTable = [bool; 512];

/// Read the input file and turn it into an Array2<u8>
fn read_input(input: &str) -> (LookupTable, Image) {
    let (lookup_table_str, image_str) = input.split_once("\n\n").unwrap();
    println!("lookup_table:\n{lookup_table_str}");
    println!("image:\n{image_str}");
    // println!(
    //     "lookup_table: {:?}",
    //     lookup_table.chars().collect::<Vec<_>>()
    // );
    let lookup_table = [false; 512];
    lookup_table_str
        .chars()
        .filter_map(|c| match c {
            '.' => Some(false),
            '#' => Some(true),
            _ => None,
        })
        .enumerate()
        .filter_map(|(i, b)| b.then(|| i))
        .for_each(|i| println!("{i}"));
    todo!("Implement read_input");
    // include_str!("../../puzzle_inputs/day_XX.txt")
    //     .lines()
    //     .collect()
}
