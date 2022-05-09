use std::ops::{Range, RangeInclusive};

// const TARGET_XS: RangeInclusive<isize> = 269..(292+1);
// const TARGET_YS: RangeInclusive<isize> = -68..(-44+1);

const TARGET_XS: RangeInclusive<isize> = 20..=30;
const TARGET_YS: RangeInclusive<isize> = -10..=-5;

// /// The location of the Probe
// #[derive(Debug)]
// struct Probe {
//     x: isize,
//     y: isize,
//     dx: isize,
//     dy: isize,
//     max_height: isize,
// }

// #[derive(Debug)]
// enum ProbeOutcome {
//     /// The probe hit the target
//     HitTarget { max_height: isize },

//     /// Missed the target and climbed too high
//     TooHigh,

//     /// Missed target
//     TooFar,
// }

// #[derive(Debug)]
// enum ProbeStatus {
//     /// In flight, at a particular location
//     InFlight(Probe),
//     Complete(ProbeOutcome),
// }

// impl Probe {
//     fn step(self) -> ProbeStatus {
//         // - The probe's x position increases by its x velocity.
//         // - The probe's y position increases by its y velocity.
//         // - Due to drag, the probe's x velocity changes by 1 toward the value 0; that is, it decreases by 1 if it is greater than 0, increases by 1 if it is less than 0, or does not change if it is already 0.
//         // - Due to gravity, the probe's y velocity decreases by 1.
//         let new_x = self.x + self.dx;
//         let new_y = self.y + self.dy;
//         if TARGET_XS.contains(&new_x) && TARGET_YS.contains(&new_y) {
//             ProbeStatus::Complete(ProbeOutcome::HitTarget {
//                 max_height: isize::max(self.max_height, new_y),
//             })
//         } else if self.y == 0 && new_y < *TARGET_YS.start() {
//             ProbeStatus::Complete(ProbeOutcome::TooHigh)
//         } else if self.x == 0 && new_x > *TARGET_XS.end() {
//             ProbeStatus::Complete(ProbeOutcome::TooFar)
//         } else {
//             ProbeStatus::InFlight(Probe {
//                 x: new_x,
//                 y: new_y,
//                 dx: isize::max(0, self.dx - 1),
//                 dy: self.dy - 1,
//                 max_height: isize::max(self.max_height, new_y),
//             })
//         }
//     }

//     fn launch(dx: isize, dy: isize) -> ProbeOutcome {
//         let mut probe = Probe {
//             x: 0,
//             y: 0,
//             dx,
//             dy,
//             max_height: 0,
//         };
//         loop {
//             match probe.step() {
//                 ProbeStatus::InFlight(next_step) => probe = next_step,
//                 ProbeStatus::Complete(outcome) => return outcome,
//             }
//         }
//     }
// }

fn main() {
    println!(
        "TARGET_XS: {TARGET_XS:?}, {} {}",
        TARGET_XS.start(),
        TARGET_XS.end()
    );
    println!(
        "TARGET_YS: {TARGET_YS:?}, {} {}",
        TARGET_YS.start(),
        TARGET_YS.end()
    );

    let min_dx = ((*TARGET_XS.start() as f64) * 2.0).sqrt() as isize;
    let max_dx = *TARGET_XS.end();
    let min_dy = *TARGET_YS.end();
    let max_dy = -*TARGET_YS.end() + 1;

    println!("min_dx: {min_dx} {}", (0..min_dx).sum::<isize>());
    println!("max_dx: {max_dx}");
    println!("min_dy: {min_dy}");
    println!("max_dy: {max_dy}");
    // panic!("done");

    let mut max_height: isize = -1;
    for launch_dx in min_dx..=max_dx {
        for launch_dy in min_dy..=max_dy {
            let mut x: isize = 0;
            let mut y: isize = 0;
            let mut dx: isize = launch_dx;
            let mut dy: isize = launch_dy;
            let mut max_y: isize = 0;
            println!("launching with {dx},{dy}");
            loop {
                // - The probe's x position increases by its x velocity.
                // - The probe's y position increases by its y velocity.
                // - Due to drag, the probe's x velocity changes by 1 toward the value 0; that is, it decreases by 1 if it is greater than 0, increases by 1 if it is less than 0, or does not change if it is already 0.
                // - Due to gravity, the probe's y velocity decreases by 1.
                x += dx;
                y += dy;
                dx = isize::max(0, dx - 1);
                dy -= 1;
                max_y = isize::max(y, max_y);
                if TARGET_XS.contains(&x) && TARGET_YS.contains(&y) {
                    println!("Hit target at {x}, {y}");
                    max_height = isize::max(max_height, max_y);
                    break;
                } else if x > *TARGET_XS.end() || y < *TARGET_YS.start() {
                    println!("Missed target at {x}, {y}");
                    break;
                } else {
                    println!("exploring {x},{y} ({dx},{dy})");
                }
            }
        }
    }
    println!("the max_height is {max_height}");
    // for _ in 0..100 {
    //     let probe_outcome = probe.step();
    //     println!("{probe_outcome:?}");
    //     if let ProbeStatus::InFlight(next_step) = probe_outcome {
    //         probe = next_step;
    //     } else {
    //         break;
    //     }
    // }
    // println!("17a: {} (123)", solve_17a());;
    // println!("17b: {} (456)", solve_17b());
}

fn solve_17a() -> isize {
    123
}

fn solve_17b() -> isize {
    456
}

// /// Read the input file and turn it into an Array2<u8>
// fn read_input() -> Vec<&'static str> {
//     include_str!("../../puzzle_inputs/day_17.txt")
//         .lines()
//         .collect()
// }
