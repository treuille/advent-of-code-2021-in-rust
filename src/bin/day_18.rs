#![allow(dead_code)]

use itertools::Itertools;
use std::fmt::{Debug, Error, Formatter, Write};
use std::mem;
use std::ops::Add;

const TEST_INPUT: &str = "
[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]
";

const TEST_INPUT_2: &str = "
[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]
[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]
[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]
[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]
[7,[5,[[3,8],[1,4]]]]
[[2,[2,2]],[8,[8,1]]]
[2,9]
[1,[[[9,3],9],[[9,0],[0,7]]]]
[[[5,[7,4]],7],1]
[[[[4,2],2],6],[8,7]]
";

fn main() {
    let s1 = SnailfishNumber::new("[[[[4,3],4],4],[7,[[8,4],9]]]");
    let s2 = SnailfishNumber::new("[1,1]");
    let answer = SnailfishNumber::new("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]");

    println!("\ns1: {s1:?} ({})", s1.max_depth());
    println!("s2: {s2:?} ({})", s2.max_depth());
    println!("answer 2: {answer:?} ({})\n", answer.max_depth());
    assert_eq!(s1 + s2, answer);
    // println!("\ns1: {s1:?}");
    // println!("s2: {s2:?}");
    // println!("answer 1: {answer:?}\n");
    // assert_eq!(s1 + s2, answer);

    let s1 = SnailfishNumber::new("[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]");
    let s2 = SnailfishNumber::new("[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]");
    let answer = SnailfishNumber::new("[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]");

    println!("\ns1: {s1:?} ({})", s1.max_depth());
    println!("s2: {s2:?} ({})", s2.max_depth());
    println!("answer 2: {answer:?} ({})\n", answer.max_depth());
    assert_eq!(s1 + s2, answer);

    // let s = SnailfishNumber::new("[[9,1],[1,9]]");
    // println!("{s:?} -> {}", s.magnitude());

    // // let input = read_input(include_str!("../../puzzle_inputs/day_18.txt"));
    // let mut input = read_input(TEST_INPUT_2.trim()).into_iter();
    // let mut sum = input.next().unwrap();
    // for (i, next) in input.enumerate() {
    //     println!("({}) {:?}", i, sum);
    //     println!("+ {:?}", next);
    //     sum = sum + next;
    //     println!("= {:?}\n", sum);
    //     if i == 1 {
    //         break;
    //     }
    // }
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

#[derive(PartialEq)]
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
                // .skip(1)
                // .take(s.len() - 2)
                .filter_map(|c| match c {
                    '[' => Some(Token::Open),
                    ']' => Some(Token::Close),
                    ',' => None,
                    c => Some(Token::Num(String::from(c).parse().unwrap())),
                })
                .collect(),
        )
        // println!("chars: {:?}", chars.collect::<Vec<char>>());
        // todo!("SnailfishNumber constructor");
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
                    assert!(depth <= 5, "There is a depth of {depth} at {i}.");
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
}

impl Add for SnailfishNumber {
    /// The resulting type after applying the `+` operator.
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        // Concatenate both tokens.
        let mut tokens1 = vec![Token::Open];
        tokens1.extend(self.0.iter().chain(rhs.0.iter()));
        tokens1.push(Token::Close);

        let num_1 = SnailfishNumber(tokens1.clone());
        println!("tokens1: {num_1:?} ({})", num_1.max_depth());

        // This is where we write the output
        let mut tokens2 = Vec::with_capacity(tokens1.len());

        // TODO:
        // 2. Change it to tuple_windows() and remove the peek thing.
        // 3. Don't check for 10 first (check for explode_right).
        loop {
            let mut depth = u8::MIN;
            let mut last_num = None;
            let mut explode_right = None;
            let mut performed_reduction = false;

            let mut indexed_tokens = tokens1.drain(..).tuple_windows().enumerate();
            while let Some((indx, (t1, t2))) = indexed_tokens.next() {
                // println!("indx: {indx:?}");
                // println!("processing: '{:?}'", token);
                // println!("last_num: {last_num:?}");
                // println!("explode_right: {explode_right:?}");
                // println!("depth: {depth:?}\n");

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
                            performed_reduction = true;
                            break;
                        } else if n >= 10 {
                            tokens2.push(Token::Open);
                            tokens2.push(Token::Num(n / 2));
                            tokens2.push(Token::Num((n + 1) / 2));
                            tokens2.push(Token::Close);
                            performed_reduction = true;
                            break;
                        } else if depth == 5 && t2.is_n() {
                            if let Some((last_indx, last_n)) = last_num {
                                tokens2[last_indx] = Token::Num(n + last_n);
                            }
                            let next_n = t2.get_n();
                            explode_right = Some(next_n);
                            // println!("about to explode_right: {:?}", explode_right);
                            assert!(matches!(
                                indexed_tokens.next(),
                                Some((_, (Token::Num(_), Token::Close)))
                            )); // skip ]
                            assert!(matches!(
                                indexed_tokens.next(),
                                Some((_, (Token::Close, _)))
                            )); // skip ]
                            tokens2.pop();
                            tokens2.push(Token::Num(0));
                            depth -= 1;
                        } else {
                            assert!(depth <= 5);
                            last_num = Some((indx, n));
                            tokens2.push(t1);
                        }
                    }
                }
            }
            if performed_reduction {
                println!("performed_reduction=true");
                tokens2.extend(indexed_tokens.map(|(_, (t1, _))| t1));
                tokens2.push(Token::Close);
                let snail_num_2 = SnailfishNumber(tokens2.clone());
                println!("tokens2: {:?} ({})", snail_num_2, snail_num_2.max_depth());
                mem::swap(&mut tokens1, &mut tokens2);
            } else {
                println!("performed_reduction=false");
                tokens2.push(Token::Close);
                break;
            }
        }
        let snail_num_2 = SnailfishNumber(tokens2.clone());
        println!("tokens2: {:?} ({})", snail_num_2, snail_num_2.max_depth());
        Self(tokens2)
    }
}

