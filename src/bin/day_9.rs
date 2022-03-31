use ndarray::{s, Array, Array2, Ix1};

fn main() {
    // Parse the input into an Array2<u8>
    let lines: Vec<&str> = include_str!("../../puzzle_inputs/day_9.txt")
        .lines()
        .collect();
    let rows = lines.len();

    let flat_input: Array<u8, Ix1> = Array::from_iter(
        lines
            .iter()
            .flat_map(|line| line.chars().map(|c| c.to_string().parse().unwrap())),
    );
    let cols = flat_input.len() / rows;
    let grid: Array2<u8> = flat_input.into_shape((rows, cols)).unwrap();
    println!("shape: {rows}x{cols}");
    println!("last row: {:?}", grid.slice(s![-1, ..]));
}
