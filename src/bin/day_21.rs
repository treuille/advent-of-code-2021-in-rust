use std::iter;

fn main() {
    // println!("XXa: {} (123)", solve_XXa());
    // println!("XXb: {} (456)", solve_XXb());
    // let mut pos = [3, 7];
    let mut die = get_die();
    for i in 0..10 {
        println!("{i} -> {:?}", die.next());
    }
}

fn get_die() -> impl Iterator<Item = usize> {
    let mut state = 99;
    iter::repeat_with(move || {
        state = (state + 1) % 100;
        state + 1
    })
}

// fn solve_XXa() -> usize {
//     123
// }

// fn solve_XXb() -> usize {
//     456
// }
