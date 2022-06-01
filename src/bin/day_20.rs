// use itertools::{iproduct, Itertools};
use ndarray::prelude::*;
use ndarray::Zip;
// use std::borrow::Borrow;
// use std::collections::HashSet;

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
    // let input = include_str!("../../puzzle_inputs/day_20.txt").trim();
    let input = DEMO_INPUT.trim();
    let (lookup_table, image) = read_input(input);
    print_image(&image.view());
    println!();

    let runs = 2;
    let mut image = grow(image, runs);
    print_image(&image.view());
    println!();

    for i in 0..runs {
        image = enhance(image, &lookup_table);
        println!("iteration: {i}");
        print_image(&image.view());
        println!();
    }
    // let image = enhance(&image, &lookup_table);

    // let image = enhance(&image, &lookup_table);
    // print_image(&image);
    // println!();

    // println!("len: {}", image.len());
}

// fn solve_XXa() -> usize {
//     123
// }

// fn solve_XXb() -> usize {
//     456
// }

// type Pt = (i64, i64);
// type Pt = (usize, usize);
type Image = Array2<bool>;
type LookupTable = [bool; 512];

fn grow(image: Image, runs: usize) -> Image {
    let (w, h) = image.dim();
    let mut bigger_image = Array2::from_elem((w + 4 * runs, h + 4 * runs), false);
    let slice = s![(2 * runs)..(2 * runs + w), (2 * runs)..(2 * runs + h)];
    bigger_image.slice_mut(slice).assign(&image);
    bigger_image
}

fn enhance(image: Image, lookup_table: &LookupTable) -> Image {
    // image.windows((3, 3)).for_each(|window| {
    //     println!("{:?}", window);
    // });
    Zip::from(image.windows((3, 3))).map_collect(|window| {
        let indx = window.iter().enumerate().fold(0, |indx, (i, x)| match x {
            false => indx,
            true => indx | 1 << (8 - i),
        });
        lookup_table[indx]
    })
    // todo!("ehance {:?}", x.dim());
    // image
    //     .iter()
    //     .flat_map(neighbors)
    //     .unique()
    // , lookup_table: &LookupTable   //     .flat_map(|pt1| {
    //         let indx =
    //             neighbors(pt1)
    //                 .enumerate()
    //                 .fold(0, |indx, (i, pt2)| match image.contains(&pt2) {
    //                     false => indx,
    //                     true => indx | 1 << (8 - i),
    //                 });
    //         // println!("{pt1:?} -> {indx}");
    //         lookup_table[indx].then(|| pt1)
    //     })
    //     .collect()
}

// fn neighbors(pt: impl Borrow<Pt>) -> impl Iterator<Item = Pt> {
//     let (i, j) = pt.borrow();
//     iproduct!((j - 1)..=(j + 1), (i - 1)..=(i + 1)).map(|(j, i)| (i, j))
// }

fn print_image(image: &ArrayView2<bool>) {
    for row in image.rows() {
        let row_chars = row.iter().map(|x| match x {
            false => '.',
            true => '#',
        });
        println!("{}", row_chars.collect::<String>());
    }
    println!("{:?}", image.dim());
}

fn parse_grid(input: &str) -> Image {
    let mut lines = input.lines().peekable();
    let w = lines.peek().unwrap().len();
    let array: Array1<bool> =
        Array::from_iter(lines.flat_map(|line| line.chars().map(|c| c == '#')));
    let h = array.len() / w;
    array.into_shape((w, h)).unwrap()
}

/// Read the input file and turn it into an Array2<u8>
fn read_input(input: &str) -> (LookupTable, Image) {
    let (lookup_table_str, image_str) = input.split_once("\n\n").unwrap();

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

    (lookup_table, parse_grid(image_str))
}
