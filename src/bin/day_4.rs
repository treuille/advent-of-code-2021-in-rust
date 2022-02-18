// use std::error::Error;
use ndarray::{Array, Array2, Axis};
// use std::num::ParseIntError;
// use std::str::FromStr;

fn main() {
    let input = include_str!("../../puzzle_inputs/day_4.txt");
    let elts: Vec<_> = input.split_terminator("\n\n").collect();
    let (guesses, boards) = elts.split_first().unwrap();
    println!("guesses: {:?}", guesses);
    let guesses: Vec<usize> = guesses.split(',').map(|s| s.parse().unwrap()).collect();
    println!("guesses: {:?}", guesses);

    let boards: Vec<_> = boards.iter().map(|s| board_from_str(s)).collect();
    println!("first:\n{:?}", boards.first());
    println!("first:\n{:?}", boards.first().unwrap().shape());
    println!("last:\n{:?}", boards.last());
    println!("last:\n{:?}", boards.last().unwrap().shape());

    let marks: Vec<Array2<bool>> = boards
        .iter()
        .map(|board| match board.shape() {
            [w, h] => Array2::default((*w, *h)),
            _ => unreachable!(),
        })
        .collect();

    // println!("first:\n{:?}", found.first());
    // println!("first:\n{:?}", found.first().unwrap().shape());
    // println!("last:\n{:?}", found.last());
    // println!("last:\n{:?}", found.last().unwrap().shape());

    let first = boards.first().unwrap();
    println!("first:\n{:?}", first);
    let idx = find_in_board(first, 18);
    if let Some((i, j)) = idx {
        println!("The solution is {}", first[(i, j)]);
    }
    println!("solved: {:?}", idx);

    // first[(0, 0)] = true;
    // first[(0, 1)] = true;
    // first[(0, 2)] = true;
    // first[(0, 3)] = true;
    // first[(0, 4)] = true;
}

fn board_from_str(s: &str) -> Array2<usize> {
    let elts: Vec<Vec<usize>> = s
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|s| s.parse().unwrap())
                .collect()
        })
        .collect();

    let (w, h) = (elts[0].len(), elts.len());
    Array::from_iter(elts.into_iter().flatten())
        .into_shape((w, h))
        .unwrap()
}

fn find_in_board(board: &Array2<usize>, find_elt: usize) -> Option<(usize, usize)> {
    board.indexed_iter().find_map(|(idx, &board_elt)| {
        if board_elt == find_elt {
            Some(idx)
        } else {
            None
        }
    })
}

fn board_is_solved(marks: &Array2<bool>) -> bool {
    marks
        .axis_iter(Axis(0))
        .chain(marks.axis_iter(Axis(1)))
        .any(|row_or_col| row_or_col.iter().all(|&x| x))
}
// #[derive(Debug)]
// struct Board {
//     w: usize,
//     h: usize,
//     elts: Vec<Vec<usize>>,
// }

// impl FromStr for Board {
//     type Err = ParseIntError;

//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         println!("parsing: {}", s);

//         let elts: Vec<Vec<usize>> = s
//             .lines()
//             .map(|line| {
//                 line.split_ascii_whitespace()
//                     .map(|s| s.parse())
//                     .collect::<Result<_, _>>()
//             })
//             .collect::<Result<_, _>>()?;

//         Ok(Self {
//             w: elts[0].len(),
//             h: elts.len(),
//             elts,
//         })
//     }
// }
