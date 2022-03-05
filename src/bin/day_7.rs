fn main() {
    println!("Hello, world.");
    let mut input: Vec<isize> = include_str!("../../puzzle_inputs/day_7.txt")
        .trim()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();

    input.sort();
    let median = input[input.len() / 2];

    println!("{input:?}");

    let fuel_required: isize = input.iter().map(|x| isize::abs(x - median)).sum();
    // let deltas: Vec<isize> = input.iter().map(|x| isize::abs(x - median)).collect();
    // println!("{deltas:?}");

    let mut requirements: Vec<(isize, isize)> = input
        .iter()
        .map(|destination| {
            (
                input
                    .iter()
                    .map(|x| isize::abs(destination - x))
                    .sum::<isize>(),
                *destination,
            )
        })
        .collect();

    requirements.sort();
    println!("{:?}", &requirements[0..10]);
    println!("median: {median}");

    // Run the simulations.
    println!("Puzzle 7a: {} (328318)", fuel_required);
    println!("Puzzle 7b: {} (1695929023803)", 456);
}
