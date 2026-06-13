use anyhow::{Result, anyhow, bail};
use intcode::{IntcodeComputer, Program, RunState, parse_program};
use std::collections::{HashSet, VecDeque};

fn main() -> Result<()> {
    let program = parse_input()?;

    let map = map_ship_bfs(&program)?;
    println!("Part 1: {}", map.min_to_oxygen);
    println!("Part 2: {}", map.min_fill_oxygen());

    Ok(())
}

fn parse_input() -> Result<Program> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line)?;
    parse_program(&line)
}

fn map_ship_bfs(program: &Program) -> Result<ShipMap> {
    let mut queue = VecDeque::new();
    queue.push_back(SearchContext::new(program));
    let mut visited = HashSet::new();
    visited.insert((0, 0));
    let mut walls = HashSet::new();
    let mut oxygen_sys = None;

    while let Some(current) = queue.pop_front() {
        for [command, x, y] in [[1, -1, 0], [2, 1, 0], [3, 0, -1], [4, 0, 1]] {
            let mut neighbor = current.clone();
            neighbor.computer.input.push_back(command);
            neighbor.x += x;
            neighbor.y += y;
            neighbor.movements += 1;

            match neighbor.computer.run() {
                RunState::ProducedOutput => {
                    let output = neighbor.computer.output.pop_front().unwrap();
                    match output {
                        0 => {
                            walls.insert((neighbor.x, neighbor.y));
                        }
                        1 | 2 => {
                            if visited.insert((neighbor.x, neighbor.y)) {
                                if output == 2 {
                                    oxygen_sys =
                                        Some(((neighbor.x, neighbor.y), neighbor.movements));
                                }
                                queue.push_back(neighbor);
                            }
                        }
                        output => bail!("Unexpected output: {output}"),
                    }
                }
                run_state => bail!("Unexpected run state: {run_state:?}"),
            }
        }
    }

    oxygen_sys
        .map(|(oxygen_sys, min_to_oxygen)| ShipMap {
            open_cells: visited,
            walls,
            oxygen_sys,
            min_to_oxygen,
        })
        .ok_or_else(|| anyhow!("No solution found"))
}

#[derive(Debug, Clone)]
struct SearchContext {
    computer: IntcodeComputer,
    x: isize,
    y: isize,
    movements: usize,
}

struct ShipMap {
    open_cells: HashSet<(isize, isize)>,
    walls: HashSet<(isize, isize)>,
    oxygen_sys: (isize, isize),
    min_to_oxygen: usize,
}

impl ShipMap {
    fn min_fill_oxygen(&self) -> usize {
        let mut minutes = 0;
        let mut filled = HashSet::from([self.oxygen_sys]);

        while self.open_cells.iter().any(|c| !filled.contains(c)) {
            let new: HashSet<_> = filled
                .iter()
                .flat_map(|&(x, y)| [(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)].into_iter())
                .filter(|n| !self.walls.contains(n))
                .collect();
            filled.extend(new.into_iter());
            minutes += 1;
        }

        minutes
    }
}

impl SearchContext {
    fn new(program: &Program) -> Self {
        Self {
            computer: IntcodeComputer::new(program),
            x: 0,
            y: 0,
            movements: 0,
        }
    }
}