// fn solve_XXa() -> usize {
//     123
// }

// fn solve_XXb() -> usize {
//     456
// }

/// Read the input file and turn it into an Array2<u8>
fn read_input(input: &str) -> Vec<SnailfishNumber> {
    input.lines().map(SnailfishNumber::new).collect()
}
//
// #[derive(PartialEq)]
// enum State {
//     /// Sweep, looking for reductions.
//     InitialSweep { depth: u8, last_num: Option<usize> },

//     /// Explode the next number we find to the left.
//     Explode0 { last_num: Option<usize> },

//     /// Find the number to explode to the right
//     Explode1,

//     /// Find the closeing paren the explosion.
//     Explode2 { found: u8 },

//     /// Explode the number we found to the right.
//     Explode3 { found: u8 },

//     /// Copy everything over from now on.
//     Copy,
// }

// // Perform a sequence of reductions on the number.
// loop {
//     let initial_state = State::InitialSweep {
//         depth: 0,
//         last_num: None,
//     };
//     let final_state =
//         tokens1
//             .drain(..)
//             .fold(initial_state, |state, token| match (state, token) {
//                 (State::Copy, token) => {
//                     // println!("Copy {token:?}");
//                     tokens2.push(token);
//                     State::Copy
//                 }

//                 (State::Explode3 { found }, Token::Num(n)) => {
//                     // println!("Explode right {n}");
//                     tokens2.push(Token::Num(n + found));
//                     State::Copy
//                 }

//                 (State::Explode3 { found }, token) => {
//                     // println!("Explode right {token:?}");
//                     tokens2.push(token);
//                     State::Explode3 { found }
//                 }

//                 (State::Explode2 { found }, Token::Close) => {
//                     // println!("Explode2 with close");
//                     State::Explode3 { found }
//                 }

//                 (State::Explode2 { found }, token) => {
//                     panic!("Malformed Explode2 found={found} {token:?}");
//                 }

//                 (State::Explode1, Token::Num(n)) => {
//                     // println!("Explode1 {n}");
//                     // if let Some(Token::Num(last_n)) = tokens2.last() {
//                     //     *tokens2.last_mut().unwrap() = Token::Num(last_n + n);
//                     // }
//                     // tokens2.push(Token::Num(0));
//                     tokens2.push(Token::Num(0));
//                     State::Explode2 { found: n }
//                     // State::Explode1
//                 }

//                 (State::Explode1, _) => panic!("Malformed Explode0"),

//                 (State::Explode0 { last_num }, Token::Num(n)) => {
//                     // println!("Exploding left {n}");
//                     if let Some(last_num_index) = last_num {
//                         tokens2[last_num_index] = match tokens2[last_num_index] {
//                             Token::Num(last_n) => Token::Num(last_n + n),
//                             _ => panic!("Expected a Token::Num"),
//                         }
//                     }
//                     State::Explode1
//                 }

//                 (State::Explode0 { last_num: _ }, _) => panic!("Malformed Explode0"),

//                 (State::InitialSweep { depth: 4, last_num }, Token::Open) => {
//                     // println!("Depth 4 with Open");
//                     State::Explode0 { last_num }
//                 }

//                 (State::InitialSweep { depth, last_num }, Token::Open) => {
//                     // println!("{token:?} depth={}", depth);
//                     tokens2.push(Token::Open);
//                     State::InitialSweep {
//                         depth: depth + 1,
//                         last_num,
//                     }
//                 }

//                 (State::InitialSweep { depth, last_num }, Token::Close) => {
//                     // println!("{token:?} depth={}", depth);
//                     tokens2.push(Token::Close);
//                     State::InitialSweep {
//                         depth: depth - 1,
//                         last_num,
//                     }
//                 }

//                 (State::InitialSweep { .. }, Token::Num(n)) if n > 9 => {
//                     // println!("Found a big number {n}");
//                     tokens2.push(Token::Open);
//                     tokens2.push(Token::Num(n / 2));
//                     tokens2.push(Token::Num((n + 1) / 2));
//                     tokens2.push(Token::Close);
//                     State::Copy
//                 }

//                 (State::InitialSweep { depth, last_num: _ }, Token::Num(n)) => {
//                     // println!("Need to implement for {n}");
//                     tokens2.push(Token::Num(n));
//                     State::InitialSweep {
//                         depth,
//                         last_num: Some(tokens2.len() - 1),
//                     }
//                 }
//             });
//     // println!("tokens2: {tokens2:?}");
//     match final_state {
//         State::InitialSweep { .. } => break,
//         _ => mem::swap(&mut tokens1, &mut tokens2),
//     }
// }
// // All done.
// Self(tokens2)
