use ndarray::{Array, Array2, Axis};
use std::mem;

fn main() {
    let input = include_str!("../../puzzle_inputs/day_04.txt");
    let elts: Vec<_> = input.split_terminator("\n\n").collect();
    let (guesses, boards) = elts.split_first().unwrap();

    let boards: Vec<Board> = boards.iter().map(|s| Board::from_str(s)).collect();

    let mut results_iter = guesses
        .split(',')
        .scan(boards, |boards, guess| {
            let guess: usize = guess.parse().unwrap();
            let mut results = Vec::new();
            let mut remaining_boards: Vec<Board> = boards
                .drain(..)
                .filter_map(|mut board| match board.mark(guess) {
                    true => {
                        results.push(board.unmarked_sum() * guess);
                        None
                    }
                    false => Some(board),
                })
                .collect();
            mem::swap(boards, &mut remaining_boards);
            Some(results.into_iter())
        })
        .flatten();
    println!("Puzzle 4a: {} (6592)", results_iter.next().unwrap());
    println!("Puzzle 4b: {} (31755)", results_iter.last().unwrap());
}

struct Board {
    /// The set of numbers on the board.
    nums: Array2<usize>,

    /// Whether any of those numbers have been marked.
    marks: Array2<bool>,
}

impl Board {
    /// Constructs a board from a string.
    fn from_str(s: &str) -> Self {
        let elts: Vec<Vec<usize>> = s
            .lines()
            .map(|line| {
                line.split_ascii_whitespace()
                    .map(|s| s.parse().unwrap())
                    .collect()
            })
            .collect();

        let (w, h) = (elts[0].len(), elts.len());
        Self {
            nums: Array::from_iter(elts.into_iter().flatten())
                .into_shape((w, h))
                .unwrap(),
            marks: Array2::default((w, h)),
        }
    }

    /// Finds a number on the board.
    fn find(&self, num: usize) -> Option<(usize, usize)> {
        self.nums.indexed_iter().find_map(
            |(idx, &board_elt)| {
                if board_elt == num {
                    Some(idx)
                } else {
                    None
                }
            },
        )
    }

    /// Returns true if the board has been solved.
    fn board_is_solved(&self) -> bool {
        self.marks
            .axis_iter(Axis(0))
            .chain(self.marks.axis_iter(Axis(1)))
            .any(|row_or_col| row_or_col.iter().all(|&x| x))
    }

    /// Marks an element in the board, returning true if this operation
    /// solved the board.
    fn mark(&mut self, num: usize) -> bool {
        if let Some(idx) = self.find(num) {
            self.marks[idx] = true;
            self.board_is_solved()
        } else {
            false
        }
    }

    /// Returns the sum of all unmarked items on the board.
    fn unmarked_sum(&self) -> usize {
        self.nums
            .iter()
            .zip(self.marks.iter())
            .filter_map(|(elt, mark)| match mark {
                false => Some(elt),
                true => None,
            })
            .sum()
    }
}
