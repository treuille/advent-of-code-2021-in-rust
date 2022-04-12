use itertools::iproduct;
use ndarray::prelude::*;

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

fn main() {
    let octopi: Array2<u8> = read_input(PUZZLE_INPUT);
    let (soln_a, soln_b): (usize, usize) = solve_puzzles(octopi);
    println!("11a: {} (1681)", soln_a);
    println!("11b: {} (276)", soln_b);
}

/// Parse the input string into a 2D array.
fn read_input(puzzle_input: &str) -> Array2<u8> {
    let input_lines: Vec<&str> = puzzle_input.trim().lines().collect();
    let shape = (input_lines.len(), input_lines[0].len());
    input_lines
        .iter()
        .flat_map(|&line| line.split("").filter_map(|s| s.parse().ok()))
        .collect::<Array1<u8>>()
        .into_shape(shape)
        .expect("Array shape mismatch.")
}

/// Run the simulation, solving both puzzles as we go.
fn solve_puzzles(mut octopi: Array2<u8>) -> (usize, usize) {
    let mut total_flashes: usize = 0;
    let mut soln_a: Option<usize> = None;
    let mut soln_b: Option<usize> = None;

    // Run the simulation until we've answered both puzzles A and B.
    for step_no in 1.. {
        // Step the simulation, processing and counting flahes.
        octopi += 1;
        while *octopi.iter().max().unwrap() > 9 {
            octopi = Array::from_shape_fn(octopi.raw_dim(), |pt| match octopi[pt] {
                0 | 10.. => 0,
                x => x + (neighbors(pt, &octopi).filter(|&&x| x > 9).count() as u8),
            });
        }
        let flashes = octopi.iter().filter(|&&x| x == 0).count();
        total_flashes += flashes;

        // Puzzle A asks for the number of flashes at step 100.
        soln_a = soln_a.or_else(|| (step_no == 100).then(|| total_flashes));

        // Puzzle B asks when the octopi first all flash simultaneously.
        soln_b = soln_b.or_else(|| (flashes == octopi.len()).then(|| step_no));

        // Stop when we've solved both puzzles.
        if let (Some(soln_a), Some(soln_b)) = (soln_a, soln_b) {
            return (soln_a, soln_b);
        }
    }
    unreachable!("Both puzzles must be answered by now.");
}

/// Iterate over all neighboring values, including diagonals.
fn neighbors<A>((i, j): (usize, usize), grid: &Array2<A>) -> impl Iterator<Item = &A> {
    let neighbors_1d = |x: usize| {
        [x.checked_sub(1), Some(x), x.checked_add(1)]
            .into_iter()
            .flatten()
    };
    iproduct!(neighbors_1d(i), neighbors_1d(j)).filter_map(move |coord| match coord {
        coord if coord == (i, j) => None,
        coord => grid.get(coord),
    })
}
