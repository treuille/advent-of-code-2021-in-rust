use std::fmt::{Debug, Error, Formatter, Write};
use std::ops::Add;

fn main() {
    let s1 = SnailfishNumber::new("[[[[4,3],4],4],[7,[[8,4],9]]]");
    let s2 = SnailfishNumber::new("[1,1]");

    println!("s1: {s1:?}");
    println!("s2: {s2:?}");
    println!("sum: {:?}", s1 + s2);
}

enum Token {
    Open,
    Close,
    Number(u8),
}

impl Debug for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        match self {
            Token::Open => f.write_char('['),
            Token::Close => f.write_char(']'),
            Token::Number(n) => f.write_fmt(format_args!("{}", n)),
        }
    }
}

#[derive(Debug)]
struct SnailfishNumber(Vec<Token>);

impl SnailfishNumber {
    fn new(s: &str) -> Self {
        Self(
            s.chars()
                .skip(1)
                .take(s.len() - 2)
                .filter_map(|c| match c {
                    '[' => Some(Token::Open),
                    ']' => Some(Token::Close),
                    ',' => None,
                    c => Some(Token::Number(String::from(c).parse().unwrap())),
                })
                .collect(),
        )
        // println!("chars: {:?}", chars.collect::<Vec<char>>());
        // todo!("SnailfishNumber constructor");
    }
}

impl Add for SnailfishNumber {
    /// The resulting type after applying the `+` operator.
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        todo!("Add.");
    }
}

// fn solve_XXa() -> usize {
//     123
// }

// fn solve_XXb() -> usize {
//     456
// }

// /// Read the input file and turn it into an Array2<u8>
// fn read_input() -> Vec<&'static str> {
//     include_str!("../../puzzle_inputs/day_XX.txt")
//         .lines()
//         .collect()
// }
