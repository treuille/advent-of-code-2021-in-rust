use itertools::{iproduct, Itertools};
use std::borrow::Borrow;
use std::collections::HashSet;

#[allow(dead_code)]
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
    let input = include_str!("../../puzzle_inputs/day_20.txt").trim();
    // let input = DEMO_INPUT.trim();
    let (lookup_table, image) = read_input(input);
    print_image(&image);
    println!();

    let image = enhance(&image, &lookup_table);
    print_image(&image);
    println!();

    let image = enhance(&image, &lookup_table);
    print_image(&image);
    println!();

    println!("len: {}", image.len());
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

fn enhance(image: &Image, lookup_table: &LookupTable) -> Image {
    image
        .iter()
        .flat_map(neighbors)
        .unique()
        .flat_map(|pt1| {
            let indx =
                neighbors(pt1)
                    .enumerate()
                    .fold(0, |indx, (i, pt2)| match image.contains(&pt2) {
                        false => indx,
                        true => indx | 1 << (8 - i),
                    });
            println!("{pt1:?} -> {indx}");
            lookup_table[indx].then(|| pt1)
        })
        .collect()
}

fn neighbors(pt: impl Borrow<Pt>) -> impl Iterator<Item = Pt> {
    let (i, j) = pt.borrow();
    iproduct!((j - 1)..=(j + 1), (i - 1)..=(i + 1)).map(|(j, i)| (i, j))
}

fn print_image(image: &Image) {
    let min_i = *image.iter().map(|(i, j)| i).min().unwrap();
    let max_i = *image.iter().map(|(i, j)| i).max().unwrap();
    let min_j = *image.iter().map(|(i, j)| j).min().unwrap();
    let max_j = *image.iter().map(|(i, j)| j).max().unwrap();
    for j in min_j..=max_j {
        for i in min_i..=max_i {
            match image.contains(&(i, j)) {
                true => print!("# "),
                false => print!(". "),
            }
        }
        println!();
    }
}

fn parse_image(image_str: &str) -> Image {
    image_str
        .lines()
        .enumerate()
        .flat_map(|(j, line)| {
            line.chars()
                .enumerate()
                .filter_map(move |(i, c)| (c == '#').then(|| (i as i64, j as i64)))
        })
        .collect()
}

/// Read the input file and turn it into an Array2<u8>
fn read_input(input: &str) -> (LookupTable, Image) {
    let (lookup_table_str, image_str) = input.split_once("\n\n").unwrap();
    println!("lookup_table:\n{lookup_table_str}");
    println!("image:\n{image_str}");

    // Parse the lookup table
    let mut lookup_table = [false; 512];
    lookup_table_str
        .chars()
        .filter_map(|c| match c {
            '.' => Some(false),
            '#' => Some(true),
            _ => None,
        })
        .enumerate()
        .filter_map(|(i, b)| b.then(|| i))
        .for_each(|i| lookup_table[i] = true);

    (lookup_table, parse_image(image_str))
}
