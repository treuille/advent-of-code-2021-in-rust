#![allow(dead_code)]

use itertools::{iproduct, Itertools};
use std::fmt::{Debug, Error, Formatter, Write};
use std::ops::Add;

fn main() {
    let snail_nums = read_input(include_str!("../../puzzle_inputs/day_18.txt"));

    println!("18a: {} (4145)", solve_18a(&snail_nums));
    println!("18b: {} (4855)", solve_18b(&snail_nums));
}

fn solve_18a(snail_nums: &[SnailfishNumber]) -> u64 {
    let mut sum = &snail_nums[0] + &snail_nums[1];
    for rhs in &snail_nums[2..] {
        sum = &sum + rhs;
    }
    sum.magnitude()
}

fn solve_18b(snail_nums: &[SnailfishNumber]) -> u64 {
    iproduct!(snail_nums.iter().enumerate(), snail_nums.iter().enumerate())
        .filter_map(|((i1, s1), (i2, s2))| (i1 != i2).then(|| (s1 + s2).magnitude()))
        .max()
        .unwrap()
}

#[derive(Copy, Clone, PartialEq)]
enum Token {
    Open,
    Close,
    Num(u8),
}

impl Token {
    fn is_n(&self) -> bool {
        matches!(self, Token::Num(_))
    }

    fn get_n(&self) -> u8 {
        match *self {
            Token::Num(n) => n,
            _ => panic!("Can only get n for Token::Num variant."),
        }
    }
}

impl Debug for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        match self {
            Token::Open => f.write_char('{'),
            Token::Close => f.write_char('}'),
            Token::Num(n) => f.write_fmt(format_args!("{}", n)),
        }
    }
}

#[derive(PartialEq, Clone)]
struct SnailfishNumber(Vec<Token>);

impl Debug for SnailfishNumber {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        let mut last_bracket = Token::Open;
        for token in &self.0 {
            match (last_bracket, token) {
                (Token::Open, Token::Open) => f.write_char('[')?,
                (_, Token::Open) => f.write_fmt(format_args!(",["))?,
                (_, Token::Close) => f.write_char(']')?,
                (Token::Open, Token::Num(n)) => f.write_fmt(format_args!("{}", n))?,
                (_, Token::Num(n)) => f.write_fmt(format_args!(",{}", n))?,
            }
            last_bracket = *token;
        }
        Ok(())
    }
}
impl SnailfishNumber {
    fn new(s: &str) -> Self {
        Self(
            s.chars()
                .filter_map(|c| match c {
                    '[' => Some(Token::Open),
                    ']' => Some(Token::Close),
                    ',' => None,
                    c => Some(Token::Num(String::from(c).parse().unwrap())),
                })
                .collect(),
        )
    }

    fn magnitude(&self) -> u64 {
        let mut magnitudes: Vec<u64> = Vec::new();
        for token in &self.0 {
            match token {
                Token::Open => (),
                Token::Close => {
                    let rhs = magnitudes.pop().unwrap();
                    let lhs = magnitudes.pop().unwrap();
                    magnitudes.push(3 * lhs + 2 * rhs);
                }
                Token::Num(n) => magnitudes.push(*n as u64),
            }
        }
        assert!(magnitudes.len() == 1, "Malformed token sequence.");
        *magnitudes.last().unwrap()
    }

    fn max_depth(&self) -> u8 {
        let mut depth = 0;
        let mut max_depth = 0;
        for (i, token) in self.0.iter().enumerate() {
            match token {
                Token::Open => {
                    depth += 1;
                    max_depth = u8::max(depth, max_depth);
                    if depth > 5 {
                        println!("{} : {:?}", i - 1, self.0[i - 1]);
                        println!("{} : {:?}", i, self.0[i]);
                        println!("{} : {:?}", i + 1, self.0[i + 1]);
                        panic!("There is a depth of {depth} at {i}.");
                    }
                }
                Token::Close => {
                    assert!(depth > 0);
                    depth -= 1;
                }
                Token::Num(_) => (),
            }
        }
        assert_eq!(depth, 0);
        max_depth
    }

    fn reduce(self) -> Self {
        let mut s1 = self;
        loop {
            let (s2, exploded) = s1.explode();
            if exploded {
                s1 = s2;
                continue;
            }
            let (s3, split) = s2.split();
            if split {
                s1 = s3;
                continue;
            }
            return s3;
        }
    }

    fn explode(self) -> (Self, bool) {
        // Concatenate both tokens.
        let mut tokens1 = self.0;
        let mut tokens2 = Vec::with_capacity(tokens1.len());

        let mut depth = 0u8;
        let mut last_num = None;
        let mut explode_right = None;
        let mut exploded = false;

        let mut token_pairs = tokens1.drain(..).tuple_windows();
        while let Some((t1, t2)) = token_pairs.next() {
            match t1 {
                Token::Open => {
                    depth += 1;
                    assert!(depth <= 5, "Depth cannot exceed 5.");
                    tokens2.push(Token::Open);
                }
                Token::Close => {
                    depth -= 1;
                    tokens2.push(Token::Close);
                }
                Token::Num(n) => {
                    if let Some(last_n) = explode_right {
                        tokens2.push(Token::Num(n + last_n));
                        break;
                    } else if depth == 5 && t2.is_n() {
                        exploded = true;
                        if let Some((last_indx, last_n)) = last_num {
                            tokens2[last_indx] = Token::Num(n + last_n);
                        }
                        let next_n = t2.get_n();
                        explode_right = Some(next_n);

                        assert_eq!(token_pairs.next(), Some((Token::Num(next_n), Token::Close))); // skip the next num

                        assert!(matches!(token_pairs.next(), Some((Token::Close, _)))); // skip ]

                        tokens2.pop();
                        tokens2.push(Token::Num(0));
                        depth -= 1;
                    } else {
                        assert!(depth <= 5);

                        let indx = tokens2.len();
                        last_num = Some((indx, n));
                        tokens2.push(t1);
                    }
                }
            }
        }
        if exploded {
            tokens2.extend(token_pairs.map(|(t1, _)| t1));
        }
        tokens2.push(Token::Close);
        (Self(tokens2), exploded)
    }

    fn split(self) -> (Self, bool) {
        let mut tokens = Vec::with_capacity(self.0.len());
        let mut token_iter = self.0.into_iter();
        let greater_than_9 = token_iter.find_map(|token| match token {
            Token::Num(n) if n > 9 => Some(n),
            _ => {
                tokens.push(token);
                None
            }
        });
        if let Some(n) = greater_than_9 {
            tokens.push(Token::Open);
            tokens.push(Token::Num(n / 2));
            tokens.push(Token::Num((n + 1) / 2));
            tokens.push(Token::Close);
            tokens.extend(token_iter);
            (Self(tokens), true)
        } else {
            (Self(tokens), false)
        }
    }
}

impl Add for &SnailfishNumber {
    /// The resulting type after applying the `+` operator.
    type Output = SnailfishNumber;

    fn add(self, rhs: Self) -> Self::Output {
        // Concatenate both tokens.
        let mut tokens1 = vec![Token::Open];
        tokens1.extend(self.0.iter().chain(rhs.0.iter()));
        tokens1.push(Token::Close);
        SnailfishNumber(tokens1).reduce()
    }
}

/// Read the input file and turn it into an Array2<u8>
fn read_input(input: &str) -> Vec<SnailfishNumber> {
    input.lines().map(SnailfishNumber::new).collect()
}
