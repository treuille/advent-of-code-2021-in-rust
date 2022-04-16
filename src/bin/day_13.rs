use anyhow::{anyhow, Error as AnyError, Result as AnyResult};
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::str::FromStr;

type Point = (usize, usize);
type Points = HashSet<Point>;

fn main() {
    // Load the input points.
    let input = include_str!("../../puzzle_inputs/day_13.txt");
    let (points, folds) = input.split_once("\n\n").unwrap();
    let mut points: Points =
        Points::from_iter(points.lines().filter_map(|line| parse_point(line).ok()));

    // Apply each fold in succession.
    let folds: Vec<Fold> = folds.lines().filter_map(|line| line.parse().ok()).collect();
    for (i, fold) in folds.iter().enumerate() {
        if i == 1 {
            println!("13a: {} (592)", points.len());
        }
        points = fold.fold(points);
    }

    // Display the second answer.
    println!("13b:");
    print_pts(&points);
    println!("(JGAJEFKU)");
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
