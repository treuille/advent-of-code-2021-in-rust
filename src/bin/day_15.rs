use ndarray::prelude::*;

const TEST_INPUT: &str = "
1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581
";

fn main() {
    // let input = read_input(TEST_INPUT);
    let input = read_input(include_str!("../../puzzle_inputs/day_15.txt"));
    let mut costs = Array2::from_elem(input.dim(), 0);
    for (x, y) in diag_iter(input.dim()) {
        costs[(x, y)] = [
            x.checked_sub(1).map(|x| (x, y)),
            y.checked_sub(1).map(|y| (x, y)),
        ]
        .into_iter()
        .flatten()
        .map(|coord| costs[coord] + input[coord])
        .min()
        .unwrap_or(0);
    }
    println!("input:\n{input}");
    println!("costs:\n{costs}");
    let (w, h) = costs.dim();
    println!("answer: {}", costs[(w - 1, h - 1)]);
    // for (i, line) in read_input().into_iter().enumerate() {
    //     println!("{i}: \"{line}\"");
    // println!("XXb: {} (456)", solve_XXb());
}

fn solve_XXa() -> usize {
    123
}

fn solve_XXb() -> usize {
    456
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

/// Iterate in diagonal stripes over the coordinates of a 2D array.
// TODO: Do I really need all these function calls and moves?
fn diag_iter((w, h): (usize, usize)) -> impl Iterator<Item = (usize, usize)> {
    let left_edge = |w| (0..w).map(|x| (x, 0));
    let bottom_edge = |w, h| (1..h).map(move |y| (w - 1, y));
    left_edge(w)
        .chain(bottom_edge(w, h))
        .flat_map(move |(x, y)| {
            (0..).map_while(move |i| match (x.checked_sub(i), y + i) {
                (Some(x), y) if y < h => Some((x, y)),
                _ => None,
            })
        })
}
