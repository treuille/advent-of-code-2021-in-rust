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
    println!("15a: {} (602)", solve_15a(&input));
    println!("15b: {} (???)", solve_15b(&input));
}

fn solve_15a(input: &Array2<u32>) -> u32 {
    find_shortest_path(input)
}

fn solve_15b(input: &Array2<u32>) -> u32 {
    let bigger_1 = embiggen(input, 5);
    let bigger_2 = embiggen_2(input, 5);
    assert_eq!(bigger_1, bigger_2);
    find_shortest_path(&bigger_2)
}

/// Na
fn embiggen(input: &Array2<u32>, grow_by: usize) -> Array2<u32> {
    let (w, h) = input.dim();
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
    bigger_input
}

fn embiggen_2(input: &Array2<u32>, grow_by: usize) -> Array2<u32> {
    let (w, h) = input.dim();
    let mut bigger_input = Array2::from_elem((w * grow_by, h * grow_by), 0);
    for i in 0..grow_by {
        for j in 0..grow_by {
            let mut block = input.clone();
            block += (i + j) as u32;
            let slice = s![(i * w)..((i + 1) * w), (j * h)..((j + 1) * h)];
            bigger_input.slice_mut(slice).assign(&block);
        }
    }
    bigger_input.map_inplace(|x| {
        if *x >= 10 {
            *x -= 9;
        }
    });
    bigger_input
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

fn find_shortest_path(input: &Array2<u32>) -> u32 {
    let mut costs = Array2::from_elem(input.dim(), 0);
    for (x, y) in diag_iter(input.dim()).skip(1) {
        costs[(x, y)] = input[(x, y)]
            + [
                x.checked_sub(1).map(|x| (x, y)),
                y.checked_sub(1).map(|y| (x, y)),
            ]
            .into_iter()
            .flatten()
            .map(|coord| costs[coord])
            .min()
            .unwrap();
    }
    println!("input:\n{input}");
    println!("costs:\n{costs}");
    let (w, h) = costs.dim();
    println!("answer: {}", costs[(w - 1, h - 1)]);
    costs[(w - 1, h - 1)]
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
