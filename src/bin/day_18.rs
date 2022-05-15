use itertools::{iproduct, Itertools};
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

#[derive(Copy, Clone)]
enum Token {
    Open,
    Close,
    Num(u8),
}

struct SnailfishNumber(Vec<Token>);

impl SnailfishNumber {
    fn new(s: &str) -> Self {
        Self(
            s.chars()
                .filter_map(|c| match c {
                    '[' => Some(Token::Open),
                    ']' => Some(Token::Close),
                    c => String::from(c).parse().ok().map(Token::Num),
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
        *magnitudes.last().unwrap()
    }

    /// Err(..) if the number exploded, Ok(..) otherwise
    fn explode(self) -> Result<Self, Self> {
        let mut depth = 0u8;
        let mut last_num = None;
        let mut exploded_with = None;

        let mut tokens = Vec::with_capacity(self.0.len());
        let mut token_pairs = self.0.into_iter().tuple_windows();
        while let Some((t1, t2)) = token_pairs.next() {
            match (t1, t2, depth, exploded_with) {
                (Token::Open, _, _, _) => depth += 1,
                (Token::Close, _, _, _) => depth -= 1,
                (Token::Num(n), _, _, Some(last_n)) => {
                    tokens.push(Token::Num(n + last_n));
                    tokens.extend(token_pairs.map(|(t1, _)| t1));
                    break;
                }
                (Token::Num(n), Token::Num(next_n), 5, None) => {
                    if let Some((last_indx, last_n)) = last_num {
                        tokens[last_indx] = Token::Num(n + last_n);
                    }
                    exploded_with = Some(next_n);

                    token_pairs.next(); // skip the next num
                    token_pairs.next(); // skip ]

                    tokens.pop();
                    tokens.push(Token::Num(0));
                    continue;
                }
                (Token::Num(n), _, _, None) => last_num = Some((tokens.len(), n)),
            }
            tokens.push(t1);
        }
        tokens.push(Token::Close);
        if exploded_with.is_some() {
            Err(Self(tokens))
        } else {
            Ok(Self(tokens))
        }
    }

    /// Err(..) if the number split, Ok(..) otherwise
    fn split(self) -> Result<Self, Self> {
        let mut tokens = Vec::with_capacity(self.0.len() + 3);
        let mut token_iter = self.0.into_iter();
        while let Some(token) = token_iter.next() {
            match token {
                Token::Num(n) if n > 9 => {
                    tokens.push(Token::Open);
                    tokens.push(Token::Num(n / 2));
                    tokens.push(Token::Num((n + 1) / 2));
                    tokens.push(Token::Close);
                    tokens.extend(token_iter);
                    return Err(Self(tokens));
                }
                _ => tokens.push(token),
            }
        }
        Ok(Self(tokens))
    }
}

impl Add for &SnailfishNumber {
    type Output = SnailfishNumber;

    fn add(self, rhs: Self) -> Self::Output {
        let mut tokens1 = vec![Token::Open];
        tokens1.extend(self.0.iter().chain(rhs.0.iter()));
        tokens1.push(Token::Close);

        let mut result = SnailfishNumber(tokens1);
        loop {
            match SnailfishNumber::explode(result).and_then(SnailfishNumber::split) {
                Ok(num) => return num,    // the number is reduced
                Err(num) => result = num, // need to keep reducing
            }
        }
    }
}

fn read_input(input: &str) -> Vec<SnailfishNumber> {
    input.lines().map(SnailfishNumber::new).collect()
}
