use ndarray::prelude::*;
use std::collections::HashSet;
use std::iter::Iterator;

type Pt = (usize, usize);

fn main() {
    let heights: Array2<u8> = read_input();
    let lowest_points: Vec<(Pt, &u8)> = lowest_points(&heights);
    println!("9a: {} (550)", solve_9a(&lowest_points));
    println!("9b: {} (1100682)", solve_9b(&heights, &lowest_points));
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
                if !(heights[ij] == 9 || basin.contains(&ij)) {
                    basin.insert(ij);
                    candidate_pts.extend(neighbors(&ij, &shape));
                }
            }
            basin.len()
        })
        .collect();
    basin_sizes.sort_unstable();
    basin_sizes.iter().rev().take(3).product()
}

/// Read the input file and turn it into an Array2<u8>
fn read_input() -> Array2<u8> {
    let rows: Vec<Array2<u8>> = include_str!("../../puzzle_inputs/day_09.txt")
        .lines()
        .map(|line| {
            Array1::from_iter(line.chars().map(|c| c.to_string().parse().unwrap()))
                .insert_axis(Axis(0))
        })
        .collect();
    let row_views: Vec<ArrayView2<u8>> = rows.iter().map(ArrayView2::from).collect();
    ndarray::concatenate(Axis(0), &row_views).unwrap()
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

/// Finds all the lowest point in the height map.
fn lowest_points(heights: &Array2<u8>) -> Vec<(Pt, &u8)> {
    let shape = (heights.shape()[0], heights.shape()[1]);
    heights
        .indexed_iter()
        .filter(|(ij, &height)| neighbors(ij, &shape).all(|neighbor| heights[neighbor] > height))
        .collect()
}
