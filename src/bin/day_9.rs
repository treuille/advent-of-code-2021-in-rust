// use ndarray::{s, Array, Array2, Ix1};
// use ndarray::{Array, Array2, ArrayView2, Ix1};
use ndarray::prelude::*;
use std::iter::Iterator;

type Pt = (usize, usize);

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
    let shape = (rows, flat_input.len() / rows);
    let heights: Array2<u8> = flat_input.into_shape(shape).unwrap();

    let risk_level_sum: usize = lowest_points(&heights)
        .iter()
        .map(|(_, &height)| (height as usize) + 1)
        .sum();
    println!("shape: {shape:?}");
    // println!("shape: {:?}", grid.shape());
    // println!("last row: {:?}", grid.slice(s![-1, ..]));
    println!("risk_level_sum: {risk_level_sum}");
}

fn neighbors(&(i, j): &Pt, &(w, h): &Pt) -> impl Iterator<Item = Pt> {
    [
        (i > 0).then(|| (i - 1, j)),
        (i < w - 1).then(|| (i + 1, j)),
        (j > 0).then(|| (i, j - 1)),
        (j < h - 1).then(|| (i, j + 1)),
    ]
    .into_iter()
    .flatten()
}

/// Finds all the lowerst point in the height map.
fn lowest_points(heights: &Array2<u8>) -> Vec<(Pt, &u8)> {
    let shape = (heights.shape()[0], heights.shape()[1]);
    heights
        .indexed_iter()
        .filter(|(ij, &height)| neighbors(ij, &shape).all(|neighbor| heights[neighbor] > height))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn neighbors_works() {
        [
            ((0, 0), vec![(1, 0), (0, 1)]),
            ((1, 0), vec![(0, 0), (2, 0), (1, 1)]),
            ((1, 1), vec![(0, 1), (2, 1), (1, 0), (1, 2)]),
            ((2, 1), vec![(1, 1), (2, 0), (2, 2)]),
        ]
        .into_iter()
        .for_each(|(ij, answer)| {
            let neighbors: Vec<Pt> = neighbors(&ij, &[3, 3]).collect();
            assert_eq!(neighbors, answer);
        });
    }
}
// TODO:
// 2. Split out lowest_points(grid) -> Vec<(ij, x)> j
// 3. Write solve_9a (lowest_points)
// 4. Write a flood fill to solve_9b (grid, lowest_points)
