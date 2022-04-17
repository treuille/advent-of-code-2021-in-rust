use core::cmp::Reverse;
use ndarray::prelude::*;
use std::collections::BinaryHeap;

/// A point on the grid.
type Pt = (usize, usize);

fn main() {
    let input = read_input(include_str!("../../puzzle_inputs/day_15.txt"));
    println!("15a: {} (602)", solve_15a(&input));
    println!("15b: {} (2935)", solve_15b(&input));
}

fn solve_15a(input: &Array2<u32>) -> u32 {
    find_shortest_path(input)
}

fn solve_15b(input: &Array2<u32>) -> u32 {
    let (w, h) = input.dim();
    let grow_by = 5;
    let mut bigger_input = Array2::from_elem((w * grow_by, h * grow_by), 0);
    for i in 0..grow_by {
        for j in 0..grow_by {
            let mut block = input.clone();
            block -= 1u32;
            block += (i + j) as u32;
            block %= 9;
            block += 1u32;
            let slice = s![(i * w)..((i + 1) * w), (j * h)..((j + 1) * h)];
            bigger_input.slice_mut(slice).assign(&block);
        }
    }
    find_shortest_path(&bigger_input)
}

/// Read the input file and turn it into an Array2<u32>
fn read_input(input: &str) -> Array2<u32> {
    let mut input = input.trim().lines().peekable();
    let w = input.peek().unwrap().len();
    let array: Array1<u32> =
        Array::from_iter(input.flat_map(|line| line.split("").filter_map(|s| s.parse().ok())));
    let h = array.len() / w;
    array.into_shape((w, h)).unwrap()
}

/// Iterate over the neighboring points to a point in the 2D array.
fn neighbors((i, j): Pt, (w, h): Pt) -> impl Iterator<Item = Pt> {
    [
        (i > 0).then(|| (i - 1, j)),
        (i < w - 1).then(|| (i + 1, j)),
        (j > 0).then(|| (i, j - 1)),
        (j < h - 1).then(|| (i, j + 1)),
    ]
    .into_iter()
    .flatten()
}

/// Find the shortest from the upper left hand side to the bottom right.
fn find_shortest_path(input: &Array2<u32>) -> u32 {
    let (w, h) = input.dim();
    let mut costs = Array2::from_elem((w, h), None);
    let mut heap = BinaryHeap::new();
    heap.push(Reverse((0, (0, 0))));
    while let Some(Reverse((cost, pt_1))) = heap.pop() {
        if costs[pt_1].is_none() {
            costs[pt_1] = Some(cost);
            for pt_2 in neighbors(pt_1, (w, h)) {
                heap.push(Reverse((cost + input[pt_2], pt_2)));
            }
        }
    }
    costs[(w - 1, h - 1)].unwrap()
}
