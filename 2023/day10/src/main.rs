use std::{
    collections::{HashSet, VecDeque},
    io::{BufRead, stdin},
};

use anyhow::{Result, anyhow};

fn main() -> Result<()> {
    let map = parse_input()?;

    println!("Part 1: {}", part1(&map)?);
    println!("Part 2: {}", part2(&map)?);

    Ok(())
}

fn parse_input() -> Result<Map> {
    let cells = stdin()
        .lock()
        .lines()
        .map(|l| Ok(l?.chars().collect()))
        .collect::<Result<Vec<_>>>()?;
    Ok(Map { cells })
}

fn part1(map: &Map) -> Result<usize> {
    let loop_tiles = map.loop_tiles().ok_or_else(|| anyhow!("loop not found"))?;
    Ok(loop_tiles.len() / 2)
}

fn part2(map: &Map) -> Result<usize> {
    let loop_tiles = map.loop_tiles().ok_or_else(|| anyhow!("loop not found"))?;
    Ok(map.enclosed_tiles(&loop_tiles))
}

#[derive(Debug)]
struct Map {
    cells: Vec<Vec<char>>,
}

const NORTH: (isize, isize) = (-1, 0);
const EAST: (isize, isize) = (0, 1);
const SOUTH: (isize, isize) = (1, 0);
const WEST: (isize, isize) = (0, -1);

impl Map {
    fn loop_tiles(&self) -> Option<HashSet<(usize, usize)>> {
        let start_loc = self.find_cell('S')?;
        let mut loc = start_loc;
        let mut dir = [NORTH, EAST, SOUTH, WEST]
            .into_iter()
            .find(|&dir| self.valid_move(start_loc, dir))?;

        let mut tiles = HashSet::from([start_loc]);
        loop {
            (loc, dir) = self.move_step(loc, dir);
            if !tiles.insert(loc) {
                break;
            }
        }
        Some(tiles)
    }

    fn find_cell(&self, cell: char) -> Option<(usize, usize)> {
        for (i, row) in self.cells.iter().enumerate() {
            for (j, &c) in row.iter().enumerate() {
                if c == cell {
                    return Some((i, j));
                }
            }
        }
        None
    }

    fn valid_move(&self, curr: (usize, usize), dir: (isize, isize)) -> bool {
        let Some((i, j)) = self.neighbor(curr, dir) else {
            return false;
        };
        let c = self.cells[i][j];
        match dir {
            NORTH => matches!(c, '|' | '7' | 'F'),
            EAST => matches!(c, '-' | 'J' | '7'),
            SOUTH => matches!(c, '|' | 'L' | 'J'),
            WEST => matches!(c, '-' | 'L' | 'F'),
            _ => false,
        }
    }

    fn move_step(
        &self,
        loc: (usize, usize),
        dir: (isize, isize),
    ) -> ((usize, usize), (isize, isize)) {
        let loc = self.neighbor(loc, dir).expect("Off map?");
        let c = self.cells[loc.0][loc.1];
        let dir = match c {
            '|' | '-' | 'S' => dir,
            'L' if dir == SOUTH => EAST,
            'L' if dir == WEST => NORTH,
            'J' if dir == EAST => NORTH,
            'J' if dir == SOUTH => WEST,
            '7' if dir == EAST => SOUTH,
            '7' if dir == NORTH => WEST,
            'F' if dir == WEST => SOUTH,
            'F' if dir == NORTH => EAST,
            _ => panic!("Off loop?"),
        };
        (loc, dir)
    }

    fn neighbor(&self, loc: (usize, usize), dir: (isize, isize)) -> Option<(usize, usize)> {
        let i = loc.0.checked_add_signed(dir.0)?;
        let j = loc.1.checked_add_signed(dir.1)?;
        self.cells.get(i)?.get(j)?;
        Some((i, j))
    }

