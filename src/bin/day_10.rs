fn main() {
    let lines: Vec<Result<char, String>> = read_input();
    println!("10a: {} (369105)", solve_10a(&lines));
    println!("10b: {} (3999363569)", solve_10b(&lines));
}

fn solve_10a(lines: &[Result<char, String>]) -> usize {
    lines
        .iter()
        .filter_map(|line| match line {
            Ok(')') => Some(3),
            Ok(']') => Some(57),
            Ok('}') => Some(1197),
            Ok('>') => Some(25137),
            Ok(c) => panic!("Unspected char: {c}"),
            _ => None,
        })
        .sum()
}

fn solve_10b(lines: &[Result<char, String>]) -> usize {
    let mut completion_points: Vec<usize> = lines
        .iter()
        .filter_map(|line| match line {
            Ok(_) => None,
            Err(s) => Some(s.chars().rev().fold(0, |result, c| {
                (match c {
                    '(' => 1,
                    '[' => 2,
                    '{' => 3,
                    '<' => 4,
                    c => panic!("Unspected char: {c}"),
                }) + 5 * result
            })),
        })
        .collect();
    completion_points.sort_unstable();
    completion_points[completion_points.len() / 2]
}

/// Convert input file into a Vec whose elments are either
/// - Ok(c) if c was the corrupting character (for puzzle 10a)
/// - Err(s) if s is the in complement of the completion sequence (for puzzle 10b)
fn read_input() -> Vec<Result<char, String>> {
    include_str!("../../puzzle_inputs/day_10.txt")
        .lines()
        .map(|line| {
            let mut stack = Vec::new();
            line.chars()
                .find_map(|c| match (stack.last(), c) {
                    (_, '(') | (_, '[') | (_, '{') | (_, '<') => {
                        stack.push(c);
                        None
                    }
                    (Some('('), ')') | (Some('['), ']') | (Some('{'), '}') | (Some('<'), '>') => {
                        stack.pop();
                        None
                    }
                    (_, c) => Some(c),
                })
                .ok_or_else(|| stack.iter().collect())
        })
        .collect()
}
