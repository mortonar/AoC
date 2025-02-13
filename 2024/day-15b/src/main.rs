use anyhow::{anyhow, Context, Result};
use std::io;
use std::ops::{Index, IndexMut};

fn main() -> Result<()> {
    let mut lines = io::stdin().lines();

    let mut warehouse = Warehouse::default();
    loop {
        let line = lines.next().context("Failed to read line")??;
        if line.trim().is_empty() {
            break;
        }
        let mut row = Vec::new();
        for c in line.chars() {
            let mut to_append = if c == 'O' {
                vec!['[', ']']
            } else if c == '@' {
                vec!['@', '.']
            } else {
                vec![c, c]
            };
            row.append(&mut to_append);
        }
        warehouse.push(row);
    }

    let mut moves = Vec::new();
    for line in lines {
        line?.trim().chars().for_each(|c| moves.push(c));
    }

    // print_warehouse(&warehouse.grid);
    play_moves(&mut warehouse, &moves)?;
    // print_warehouse(&warehouse.grid);

    let total: usize = warehouse
        .grid
        .iter()
        .enumerate()
        .flat_map(|(i, row)| row.iter().enumerate().map(move |(j, &c)| (i, j, c)))
        .filter(|(_i, _j, c)| *c == '[')
        .map(|(i, j, _c)| 100 * i + j)
        .sum();
    println!("{}", total);

    Ok(())
}

#[allow(dead_code)]
fn print_warehouse(warehouse: &Vec<Vec<char>>) {
    for i in 0..warehouse.len() {
        for j in 0..warehouse[i].len() {
            print!("{}", warehouse[i][j]);
        }
        println!();
    }
    println!("----------------------------------------------------------------------");
}

fn play_moves(warehouse: &mut Warehouse, moves: &[char]) -> Result<()> {
    let robot = warehouse
        .grid
        .iter()
        .enumerate()
        .flat_map(|(i, row)| row.iter().enumerate().map(move |(j, &c)| (i, j, c)))
        .find(|(_i, _j, c)| *c == '@')
        .context("Robot not found")?;
    let mut robot = (robot.0, robot.1);

    for &m in moves {
        let orient: (isize, isize) = match m {
            '<' => (0, -1),
            '^' => (-1, 0),
            '>' => (0, 1),
            'v' => (1, 0),
            _ => return Err(anyhow!("Invalid move")),
        };

        let next_coords = (
            (robot.0 as isize + orient.0) as usize,
            (robot.1 as isize + orient.1) as usize,
        );

        let next = warehouse[next_coords];
        match next {
            '#' => continue,
            '[' | ']' => {
                let boxes = get_boxes(&warehouse, next_coords, orient);
                if !boxes.is_empty() {
                    for &b in boxes.iter().rev() {
                        let next = (
                            (b.0 as isize + orient.0) as usize,
                            (b.1 as isize + orient.1) as usize,
                        );
                        warehouse[next] = warehouse[b];
                        warehouse[b] = '.';
                    }

                    warehouse[robot] = '.';
                    robot = next_coords;
                    warehouse[robot] = '@';
                }
            }
            _ => {
                warehouse[robot] = '.';
                robot = next_coords;
                warehouse[robot] = '@';
            }
        }
    }

    Ok(())
}

// Search for all boxes via BFS in the given orientation from the starting location.
// Vertical orientations must expand the search to include the full width of the box.
fn get_boxes(
    warehouse: &Warehouse,
    start: (usize, usize),
    orient: (isize, isize),
) -> Vec<(usize, usize)> {
    let mut boxes = vec![];
    let mut queue = vec![start];

    while !queue.is_empty() {
        let next = queue.remove(0);
        let next_space = warehouse[next];
        match next_space {
            '[' | ']' => {
                boxes.push(next);
                // If we're moving up or down, we need to account for width of the whole box pushing adjacent boxes in other columns
                if orient.0 != 0 {
                    let pair = if next_space == '[' {
                        (next.0, next.1 + 1)
                    } else {
                        (next.0, next.1 - 1)
                    };
                    if !queue.contains(&pair) && !boxes.contains(&pair) {
                        queue.push(pair);
                    }
                }
                queue.push((
                    (next.0 as isize + orient.0) as usize,
                    (next.1 as isize + orient.1) as usize,
                ));
            }
            c => {
                // We hit a wall.
                // From the examples, a group of boxes moves as one large box rather than independently.
                if c == '#' {
                    boxes.clear();
                    break;
                }
            }
        }
    }

    boxes
}

#[derive(Default)]
struct Warehouse {
    grid: Vec<Vec<char>>,
}

impl Warehouse {
    fn push(&mut self, row: Vec<char>) {
        self.grid.push(row);
    }
}

impl Index<(usize, usize)> for Warehouse {
    type Output = char;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.grid[index.0][index.1]
    }
}

impl IndexMut<(usize, usize)> for Warehouse {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        &mut self.grid[index.0][index.1]
    }
}
