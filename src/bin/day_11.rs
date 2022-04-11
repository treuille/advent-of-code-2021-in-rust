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
    // let mut first_all_flash: Option<usize> = None;
    for step_no in 1..=1000usize {
        let (new_x, new_flashes, all_flashed) = step(x);
        if all_flashed {
            panic!("done: {step_no}");
            // first_all_flash = first_all_flash.or(Some(step_no));
        } else {
            println!("not done: {step_no}");
        }
        x = new_x;
        flashes += new_flashes;

        println!("step {step_no}: ({flashes} flshes)\n{x}");
        for row in x.rows() {
            println!("{}", row.map(|&x| x.to_string()).iter().join(""));
        }
        println!("---");
    }
    // println!("puzzle a response: {}", 1681);
    // println!("first_all_flash: {first_all_flash:?}");
}

fn step(octopi: Array2<u8>) -> (Array2<u8>, usize, bool) {
    let mut octopi = octopi + 1;
    let mut flashes = 0;
    let mut all_flashed = false;
    let (w, h) = match octopi.shape() {
        &[w, h] => (w, h),
        _ => unreachable!(),
    };
    while *octopi.iter().max().unwrap() > 9 {
        let new_flashes = octopi.iter().filter(|&&x| x > 9).count();
        flashes += new_flashes;
        octopi = Array::from_shape_fn(octopi.raw_dim(), |pt| match octopi[pt] {
            x if x == 0 || x > 9 => 0,
            x => x + (neighbors(pt, &octopi).filter(|&&x| x > 9).count() as u8),
        });
    }
    let n_flashed = octopi.iter().filter(|&&x| x == 0u8).count();
    println!("n_flashed: {n_flashed}");
    let all_flashed = n_flashed == w * h;
    assert!(*octopi.iter().max().unwrap() <= 9);
    (octopi, flashes, all_flashed)
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
