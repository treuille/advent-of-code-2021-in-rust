use cached::proc_macro::cached;

fn main() {
    let state = GameState::new(1, 5);

    println!("21a: {} (432450)", solve_21a(state.clone()));
    println!("21b: {} (138508043837521)", solve_21b(state.clone()));
}

fn solve_21a(mut state: GameState) -> usize {
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

fn solve_21b(state: GameState) -> usize {
    let [player_1_wins, player_2_wins] = dirac_wins(state);
    usize::max(player_1_wins, player_2_wins)
}

#[derive(Clone, Hash, PartialEq, Eq)]
struct GameState {
    pos: [usize; 2],
    score: [usize; 2],
    player: usize,
}

impl GameState {
    fn new(player_1_pos: usize, player_2_pos: usize) -> Self {
        Self {
            pos: [player_1_pos - 1, player_2_pos - 1],
            score: [0, 0],
            player: 0,
        }
    }

    /// Advance the game state after summing three dice rolls.
    fn step(&self, die_sum: usize) -> Self {
        let mut next = self.clone();
        next.pos[self.player] = (self.pos[self.player] + die_sum) % 10;
        next.score[self.player] += next.pos[self.player] + 1;
        next.player = (self.player + 1) % 2;
        next
    }

    /// Compute all possible future Dirac-states and their frequencies.
    fn step_dirac(&self) -> impl Iterator<Item = (Self, usize)> + '_ {
        // All the possible sums of thrice rolling a 3-sided die, and their frequencies
        const SUMS: [(usize, usize); 7] = [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)];
        SUMS.iter()
            .map(|&(die_sum, frequency)| (self.step(die_sum), frequency))
    }
}

/// Find in how many Dirac-die universes each player will win from this start state.
#[cached]
fn dirac_wins(state: GameState) -> [usize; 2] {
    match state.score {
        [score_1, _] if score_1 >= 21 => [1, 0],
        [_, score_2] if score_2 >= 21 => [0, 1],
        _ => state
            .step_dirac()
            .fold([0, 0], |[wins_1, wins_2], (child, frequency)| {
                let [child_w1, child_w2] = dirac_wins(child);
                [wins_1 + child_w1 * frequency, wins_2 + child_w2 * frequency]
            }),
    }
}
