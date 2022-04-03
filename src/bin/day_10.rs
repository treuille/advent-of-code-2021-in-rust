fn main() {
    let lines: Vec<&'static str> = read_input();
    println!("10a: {} (123)", solve_10a(&lines));
    // println!("10b: {} (456)", solve_10b());
}

fn solve_10a(lines: &[&'static str]) -> usize {
    lines
        .iter()
        .filter_map(|&line| {
            let mut stack: Vec<char> = Vec::new();
            line.chars().find_map(|c| match c {
                '(' | '[' | '{' | '<' => {
                    stack.push(c);
                    None
                }
                ')' if stack.pop() != Some('(') => Some(3),
                ']' if stack.pop() != Some('[') => Some(57),
                '}' if stack.pop() != Some('{') => Some(1197),
                '>' if stack.pop() != Some('<') => Some(25137),
                ')' | ']' | '}' | '>' => None,
                c => panic!("Unpected character: {c}"),
            })
        })
        .sum()
}

fn solve_10b() -> usize {
    456
}

/// Read the input file and turn it into an Array2<u8>
fn read_input() -> Vec<&'static str> {
    include_str!("../../puzzle_inputs/day_10.txt")
        .lines()
        .collect()
}
