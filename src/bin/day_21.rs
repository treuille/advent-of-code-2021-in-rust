fn main() {
    let s = State::new(4, 8);
    // let mut s = State::new(1, 5);
    // old_main(s);

    let [w1, w2] = s.wins_dirac();
    println!("w1: {w1}");
    println!("w2: {w2}");
}

#[derive(Hash, Clone)]
struct State {
    pos: [usize; 2],
    score: [usize; 2],
    player: usize,
    die: usize,
    rolls: usize,
}

impl State {
    fn new(player_1_pos: usize, player_2_pos: usize) -> Self {
        Self {
            pos: [player_1_pos - 1, player_2_pos - 1],
            score: [0, 0],
            player: 0,
            die: 99,
            rolls: 0,
        }
    }

    fn roll(&mut self) -> usize {
        self.die = (self.die + 1) % 100;
        self.die + 1
    }

    fn step_deterministic(&mut self) {
        let roll = self.roll() + self.roll() + self.roll();
        self.step(roll);
        self.rolls += 3;
    }

    fn step_dirac(&self) -> [Self; 3] {
        let mut selfs = [self.clone(), self.clone(), self.clone()];

        #[allow(clippy::needless_range_loop)]
        for i in 0..3 {
            selfs[i].step(i + 1)
        }

        selfs
    }

    fn step(&mut self, roll: usize) {
        self.pos[self.player] = (self.pos[self.player] + roll) % 10;
        self.score[self.player] += self.pos[self.player] + 1;
        self.player = (self.player + 1) % 2;
    }

    /// From this starting position, in how many universes does each player win?
    fn wins_dirac(&self) -> [usize; 2] {
        match self.score {
            [score_1, _] if score_1 >= 21 => {
                assert_eq!(self.player, 1);
                [1, 0]
            }
            [_, score_2] if score_2 >= 21 => {
                assert_eq!(self.player, 0);
                [0, 1]
            }
            _ => self
                .step_dirac()
                .into_iter()
                .fold([0, 0], |[s1, s2], next_self| {
                    let [t1, t2] = next_self.wins_dirac();
                    [s1 + t1, s2 + t2]
                }),
        }
    }
}

#[allow(dead_code)]
fn old_main(mut s: State) {
    while s.score.iter().max().unwrap() < &1000 {
        s.step_deterministic();
    }

    let loser_score = s.score.iter().min().unwrap();
    println!("loser_score: {loser_score}");
    println!("rolls: {}", s.rolls);
    println!("answer: {}", loser_score * s.rolls);
}

// fn get_die() -> impl Iterator<Item = usize> {
//     let mut state = 99;
//     iter::repeat_with(move || {
//         state = (state + 1) % 100;
//         state + 1
//     })
// }
