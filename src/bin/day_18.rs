#![allow(dead_code)]

use itertools::{iproduct, Itertools};
use std::fmt::{Debug, Error, Formatter, Write};
// use std::mem;
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
    // let mut input = read_input(include_str!("../../puzzle_inputs/day_18.txt")).into_iter();
    // // let mut input = read_input(TEST_INPUT_2.trim()).into_iter();
    // let mut sum = input.next().unwrap();
    // for (i, next) in input.enumerate() {
    //     println!("({}) {:?}", i, sum);
    //     println!("+ {:?}", next);
    //     sum = sum + next;
    //     println!("= {:?}\n", sum);
    //     // if i == 1 {
    //     //     break;
    //     // }
    // }
    // println!("sum magnitude: {} (4145)", sum.magnitude());

    let input = read_input(include_str!("../../puzzle_inputs/day_18.txt"));
    let answer = iproduct!(input.iter().enumerate(), input.iter().enumerate())
        .filter_map(|((i1, s1), (i2, s2))| (i1 != i2).then(|| (s1 + s2).magnitude()))
        .max()
        .unwrap();
    // // let mut input = read_input(TEST_INPUT_2.trim()).into_iter();
    // let mut sum = input.next().unwrap();
    // for (i, next) in input.enumerate() {
    //     println!("({}) {:?}", i, sum);
    //     println!("+ {:?}", next);
    //     sum = sum + next;
    //     println!("= {:?}\n", sum);
    //     // if i == 1 {
    //     //     break;
    //     // }
    // }
    println!(": {} (4145)", answer);
}

// fn test_reductions() {
//     let examples = [
//         (
//             SnailfishNumber::new("[[[[[9,8],1],2],3],4]"),
//             SnailfishNumber::new("[[[[0,9],2],3],4]"),
//         ),
//         (
//             SnailfishNumber::new("[7,[6,[5,[4,[3,2]]]]]"),
//             SnailfishNumber::new("[7,[6,[5,[7,0]]]]"),
//         ),
//         (
//             SnailfishNumber::new("[[6,[5,[4,[3,2]]]],1]"),
//             SnailfishNumber::new("[[6,[5,[7,0]]],3]"),
//         ),
//         (
//             SnailfishNumber::new("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]"),
//             SnailfishNumber::new("[[3,[2,[8,0]]],[9,[5,[7,0]]]]"),
//             // SnailfishNumber::new("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]"),
//         ),
//         (
//             SnailfishNumber::new("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]"),
//             SnailfishNumber::new("[[3,[2,[8,0]]],[9,[5,[7,0]]]]"),
//         ),
//         (
//             SnailfishNumber::new("[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]"),
//             SnailfishNumber::new("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]"),
//         ),
//     ];
//     for (s1, s2) in examples {
//         // println!("s1: {:?}", s1);
//         println!("s1a: {:?}", s1);
//         println!("s2a: {:?}", s2);
//         let s1 = s1.reduce();
//         println!("s1b: {:?}", s1);
//         println!("s2b: {:?}", s2);
//         assert_eq!(s1, s2);
//         println!();
//     }
//     println!("All tests passed.");
// }

// fn final_summation_example() {
//     let mut sum = SnailfishNumber::new("[1,1]");
//     for i in 2..=6 {
//         let rhs = SnailfishNumber::new(format!("[{i},{i}]").as_str());
//         println!("rhs: {:?}", rhs);
//         sum = *(&sum + &rhs);
//         println!("sum: {:?}", sum);
//     }
// }

// fn big_summation_example() {
//     let examples = [
//         (
//             SnailfishNumber::new("[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]"),
//             SnailfishNumber::new("[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]"),
//             SnailfishNumber::new("[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]"),
//         ),
//         (
//             SnailfishNumber::new("[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]"),
//             SnailfishNumber::new("[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]"),
//             SnailfishNumber::new("[[[[6,7],[6,7]],[[7,7],[0,7]]],[[[8,7],[7,7]],[[8,8],[8,0]]]]"),
//         ),
//         (
//             SnailfishNumber::new("[[[[6,7],[6,7]],[[7,7],[0,7]]],[[[8,7],[7,7]],[[8,8],[8,0]]]]"),
//             SnailfishNumber::new("[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]"),
//             SnailfishNumber::new("[[[[7,0],[7,7]],[[7,7],[7,8]]],[[[7,7],[8,8]],[[7,7],[8,7]]]]"),
//         ),
//         (
//             SnailfishNumber::new("[[[[7,0],[7,7]],[[7,7],[7,8]]],[[[7,7],[8,8]],[[7,7],[8,7]]]]"),
//             SnailfishNumber::new("[7,[5,[[3,8],[1,4]]]]"),
//             SnailfishNumber::new("[[[[7,7],[7,8]],[[9,5],[8,7]]],[[[6,8],[0,8]],[[9,9],[9,0]]]]"),
//         ),
//         (
//             SnailfishNumber::new("[[[[7,7],[7,8]],[[9,5],[8,7]]],[[[6,8],[0,8]],[[9,9],[9,0]]]]"),
//             SnailfishNumber::new("[[2,[2,2]],[8,[8,1]]]"),
//             SnailfishNumber::new("[[[[6,6],[6,6]],[[6,0],[6,7]]],[[[7,7],[8,9]],[8,[8,1]]]]"),
//         ),
//         (
//             SnailfishNumber::new("[[[[6,6],[6,6]],[[6,0],[6,7]]],[[[7,7],[8,9]],[8,[8,1]]]]"),
//             SnailfishNumber::new("[2,9]"),
//             SnailfishNumber::new("[[[[6,6],[7,7]],[[0,7],[7,7]]],[[[5,5],[5,6]],9]]"),
//         ),
//         (
//             SnailfishNumber::new("[[[[6,6],[7,7]],[[0,7],[7,7]]],[[[5,5],[5,6]],9]]"),
//             SnailfishNumber::new("[1,[[[9,3],9],[[9,0],[0,7]]]]"),
//             SnailfishNumber::new("[[[[7,8],[6,7]],[[6,8],[0,8]]],[[[7,7],[5,0]],[[5,5],[5,6]]]]"),
//         ),
//         (
//             SnailfishNumber::new("[[[[7,8],[6,7]],[[6,8],[0,8]]],[[[7,7],[5,0]],[[5,5],[5,6]]]]"),
//             SnailfishNumber::new("[[[5,[7,4]],7],1]"),
//             SnailfishNumber::new("[[[[7,7],[7,7]],[[8,7],[8,7]]],[[[7,0],[7,7]],9]]"),
//         ),
//         (
//             SnailfishNumber::new("[[[[7,7],[7,7]],[[8,7],[8,7]]],[[[7,0],[7,7]],9]]"),
//             SnailfishNumber::new("[[[[4,2],2],6],[8,7]]"),
//             SnailfishNumber::new("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]"),
//         ),
//     ];

