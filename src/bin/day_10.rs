fn main() {
    let lines: Vec<Result<String, char>> = read_input();
    println!("10a: {} (369105)", solve_10a(&lines));
    println!("10b: {} (3999363569)", solve_10b(&lines));
}

fn solve_10a(lines: &[Result<String, char>]) -> usize {
    lines
        .iter()
        .filter_map(|line| match line {
            Err(')') => Some(3),
            Err(']') => Some(57),
            Err('}') => Some(1197),
            Err('>') => Some(25137),
            Err(c) => panic!("Unexpected char: {c}"),
            _ => None,
        })
        .sum()
}

fn solve_10b(lines: &[Result<String, char>]) -> usize {
    let mut completion_points: Vec<usize> = lines
        .iter()
        .filter_map(|line| match line {
            Ok(s) => Some(s.chars().rev().fold(0, |result, c| {
                (match c {
                    '(' => 1,
                    '[' => 2,
                    '{' => 3,
                    '<' => 4,
                    c => panic!("Unexpected char: {c}"),
                }) + 5 * result
            })),
            Err(_) => None,
        })
        .collect();
    completion_points.sort_unstable();
    completion_points[completion_points.len() / 2]
}

/// Convert input file into a Vec whose elments are either
/// - Ok(s) if s is the in complement of the completion sequence (for puzzle 10b)
/// - Err(c) if c was the corrupting character (for puzzle 10a)
fn read_input() -> Vec<Result<String, char>> {
    include_str!("../../puzzle_inputs/day_10.txt")
        .lines()
        .map(|line| {
            let mut stack = Vec::new();
            let error_char = line.chars().find_map(|c| match (stack.last(), c) {
                (_, '(') | (_, '[') | (_, '{') | (_, '<') => {
                    stack.push(c);
                    None
                }
                (Some('('), ')') | (Some('['), ']') | (Some('{'), '}') | (Some('<'), '>') => {
                    stack.pop();
                    None
                }
                (_, c) => Some(c),
            });
            match error_char {
                None => Ok(stack.iter().collect()),
                Some(error_char) => Err(error_char),
            }
        })
        .collect()
}
