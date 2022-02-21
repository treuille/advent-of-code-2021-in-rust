fn main() {
    // Parse the input
    let mut generations = [0usize; 9];
    include_str!("../../puzzle_inputs/day_6.txt")
        .trim()
        .split(',')
        .for_each(|s| {
            let fish_age: usize = s.parse().unwrap();
            generations[fish_age] += 1;
        });

    // Run the simulations.
    println!("Puzzle 6a: {} (377263)", sim(generations, 80));
    println!("Puzzle 6b: {} (1695929023803)", sim(generations, 256));
}

/// Returns the number of latern fish after `days` days of simulation.
fn sim(mut generations: [usize; 9], days: usize) -> usize {
    for day in 0..days {
        generations[(day + 7) % 9] += generations[day % 9];
    }
    generations.into_iter().sum()
}