//     for (lhs, rhs, sum) in examples.into_iter().skip(1) {
//         let my_sum = lhs.clone() + rhs.clone();
//         println!("lhs: {lhs:?}");
//         println!("rhs: {rhs:?}");
//         println!("my_sum: {my_sum:?}");
//         println!("sum: {sum:?}");
//         // break;
//     }
//     println!("All tests passed - big_summation_example()");
// }

fn test_explode() {
    let examples = [
        (
            SnailfishNumber::new("[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]"),
            SnailfishNumber::new("[[[[0,7],4],[7,[[8,4],9]]],[1,1]]"),
        ),
        // (
        //     SnailfishNumber::new("[[[[0,7],4],[7,[[8,4],9]]],[1,1]]"),
        //     SnailfishNumber::new("[[[[0,7],4],[15,[0,13]]],[1,1]]"),
        // ),
        (
            SnailfishNumber::new("[[[[0,7],4],[[7,8],[0,[6,7]]]],[1,1]]"),
            SnailfishNumber::new("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]"),
        ),
    ];

    for (s1, s2) in examples {
        println!("s1: {s1:?}");
        let (s1, exploded) = s1.explode();
        assert!(exploded, "Should have exploded.");
        println!("s1: {s1:?}");
        println!("s2: {s2:?}");
        assert_eq!(s1, s2);
    }
    println!("All tests passed.");
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

        // Debug
        let num_1 = SnailfishNumber(tokens1.clone());
        println!("tokens1: {num_1:?} ({})", num_1.max_depth());

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
            println!("exploded");
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

    //         while let Some((t1, t2)) = token_pairs.next() {
    //             match t1 {
    //                 Token::Open => {
    //                     depth += 1;
    //                     assert!(depth <= 5, "Depth cannot exceed 5.");
    //                     tokens2.push(Token::Open);
    //                 }
    //                 Token::Close => {
    //                     depth -= 1;
    //                     tokens2.push(Token::Close);
    //                 }
    //                 Token::Num(n) => {
    //                     // if n == 19 {
    //                     //     println!("Found a 19!!!");
    //                     // }
    //                     if let Some(last_n) = explode_right {
    //                         tokens2.push(Token::Num(n + last_n));
    //                         break;
    //                     } else if depth == 5 && t2.is_n() {
    //                         performed_reduction = true;
    //                         if let Some((last_indx, last_n)) = last_num {
    //                             tokens2[last_indx] = Token::Num(n + last_n);
    //                         }
    //                         let next_n = t2.get_n();
    //                         explode_right = Some(next_n);

    //                         assert_eq!(token_pairs.next(), Some((Token::Num(next_n), Token::Close))); // skip the next num

    //                         assert!(matches!(token_pairs.next(), Some((Token::Close, _)))); // skip ]

    //                         tokens2.pop();
    //                         tokens2.push(Token::Num(0));
    //                         depth -= 1;
    //                     } else if depth > 5 {
    //                         panic!("Depth too high: {depth}");
    //                     } else if n >= 10 {
    //                         performed_reduction = true;

    //                         break;
    //                     } else {
    //                         assert!(depth <= 5);

    //                         let indx = tokens2.len();
    //                         last_num = Some((indx, n));
    //                         tokens2.push(t1);
    //                     }
    //                 }
    //             }
    //         }
    //         if performed_reduction {
    //             println!("performed_reduction=true");
    //             tokens2.extend(token_pairs.map(|(t1, _)| t1));
    //             tokens2.push(Token::Close);
    //             let snail_num_2 = SnailfishNumber(tokens2.clone());
    //             print!("tokens2: {:?} (", snail_num_2);
    //             println!("{})", snail_num_2.max_depth());
    //             mem::swap(&mut tokens1, &mut tokens2);
    //         } else {
    //             println!("performed_reduction=false");
    //             tokens2.push(Token::Close);
    //             break;
    //         }
    //         // end of the road
    //         todo!("end of the road");
    //         let snail_num_2 = SnailfishNumber(tokens2.clone());
    //         print!("tokens2: {:?} (", snail_num_2);
    //         println!("{})", snail_num_2.max_depth());
    //         Self(tokens2)
    //     }
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
