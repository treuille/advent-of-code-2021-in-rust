use itertools::iproduct;

fn main() {
    // The target goal.
    let (goal_x_min, goal_x_max): (isize, isize) = (269, 292);
    let (goal_y_min, goal_y_max): (isize, isize) = (-68, -44);

    // The range of potentially viable start velocities. 
    let launch_vels_x = (((goal_x_min * 2) as f64).sqrt() as isize)..=goal_x_max;
    let launch_vels_y = goal_y_min..-goal_y_min;

    let mut max_height: isize = isize::MIN;
    let mut n_hits: usize = 0;

    for launch_vel in iproduct!(launch_vels_x, launch_vels_y) {
        // Launch a new problem with velocity `launch_vel`.
        let (mut x, mut y): (isize, isize) = (0, 0);
        let (mut dx, mut dy): (isize, isize) = launch_vel;
        let mut max_y: isize = 0;

        while x <= goal_x_max && y >= goal_y_min {
            if x >= goal_x_min && y <= goal_y_max {
                // We hit the goal. Record this result.
                max_height = isize::max(max_height, max_y);
                n_hits += 1;
                break;
            }

            // We haven't hit the goal yet, step the simulation.
            x += dx;
            y += dy;
            dx = isize::max(0, dx - 1);
            dy -= 1;
            max_y = isize::max(y, max_y);
        }
    }

    println!("17a: {} (2278)", max_height);
    println!("17b: {} (996)", n_hits);
}
