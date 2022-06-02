use std::iter;

fn main() {
    // let mut pos = [0, 4];
    let mut pos = [3, 7];
    let mut score = [0, 0];
    let mut player = 0;
    let mut rolls = 0;
    let mut die = get_die();

    while score.iter().max().unwrap() < &1000 {
        let roll: usize = [die.next(), die.next(), die.next()]
            .into_iter()
            .flatten()
            .sum();
        pos[player] = (pos[player] + roll) % 10;
        score[player] += pos[player] + 1;
        player = (player + 1) % 2;
        rolls += 3;
    }
    let loser_score = score.iter().min().unwrap();
    println!("loser_score: {loser_score}");
    println!("rolls: {rolls}");
    println!("answer: {}", loser_score * rolls);
}

fn get_die() -> impl Iterator<Item = usize> {
    let mut state = 99;
    iter::repeat_with(move || {
        state = (state + 1) % 100;
        state + 1
    })
}