    fn enclosed_tiles(&self, loop_tiles: &HashSet<(usize, usize)>) -> usize {
        let scaled = self.scale_up(loop_tiles);

        let mut outside: HashSet<(usize, usize)> = HashSet::from([(0, 0)]);
        let mut queue: VecDeque<(usize, usize)> = VecDeque::from([(0, 0)]);
        while let Some((x, y)) = queue.pop_front() {
            for &(xd, yd) in [NORTH, EAST, SOUTH, WEST].iter() {
                if let (Some(xn), Some(yn)) = (x.checked_add_signed(xd), y.checked_add_signed(yd))
                    && xn < scaled.len()
                    && yn < scaled[xn].len()
                    && scaled[xn][yn] == '.'
                    && outside.insert((xn, yn))
                {
                    queue.push_back((xn, yn));
                }
            }
        }

        let mut enclosed = 0;
        for (i, row) in self.cells.iter().enumerate() {
            for (j, _c) in row.iter().enumerate() {
                let (x, y) = (i * 3 + 1, j * 3 + 1);
                if scaled[x][y] == '.' && !outside.contains(&(x, y)) {
                    enclosed += 1;
                }
            }
        }

        enclosed
    }

    // Scale each pipe up to a 3 x 3 block. The center is the original tile and
    // each connection gets an arm reaching the edge of the block:
    //
    //   |     -     L     J     7     F  
    //   .#.   ...   .#.   .#.   ...   ...
    //   .#.   ###   .##   ##.   ##.   .##
    //   .#.   ...   ...   ...   .#.   .#.
    //
    // Only loop tiles are drawn, and S is drawn with an arm towards each loop
    // neighbour that connects back to it.
    //
    // A nice side effect of this is we get a flood-fillable border for "free".
    fn scale_up(&self, loop_tiles: &HashSet<(usize, usize)>) -> Vec<Vec<char>> {
        let mut scaled = vec![vec!['.'; self.cells[0].len() * 3]; self.cells.len() * 3];

        for (i, row) in self.cells.iter().enumerate() {
            for (j, &c) in row.iter().enumerate() {
                if !loop_tiles.contains(&(i, j)) {
                    continue;
                }
                match c {
                    '|' => {
                        scaled[3 * i][3 * j + 1] = '#';
                        scaled[3 * i + 1][3 * j + 1] = '#';
                        scaled[3 * i + 2][3 * j + 1] = '#';
                    }
                    '-' => {
                        scaled[3 * i + 1][3 * j] = '#';
                        scaled[3 * i + 1][3 * j + 1] = '#';
                        scaled[3 * i + 1][3 * j + 2] = '#';
                    }
                    'L' => {
                        scaled[3 * i][3 * j + 1] = '#';
                        scaled[3 * i + 1][3 * j + 1] = '#';
                        scaled[3 * i + 1][3 * j + 2] = '#';
                    }
                    'J' => {
                        scaled[3 * i][3 * j + 1] = '#';
                        scaled[3 * i + 1][3 * j + 1] = '#';
                        scaled[3 * i + 1][3 * j] = '#';
                    }
                    '7' => {
                        scaled[3 * i + 2][3 * j + 1] = '#';
                        scaled[3 * i + 1][3 * j + 1] = '#';
                        scaled[3 * i + 1][3 * j] = '#';
                    }
                    'F' => {
                        scaled[3 * i + 2][3 * j + 1] = '#';
                        scaled[3 * i + 1][3 * j + 1] = '#';
                        scaled[3 * i + 1][3 * j + 2] = '#';
                    }
                    'S' => {
                        let connects = |dir| {
                            self.valid_move((i, j), dir)
                                && self
                                    .neighbor((i, j), dir)
                                    .is_some_and(|n| loop_tiles.contains(&n))
                        };
                        scaled[3 * i + 1][3 * j + 1] = '#';
                        if connects(NORTH) {
                            scaled[3 * i][3 * j + 1] = '#';
                        }
                        if connects(EAST) {
                            scaled[3 * i + 1][3 * j + 2] = '#';
                        }
                        if connects(SOUTH) {
                            scaled[3 * i + 2][3 * j + 1] = '#';
                        }
                        if connects(WEST) {
                            scaled[3 * i + 1][3 * j] = '#';
                        }
                    }
                    _ => {}
                }
            }
        }

        scaled
    }
}
