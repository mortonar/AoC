use anyhow::Result;
use std::io::stdin;

fn main() -> Result<()> {
    let mut grid_state = parse_input()?;

    let mut grid_state_2 = grid_state.clone();
    grid_state_2.rules = Rules::Enhanced;

    (0..10_000).for_each(|_| grid_state.burst());
    println!("Part 1: {}", grid_state.infections_caused);

    (0..10_000_000).for_each(|_| grid_state_2.burst());
    println!("Part 2: {}", grid_state_2.infections_caused);

    Ok(())
}

/// Adjust for padding needed to not run out of bounds
const GRID_SIZE: usize = 10_000;

const MID: usize = GRID_SIZE / 2;

/// Parse what the middle of the grid looks like (puzzle input) and "move" it into the center of a
/// much larger grid.
fn parse_input() -> Result<GridState> {
    let mut middle = Vec::new();
    for line in stdin().lines() {
        let mut row_cells = Vec::new();
        for c in line?.trim_end().chars() {
            row_cells.push(match c {
                '#' => NodeState::Infected,
                _ => NodeState::Clean,
            });
        }
        middle.push(row_cells);
    }

    // Declare this on the heap because an array will blow the stack
    let mut cells = vec![vec![NodeState::Clean; GRID_SIZE]; GRID_SIZE];
    for i in 0..middle.len() {
        for j in 0..middle[i].len() {
            cells[MID + i][MID + j] = middle[i][j];
        }
    }

    Ok(GridState {
        cells,
        carrier: (MID + middle.len() / 2, MID + middle[0].len() / 2),
        dir: Dir::Up,
        infections_caused: 0,
        rules: Rules::Basic,
    })
}

#[derive(Clone)]
struct GridState {
    cells: Vec<Vec<NodeState>>,
    carrier: (usize, usize),
    dir: Dir,
    infections_caused: usize,
    rules: Rules,
}

impl GridState {
    fn burst(&mut self) {
        match self.rules {
            Rules::Basic => match self.get_carrier_state() {
                NodeState::Clean => {
                    self.dir.turn(Turn::Left);
                    self.set_carrier_cell_state(NodeState::Infected);
                    self.infections_caused += 1;
                }
                NodeState::Infected => {
                    self.dir.turn(Turn::Right);
                    self.set_carrier_cell_state(NodeState::Clean);
                }
                _ => unreachable!(),
            },
            Rules::Enhanced => match self.get_carrier_state() {
                NodeState::Clean => {
                    self.dir.turn(Turn::Left);
                    self.set_carrier_cell_state(NodeState::Weakened);
                }
                NodeState::Weakened => {
                    self.set_carrier_cell_state(NodeState::Infected);
                    self.infections_caused += 1;
                }
                NodeState::Infected => {
                    self.dir.turn(Turn::Right);
                    self.set_carrier_cell_state(NodeState::Flagged);
                }
                NodeState::Flagged => {
                    self.dir.reverse();
                    self.set_carrier_cell_state(NodeState::Clean);
                }
            },
        }

        self.advance();
    }

    fn get_carrier_state(&self) -> &NodeState {
        &self.cells[self.carrier.0][self.carrier.1]
    }

    fn set_carrier_cell_state(&mut self, state: NodeState) {
        self.cells[self.carrier.0][self.carrier.1] = state
    }

    fn advance(&mut self) {
        self.carrier = self.dir.apply(self.carrier);
    }
}

#[derive(Clone, Copy)]
enum Rules {
    Basic,
    Enhanced,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
enum NodeState {
    Clean,
    Weakened,
    Infected,
    Flagged,
}

#[derive(Clone, Copy)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

#[derive(PartialEq)]
enum Turn {
    Left,
    Right,
}

impl Dir {
    fn turn(&mut self, t: Turn) {
        *self = match self {
            Dir::Up if t == Turn::Left => Dir::Left,
            Dir::Up => Dir::Right,
            Dir::Down if t == Turn::Left => Dir::Right,
            Dir::Down => Dir::Left,
            Dir::Left if t == Turn::Left => Dir::Down,
            Dir::Left => Dir::Up,
            Dir::Right if t == Turn::Left => Dir::Up,
            Dir::Right => Dir::Down,
        };
    }

    fn reverse(&mut self) {
        *self = match self {
            Dir::Up => Dir::Down,
            Dir::Down => Dir::Up,
            Dir::Left => Dir::Right,
            Dir::Right => Dir::Left,
        }
    }

    fn apply(&self, coords: (usize, usize)) -> (usize, usize) {
        match self {
            Dir::Up => (coords.0 - 1, coords.1),
            Dir::Down => (coords.0 + 1, coords.1),
            Dir::Left => (coords.0, coords.1 - 1),
            Dir::Right => (coords.0, coords.1 + 1),
        }
    }
}
