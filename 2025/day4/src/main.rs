use anyhow::Result;
use std::io::{BufRead, stdin};

fn main() -> Result<()> {
    let mut grid = parse_grid()?;
    let mut to_remove = grid.get_accessible();
    println!("Part 1: {}", to_remove.len());

    while !to_remove.is_empty() {
        grid.remove_rolls(&to_remove);
        to_remove = grid.get_accessible();
    }
    println!("Part 2: {}", grid.total_removed);

    Ok(())
}

fn parse_grid() -> Result<Grid> {
    let cells: Result<Vec<Vec<char>>> = stdin()
        .lock()
        .lines()
        .map(|line| Ok(line?.chars().collect()))
        .collect();
    Ok(Grid::new(cells?))
}

#[derive(Debug)]
struct Grid {
    cells: Vec<Vec<char>>,
    total_removed: usize,
}

impl Grid {
    fn new(cells: Vec<Vec<char>>) -> Self {
        Self {
            cells,
            total_removed: 0,
        }
    }

    fn get_accessible(&self) -> Vec<(usize, usize)> {
        self.cells
            .iter()
            .enumerate()
            .flat_map(|(x, row)| row.iter().enumerate().map(move |(y, char)| (x, y, *char)))
            .filter_map(|(x, y, cell)| if cell == '@' { Some((x, y)) } else { None })
            .filter(|(x, y)| {
                let adjacent = ORIENTATIONS
                    .iter()
                    .filter(|&[xd, yd]| {
                        let (x, y) = (*x as isize + xd, *y as isize + yd);
                        self.in_bounds((x, y)) && self.cells[x as usize][y as usize] == '@'
                    })
                    .count();
                adjacent < 4
            })
            .collect()
    }

    fn in_bounds(&self, coords: (isize, isize)) -> bool {
        let (x, y) = coords;
        x >= 0
            && x < self.cells.len() as isize
            && y >= 0
            && y < self.cells[x as usize].len() as isize
    }

    fn remove_rolls(&mut self, coords: &[(usize, usize)]) {
        coords.iter().for_each(|&c| self.remove_roll(c));
    }

    fn remove_roll(&mut self, coords: (usize, usize)) {
        let (x, y) = coords;
        self.cells[x][y] = '.';
        self.total_removed += 1;
    }
}

const ORIENTATIONS: [[isize; 2]; 8] = [
    [-1, -1],
    [-1, 0],
    [-1, 1],
    [0, 1],
    [1, 1],
    [1, 0],
    [1, -1],
    [0, -1],
];
