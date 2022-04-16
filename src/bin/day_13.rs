use anyhow::{anyhow, Error as AnyError, Result as AnyResult};
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::str::FromStr;

type Point = (usize, usize);
type Points = HashSet<Point>;

#[allow(dead_code)]
const TEST_INPUT: &str = "
6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5
";

fn main() {
    let input = include_str!("../../puzzle_inputs/day_13.txt");
    let (points, folds) = input.split_once("\n\n").unwrap();
    // let (points, folds) = TEST_INPUT.split_once("\n\n").unwrap();
    let points: Points =
        Points::from_iter(points.lines().filter_map(|line| parse_point(line).ok()));
    println!("points: {points:?}");
    print_pts(&points);

    let folds: Vec<Fold> = folds.lines().filter_map(|line| line.parse().ok()).collect();
    println!("folds: {folds:?}");

    let points = folds[0].fold(points);
    print_pts(&points);
    println!("answer: {}", points.len());
}

fn print_pts(points: &Points) {
    let w = *points.iter().map(|(x, _)| x).max().unwrap();
    let h = *points.iter().map(|(_, y)| y).max().unwrap();
    let display_char = HashMap::from([(false, ","), (true, "X")]);
    for y in 0..=h {
        for x in 0..=w {
            print!("{}", display_char[&points.contains(&(x, y))]);
        }
        println!();
    }
}

#[allow(dead_code)]
fn print_pts_old(points: &Points) {
    let width = points.iter().map(|(x, _)| x).max().unwrap();
    let mut points: Vec<&Point> = points.iter().collect();
    points.sort_unstable_by_key(|(x, y)| (y, x));
    let mut points = points.into_iter().peekable();
    for y in 0usize.. {
        if points.peek().is_none() {
            break;
        }
        for x in 0..=*width {
            match points.peek() {
                Some(&&pt) if pt == (x, y) => {
                    points.next();
                    print!("X");
                }
                _ => print!(","),
            }
        }
        println!();
    }
}

fn parse_point(line: &str) -> AnyResult<Point> {
    let (a, b) = line.split_once(",").ok_or_else(|| anyhow!("not a point"))?;
    Ok((a.parse()?, b.parse()?))
}

#[derive(Debug)]
enum Fold {
    X(usize),
    Y(usize),
}

impl Fold {
    fn fold(&self, points: Points) -> Points {
        Points::from_iter(points.iter().map(|&(x, y)| match *self {
            Fold::X(location) if x > location => (2 * location - x, y),
            Fold::Y(location) if y > location => (x, 2 * location - y),
            Fold::X(location) if x == location => panic!("Point on fold"),
            Fold::X(location) if y == location => panic!("Point on fold"),
            _ => (x, y),
        }))
    }
}

impl FromStr for Fold {
    type Err = AnyError;

    fn from_str(s: &str) -> AnyResult<Fold> {
        let re = Regex::new(r"fold along ([xy])=(\d+)")?;
        let caps = re.captures(s).ok_or_else(|| anyhow!("Invalid fold"))?;
        let loc = caps
            .get(2)
            .ok_or_else(|| anyhow!("Missing location"))?
            .as_str()
            .parse()?;
        match caps.get(1).map(|x| x.as_str()) {
            Some("x") => Ok(Fold::X(loc)),
            Some("y") => Ok(Fold::Y(loc)),
            _ => Err(anyhow!("Invalid dimension")),
        }
    }
}

fn solve_13a() -> usize {
    123
}

fn solve_13b() -> usize {
    456
}

// /// Read the input file and turn it into an Array2<u8>
// fn read_input() -> Vec<&'static str> {}
