use std::ops::{Range, RangeInclusive};

const TARGET_XS: RangeInclusive<isize> = 269..=292;
const TARGET_YS: RangeInclusive<isize> = -68..=-44;

fn main() {
    let min_dx = ((*TARGET_XS.start() as f64) * 2.0).sqrt() as isize;
    let max_dx = *TARGET_XS.end();
    let min_dy = *TARGET_YS.start();
    let max_dy = -*TARGET_YS.start() + 1;

    let mut max_height: isize = -1;
    let mut n_hits: usize = 0;

    for launch_dx in min_dx..=max_dx {
        for launch_dy in min_dy..=max_dy {
            let mut x: isize = 0;
            let mut y: isize = 0;
            let mut dx: isize = launch_dx;
            let mut dy: isize = launch_dy;
            let mut max_y: isize = 0;
            loop {
                x += dx;
                y += dy;
                dx = isize::max(0, dx - 1);
                dy -= 1;
                max_y = isize::max(y, max_y);
                if TARGET_XS.contains(&x) && TARGET_YS.contains(&y) {
                    max_height = isize::max(max_height, max_y);
                    n_hits += 1;
                    break;
                } else if x > *TARGET_XS.end() || y < *TARGET_YS.start() {
                    break;
                }
            }
        }
    }
    println!("the max_height is {max_height}");
    println!("there were {n_hits} hits");
    // println!("17a: {} (123)", solve_17a());;
    // println!("17b: {} (456)", solve_17b());
}
