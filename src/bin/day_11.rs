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

fn step(x: Array2<u8>) -> Array2<u8> {
    let y = &x + 1;
    println!("{y:?}");
    let z = y.map(|&elt| elt > 9);
    println!("{z:?}");
    y
}

#[allow(dead_code)]
fn solve_11a() -> usize {
    123
}

#[allow(dead_code)]
fn solve_11b() -> usize {
    456
}
