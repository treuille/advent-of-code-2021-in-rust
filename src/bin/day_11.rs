use itertools::iproduct;
use itertools::Itertools;
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

/// Toggle between the two inputs;String::from(x)));
const INPUT: &str = PUZZLE_INPUT;

fn main() {
    // Parse the input into an array.
    let input_lines: Vec<&str> = INPUT.trim().lines().collect();
    let shape = (input_lines.len(), input_lines[0].len());
    let mut x: Array2<u8> = input_lines
        .iter()
        .flat_map(|&line| line.split("").filter_map(|s| s.parse().ok()))
        .collect::<Array1<u8>>()
        .into_shape(shape)
        .expect("Array shape mismatch");
    println!("{x:?}");

    let mut flashes: usize = 0;
    for step_no in 1..=100 {
        let (new_x, new_flashes) = step(x);
        x = new_x;
        flashes += new_flashes;

        println!("step {step_no}: ({flashes} flshes)\n{x}");
        for row in x.rows() {
            println!("{}", row.map(|&x| x.to_string()).iter().join(""));
        }
        println!("---");
    }
    println!("puzzle a response: {}", 1681);
}

fn step(octopi: Array2<u8>) -> (Array2<u8>, usize) {
    let mut octopi = octopi + 1;
    let mut flashes = 0;
    while *octopi.iter().max().unwrap() > 9 {
        flashes += octopi.iter().filter(|&&x| x > 9).count();
        octopi = Array::from_shape_fn(octopi.raw_dim(), |pt| match octopi[pt] {
            x if x == 0 || x > 9 => 0,
            x => x + (neighbors(pt, &octopi).filter(|&&x| x > 9).count() as u8),
        });
    }
    assert!(*octopi.iter().max().unwrap() <= 9);
    (octopi, flashes)
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
