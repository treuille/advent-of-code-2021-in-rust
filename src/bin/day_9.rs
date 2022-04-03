use ndarray::prelude::*;
use std::collections::HashSet;
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

    let lowest_points: Vec<(Pt, &u8)> = lowest_points(&heights);
    println!("Puzzle 9a: {} (387)", solve_9a(&lowest_points));
    println!("Puzzle 9b: {} (xyz)", solve_9b(&heights, &lowest_points));
}

fn solve_9a(lowest_points: &[(Pt, &u8)]) -> usize {
    lowest_points
        .iter()
        .map(|(_, &height)| (height as usize) + 1)
        .sum()
}

fn solve_9b(heights: &Array2<u8>, lowest_points: &[(Pt, &u8)]) -> usize {
    let shape = (heights.shape()[0], heights.shape()[1]);
    let mut basin_sizes: Vec<usize> = lowest_points
        .iter()
        .map(|(ij, _)| {
            let mut candidate_pts: Vec<Pt> = vec![*ij];
            let mut basin: HashSet<Pt> = HashSet::new();
            while let Some(ij) = candidate_pts.pop() {
                if !basin.contains(&ij) {
                    basin.insert(ij);
                    candidate_pts
                        .extend(neighbors(&ij, &shape).filter(|neighbor| heights[*neighbor] != 9));
                }
            }
            basin.len()
        })
        .collect();
    basin_sizes.sort_unstable();
    basin_sizes[(basin_sizes.len() - 3)..].iter().product()
}

/// Iterate over the neighboring points to a point in the 2D array.
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
            let neighbors: Vec<Pt> = neighbors(&ij, &(3, 3)).collect();
            assert_eq!(neighbors, answer);
        });
    }
}
// TODO:
//
// 3. Write solve_9a (lowest_points)
// 4. Write a flood fill to solve_9b (grid, lowest_points)