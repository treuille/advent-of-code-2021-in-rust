// use ndarray::{s, Array, Array2, Ix1};
use std::iter::Iterator;

fn main() {
    println!("Hello world.");

    print_neighbors(&(0, 0), &[10, 10]);

    // // Parse the input into an Array2<u8>
    // let lines: Vec<&str> = include_str!("../../puzzle_inputs/day_9.txt")
    //     .lines()
    //     .collect();
    // let rows = lines.len();

    // let flat_input: Array<u8, Ix1> = Array::from_iter(
    //     lines
    //         .iter()
    //         .flat_map(|line| line.chars().map(|c| c.to_string().parse().unwrap())),
    // );
    // let cols = flat_input.len() / rows;
    // let grid: Array2<u8> = flat_input.into_shape((rows, cols)).unwrap();

    // let risk_level_sum: usize = grid
    //     .indexed_iter()
    //     .filter(|((i, j), height)| {
    //         [(-1, 0), (1, 0), (0, -1), (0, 1)]
    //             .into_iter()
    //             .map(|(di, dj)| (((*i as isize) + di) as usize, ((*j as isize) + dj) as usize))
    //             .filter_map(|ij| grid.get(ij))
    //             .all(|neighbor_height| neighbor_height > height)
    //     })
    //     .map(|(_, height)| (*height as usize) + 1)
    //     .sum();
    // println!("shape: {rows}x{cols}");
    // // println!("shape: {:?}", grid.shape());
    // // println!("last row: {:?}", grid.slice(s![-1, ..]));
    // println!("risk_level_sum: {risk_level_sum}");
}

fn print_neighbors(&(i, j): &(usize, usize), &[w, h]: &[usize; 2]) {
    for (indx, neighbor) in neighbors(&(i, j), &[w, h]).enumerate() {
        println!("{i},{j} {w}x{h} -> {indx} -> {neighbor:?}");
    }
}

fn neighbors(
    &(i, j): &(usize, usize),
    &[w, h]: &[usize; 2],
) -> impl Iterator<Item = (usize, usize)> {
    [
        (i > 0).then(|| (i - 1, j)),
        (i < w - 1).then(|| (i + 1, j)),
        (j > 0).then(|| (i, j - 1)),
        (j < h - 1).then(|| (i, j + 1)),
    ]
    .into_iter()
    .flatten()
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
            let neighbors: Vec<(usize, usize)> = neighbors(&ij, &[3, 3]).collect();
            assert_eq!(neighbors, answer);
        });
    }
}
// TODO:
// 1. Split out neighbors: (shape, ij) -> Iterator<Item=ij>
// 2. Split out lowest_points(grid) -> Vec<(ij, x)> j
// 3. Write solve_9a (lowest_points)
// 4. Write a flood fill to solve_9b (grid, lowest_points)
