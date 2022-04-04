fn main() {
    let mut input: Vec<isize> = include_str!("../../puzzle_inputs/day_7.txt")
        .trim()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();

    input.sort();

    // Puzzle A
    let median = input[input.len() / 2];
    let fuel_required: isize = input.iter().map(|x| isize::abs(x - median)).sum();
    println!("Puzzle 7a: {} (328318)", fuel_required);

    // Puzzle B
    let fuel_required = input
        .iter()
        .map(|destination| {
            input
                .iter()
                .map(|x| {
                    let dist = isize::abs(destination - x);
                    dist * (dist + 1) / 2
                })
                .sum::<isize>()
        })
        .min()
        .unwrap();
    println!("Puzzle 7b: {} (89791146)", fuel_required);
}
