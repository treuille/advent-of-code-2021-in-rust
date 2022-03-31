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

    let risk_level_sum: usize = grid
        .indexed_iter()
        .filter(|((i, j), height)| {
            [(-1, 0), (1, 0), (0, -1), (0, 1)]
                .into_iter()
                .map(|(di, dj)| (((*i as isize) + di) as usize, ((*j as isize) + dj) as usize))
                .filter_map(|ij| grid.get(ij))
                .all(|neighbor_height| neighbor_height > height)
        })
        .map(|(_, height)| (*height as usize) + 1)
        .sum();
    println!("shape: {rows}x{cols}");
    println!("shape: {:?}", grid.shape());
    println!("last row: {:?}", grid.slice(s![-1, ..]));
    println!("risk_level_sum: {risk_level_sum}");
}

// TODO:
// 1. Split out neighbors: (shape, ij) -> Iterator<Item=ij>
// 2. Split out lowest_points(grid) -> Vec<(ij, x)>
// 3. Write solve_9a (lowest_points)
// 4. Write a flood fill to solve_9b (grid, lowest_points)
