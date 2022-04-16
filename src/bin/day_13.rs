use regex::Regex;
use std::collections::{HashMap, HashSet};

type Point = (usize, usize);
type Points = HashSet<Point>;

fn main() {
    // Load the input points.
    let input = include_str!("../../puzzle_inputs/day_13.txt");
    let (points, folds) = input.split_once("\n\n").unwrap();
    let mut points: Points = Points::from_iter(points.lines().map(parse_point));

    // Solve 13a.
    let mut folds = folds.lines().map(Fold::from_str);
    points = folds.next().unwrap().fold(points);
    println!("13a: {} (592)\n", points.len());

    // Solve 13b,
    for fold in folds {
        points = fold.fold(points);
    }
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

fn parse_point(line: &str) -> Point {
    let (a, b) = line.split_once(",").unwrap();
    (a.parse().unwrap(), b.parse().unwrap())
}

enum Fold {
    X(usize),
    Y(usize),
}

impl Fold {
    fn from_str(s: &str) -> Self {
        let re = Regex::new(r"fold along ([xy])=(\d+)").unwrap();
        let caps = re.captures(s).unwrap();
        let loc = caps.get(2).unwrap().as_str().parse().unwrap();
        match caps.get(1).map(|x| x.as_str()) {
            Some("x") => Fold::X(loc),
            Some("y") => Fold::Y(loc),
            _ => panic!("Invalid fold"),
        }
    }

    fn fold(&self, points: Points) -> Points {
        Points::from_iter(points.iter().map(|&(x, y)| match *self {
            Fold::X(location) if x > location => (2 * location - x, y),
            Fold::Y(location) if y > location => (x, 2 * location - y),
            _ => (x, y),
        }))
    }
}
