use std::collections::HashMap;

fn main() {
    let state = State::new(1, 5);
    // let state = State::new(4, 8);

    println!("21a: {} (432450)", solve_21a(state.clone()));
    println!("21b: {} (138508043837521)", solve_21b(state.clone()));
}

fn solve_21a(mut state: State) -> usize {
    let mut die: usize = 99;
    let mut rolls: usize = 0;

    while state.score.iter().max().unwrap() < &1000 {
        // Roll the die three times
        let mut die_sum = 0;
        for _ in 0..3 {
            die = (die + 1) % 100;
            die_sum += die + 1;
        }
        rolls += 3;

        // Step the simulation
        state = state.step(die_sum);
    }

    let loser_score = state.score.iter().min().unwrap();
    loser_score * rolls
}

fn solve_21b(state: State) -> usize {
    let mut wins_table: DiracWins = DiracWins::new();
    let [w1, w2] = wins_dirac(&state, &mut wins_table);
    usize::max(w1, w2)
}

/// From this starting position, in how many universes does each player win?
fn wins_dirac(state: &State, wins_table: &mut DiracWins) -> [usize; 2] {
    match state.score {
        [score_1, _] if score_1 >= 21 => {
            assert_eq!(state.player, 1);
            [1, 0]
        }
        [_, score_2] if score_2 >= 21 => {
            assert_eq!(state.player, 0);
            [0, 1]
        }
        _ => {
            #[allow(clippy::map_entry)]
            if !wins_table.contains_key(state) {
                let answer =
                    state
                        .step_dirac()
                        .fold([0, 0], |[wins_1, wins_2], (child, frequency)| {
                            let [child_w1, child_w2] = wins_dirac(&child, wins_table);
                            [wins_1 + child_w1 * frequency, wins_2 + child_w2 * frequency]
                        });
                wins_table.insert(state.clone(), answer);
            }
            wins_table[state]
        }
    }
}

type DiracWins = HashMap<State, [usize; 2]>;

#[derive(Clone, Hash, PartialEq, Eq)]
struct State {
    pos: [usize; 2],
    score: [usize; 2],
    player: usize,
}

impl State {
    fn new(player_1_pos: usize, player_2_pos: usize) -> Self {
        Self {
            pos: [player_1_pos - 1, player_2_pos - 1],
            score: [0, 0],
            player: 0,
        }
    }

    fn step(&self, roll: usize) -> Self {
        let mut next = self.clone();
        next.pos[self.player] = (self.pos[self.player] + roll) % 10;
        next.score[self.player] += next.pos[self.player] + 1;
        next.player = (self.player + 1) % 2;
        next
    }

    fn step_dirac(&self) -> impl Iterator<Item = (Self, usize)> + '_ {
        const SUMS: [(usize, usize); 7] = [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)];
        SUMS.iter()
            .map(|&(die_sum, frequency)| (self.step(die_sum), frequency))
    }
}
