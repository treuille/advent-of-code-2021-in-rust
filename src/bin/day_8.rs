// digit -> segments
// 1 -> 2
// 4 -> 4
// 7 -> 3
// 8 -> 7
fn main() {
    println!("Hello, world.");
    let count = include_str!("../../puzzle_inputs/day_8.txt")
        .lines()
        .fold(0, |sum, line| {
            println!("{}", line);
            let (_, output) = line.split_once("|").unwrap();
            println!("output: {output}");
            // println!("signals: {signals}");
            let blah: Vec<_> = output.split_whitespace().collect();
            println!("blah: {blah:?}");
            let count = output
                .split_whitespace()
                .filter(|&s| {
                    let x = s.len();
                    x == 2 || x == 3 || x == 4 || x == 7
                })
                .count();
            sum + count
        });
    println!("count: {count}");
    //     .trim()
    //     .split(',')
    //     .map(|s| s.parse().unwrap())
    //     .collect();

    // input.sort();

    // // Puzzle A
    // let median = input[input.len() / 2];
    // let fuel_required: isize = input.iter().map(|x| isize::abs(x - median)).sum();
    // println!("Puzzle 7a: {} (328318)", fuel_required);

    // // Puzzle B
    // let fuel_required = input
    //     .iter()
    //     .map(|destination| {
    //         input
    //             .iter()
    //             .map(|x| {
    //                 let dist = isize::abs(destination - x);
    //                 dist * (dist + 1) / 2
    //             })
    //             .sum::<isize>()
    //     })
    //     .min()
    //     .unwrap();
    // println!("Puzzle 7b: {} (89791146)", fuel_required);
}
