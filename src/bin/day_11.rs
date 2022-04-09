use itertools::iproduct;
use ndarray::prelude::*;

/// This is the puzzle input I got.
#[allow(dead_code)]
const PUZZLE_INPUT: &str = "
6227618536
2368158384
5385414113
4556757523
6746486724
4881323884
4648263744
4871332872
4724128228
4316512167
";

/// This is the example input in the puzzle description.
#[allow(dead_code)]
const EXAMPLE_INPUT_1: &str = "
5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526
";

/// Anohter example input in the puzzle description.
#[allow(dead_code)]
const EXAMPLE_INPUT_2: &str = "
11111
19991
19191
19991
11111
";

/// Toggle between the two inputs;
const INPUT: &str = EXAMPLE_INPUT_2;

fn main() {
    // Parse the input into an array.
    let input_lines: Vec<&str> = INPUT.trim().lines().collect();
    let shape = (input_lines.len(), input_lines[0].len());
    let x: Array2<u8> = input_lines
        .iter()
        .flat_map(|&line| line.split("").filter_map(|s| s.parse().ok()))
        .collect::<Array1<u8>>()
        .into_shape(shape)
        .expect("Array shape mismatch");
    println!("{x:?}");

    println!("step:\n{}", step(x));
}

fn step(octopi: Array2<u8>) -> Array2<u8> {
    let mut octopi = octopi + 1;
    while *octopi.iter().max().unwrap() >= 9 {
        octopi = Array::from_shape_fn(octopi.raw_dim(), |pt| match octopi[pt] {
            0 => 0,
            9 => 0,
            x => {
                let neighbors: Vec<&u8> = neighbors(pt, &octopi).collect();
                let flashed: Vec<&&u8> = neighbors.iter().filter(|&&&x| x >= 9).collect();
                let x2: u8 = x + (flashed.len() as u8);
                println!("{pt:?} -> {x} :: {neighbors:?} -> {flashed:?} -> {x2}");
                x2
            }
        });
    }
    assert!(*octopi.iter().max().unwrap() < 9);
    octopi
}

#[allow(dead_code)]
fn solve_11a() -> usize {
    123
}

#[allow(dead_code)]
fn solve_11b() -> usize {
    456
}

/// Iterate over the neighboring points to a point in the 2D array.
// fn neighbors<'a, A>((i, j): (usize, usize), grid: &'a Array2<A>) -> impl Iterator<Item = &'a A> {
fn neighbors<A>((i, j): (usize, usize), grid: &Array2<A>) -> impl Iterator<Item = &A> {
    let neighbors_1d = |x: usize| {
        [x.checked_sub(1), Some(x), x.checked_add(1)]
            .into_iter()
            .flatten()
    };
    iproduct!(neighbors_1d(i), neighbors_1d(j)).filter_map(move |coord| match coord {
        coord if coord == (i, j) => None,
        coord => grid.get(coord),
    })
}
