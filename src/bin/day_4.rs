use ndarray::{Array, Array2, Axis};
use std::iter;

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

    let mut marks: Vec<Array2<bool>> = boards
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

    // let first = boards.first().unwrap();
    // println!("first:\n{:?}", first);
    // let idx = find_in_board(first, 18);
    // if let Some((i, j)) = idx {
    //     println!("The solution is {}", first[(i, j)]);
    // }
    // println!("solved: {:?}", idx);
    //
    // let n_boards = boards.len();
    // let won = vec![false; n_boards];

    let mut first_result = None;
    let mut last_result = None;
    for guess in guesses {
        for (board, marks) in boards.iter().zip(marks.iter_mut()) {
            if board_is_solved(marks) {
                continue;
            }
            if let Some(idx) = find_in_board(board, guess) {
                marks[idx] = true;
                if board_is_solved(marks) {
                    println!("board:\n{board}");
                    println!("marks:\n{marks}");
                    let result = unmarked_sum(board, marks) * guess;
                    first_result.get_or_insert(result);
                    last_result = Some(result);
                }
            }
        }
    }
    match (first_result, last_result) {
        (Some(first_result), Some(last_result)) => {
            println!("first_result: {first_result}");
            println!("last_result: {last_result}");
        }
        _ => panic!("no resutls"),
    };
    // let results = guesses
    //     .iter()
    // let result = unmarked_sum(board, marks) * guess;
    //         // println!("guess: {guess}");
    //         boards
    //             .iter()
    //             .zip(marks.iter_mut())
    //             .filter_map(|(board, the_marks)| match
    //                 false => Some((board, the_marks)),
    //                 true => None,
    //             })
    //             .filter_map(|(board, the_marks)| {
    //             })
    //     })
    //     .collect::<Vec<_>>();
    // println!("results: {:?}", results);
    // won.iter()
    //     .enumerate()
    //     .filter_map(|(i, did_win)| match did_win {
    //         false => Some((i, boards[i], marks[i])),
    //         true => None,
    //     })
    //     .filter_map(|(i, board, mark)| {
    //         if
    //             }
    //         }
    //         None
    //     })
    //     .for_each(|result| {
    //         println!("All done: {result} (6592)");
    //     })
    // }
    // println!("Unable to find a solution.");
    // // first[(0, 0)] = true;
    // // first[(0, 1)] = true;
    // // first[(0, 2)] = true;
    // // first[(0, 3)] = true;
    // // first[(0, 4)] = true;
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

/// Returns the sum of all unmarked items on the board.
fn unmarked_sum(board: &Array2<usize>, marks: &Array2<bool>) -> usize {
    println!("board:\n{board}");
    println!("{:?}", board.iter().collect::<Vec<_>>());
    println!("marks:\n{marks}");
    println!("{:?}", marks.iter().collect::<Vec<_>>());
    let x: usize = board
        .iter()
        .zip(marks.iter())
        .filter_map(|(elt, mark)| match mark {
            false => Some(elt),
            true => None,
        })
        .sum();
    println!("sum1: {:?}", x);

    // println!("sum2: {:?}", x.sum());
    board
        .iter()
        .zip(marks.iter())
        .filter_map(|(elt, mark)| match mark {
            false => Some(elt),
            true => None,
        })
        .sum()
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
