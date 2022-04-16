use anyhow::{anyhow, Result as AResult};
use std::collections::HashSet;

fn main() {
    let input = include_str!("../../puzzle_inputs/day_13.txt").lines();
    let parse_point = |line: &str| -> AResult<(usize, usize)> {
        let (a, b) = line.split_once(",").ok_or_else(|| anyhow!("not a point"))?;
        Ok((a.parse()?, b.parse()?))
    };
    let points: HashSet<(usize, usize)> =
        HashSet::from_iter(input.map_while(|line| parse_point(line).ok()));
    println!("points: {points:?}");
    // println!("points: {:?}", points[0]);
    // let pair: Result<Vec<usize>, _> = line.split("-").map(|s| s.parse()).collect();
    // pair.ok().and_then(|pair| pair.next().and_then(pair.next()))});
    // // wekkk.map(|(a, b)| (a.parse().unwrap(), b.parse()))
    // }));

    // for (i, line) in read_input().into_iter().enumerate() {
    //     println!("{i}: \"{line}\"");
    // }
    // println!("13a: {} (123)", solve_13a());
    // println!("13b: {} (456)", solve_13b());
}

fn solve_13a() -> usize {
    123
}

fn solve_13b() -> usize {
    456
}

// /// Read the input file and turn it into an Array2<u8>
// fn read_input() -> Vec<&'static str> {}
