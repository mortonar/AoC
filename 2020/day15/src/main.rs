use anyhow::Result;
use std::collections::HashMap;
use std::io::stdin;

fn main() -> Result<()> {
    let state = parse_input()?;

    println!("Part 1: {}", play(&state, 2020));
    println!("Part 2: {}", play(&state, 30_000_000));

    Ok(())
}

fn parse_input() -> Result<GameState> {
    let mut line = String::new();
    stdin().read_line(&mut line)?;

    let mut numbers = HashMap::new();
    let mut last = 0;
    for (i, n) in line.trim().split(',').enumerate() {
        last = n.parse()?;
        numbers.insert(last, i + 1);
    }
    numbers.remove(&last);

    Ok(GameState::new(numbers, last))
}

#[derive(Debug, Clone)]
struct GameState {
    /// number -> last turn number was spoken
    numbers: HashMap<usize, usize>,
    /// last number spoken
    last: usize,
    turn: usize,
}

impl GameState {
    fn new(numbers: HashMap<usize, usize>, last: usize) -> GameState {
        let turn = numbers.len() + 2;
        Self {
            numbers,
            last,
            turn,
        }
    }

    fn turn(&mut self) {
        let prev_last = self.last;

        if let Some(&prev_turn) = self.numbers.get(&self.last) {
            self.last = self.turn - prev_turn - 1;
        } else {
            self.last = 0;
        }

        self.numbers.insert(prev_last, self.turn - 1);
        self.turn += 1;
    }
}

fn play(state: &GameState, rounds: usize) -> usize {
    let mut state = state.clone();
    (state.turn..=rounds).for_each(|_| state.turn());
    state.last
}
