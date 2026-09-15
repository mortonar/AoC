use anyhow::Result;
use std::io::BufRead;
use std::io::stdin;

fn main() -> Result<()> {
    let sea_floor = parse_input()?;

    println!("Part 1: {}", part1(&sea_floor));

    Ok(())
}

fn parse_input() -> Result<SeaFloor> {
    let cells = stdin()
        .lock()
        .lines()
        .map(|l| Ok(l?.chars().collect::<Vec<char>>()))
        .collect::<Result<Vec<_>>>()?;
    Ok(SeaFloor { cells })
}

fn part1(sea_floor: &SeaFloor) -> usize {
    let mut sea_floor = sea_floor.clone();
    for step in 1.. {
        let moves = sea_floor.step();
        if moves == 0 {
            return step;
        }
    }
    panic!("cucumbers never stopped!")
}

#[derive(Debug, Clone)]
struct SeaFloor {
    cells: Vec<Vec<char>>,
}

#[allow(clippy::needless_range_loop)]
impl SeaFloor {
    fn step(&mut self) -> usize {
        self.move_east() + self.move_south()
    }

    fn move_east(&mut self) -> usize {
        let mut moves = 0;
        let rows = self.cells.len();
        let cols = self.cells[0].len();

        let mut next_cells = self.cells.clone();
        for col in 0..cols {
            for row in 0..rows {
                if self.cells[row][col] != '>' {
                    continue;
                }
                let next_col = (col + 1) % cols;
                if self.cells[row][next_col] == '.' {
                    next_cells[row][col] = '.';
                    next_cells[row][next_col] = '>';
                    moves += 1;
                }
            }
        }
        self.cells = next_cells;
        moves
    }

    fn move_south(&mut self) -> usize {
        let mut moves = 0;
        let mut next_cells = self.cells.clone();
        let rows = self.cells.len();
        let cols = self.cells[0].len();
        for row in 0..rows {
            for col in 0..cols {
                if self.cells[row][col] != 'v' {
                    continue;
                }
                let next_row = (row + 1) % rows;
                if self.cells[next_row][col] == '.' {
                    next_cells[row][col] = '.';
                    next_cells[next_row][col] = 'v';
                    moves += 1;
                }
            }
        }
        self.cells = next_cells;
        moves
    }
}
