use anyhow::Result;
use std::collections::HashSet;
use std::io::BufRead;

fn main() -> Result<()> {
    let initial = parse_input()?;

    let mut grid = initial.clone();
    let mut grids = HashSet::new();
    while grids.insert(grid.clone()) {
        grid = grid.next();
    }
    println!("Part 1: {}", grid.biodiversity());

    let mut grids = vec![initial.clone()];
    let empty = Grid::default();
    for _ in 0..200 {
        grids.insert(0, Grid::default());
        grids.push(Grid::default());

        let mut new_grids = grids.clone();
        let len = grids.len();

        for (i, grid) in grids.iter().enumerate() {
            let outer = if i == 0 { &empty } else { &grids[i - 1] };
            let inner = if i == len - 1 { &empty } else { &grids[i + 1] };
            new_grids[i] = grid.next_recursive(outer, inner)
        }

        grids = new_grids;
    }
    let bugs = grids.iter().map(Grid::bug_count).sum::<usize>();
    println!("Part 2: {bugs}");

    Ok(())
}

fn parse_input() -> Result<Grid> {
    let mut grid = Grid::default();
    for (x, line) in std::io::stdin().lock().lines().enumerate() {
        for (y, ch) in line?.trim().chars().enumerate() {
            if ch == '#' {
                grid.cells[x][y] = true;
            }
        }
    }
    Ok(grid)
}

#[derive(Default, Clone, Debug, Eq, PartialEq, Hash)]
struct Grid {
    cells: [[bool; 5]; 5],
}

impl Grid {
    fn next(self) -> Grid {
        let mut next = self.clone();
        for (x, row) in self.cells.iter().enumerate() {
            for (y, cell) in row.iter().enumerate() {
                let adj = self.adj_bugs((x, y), false);
                next.cells[x][y] = match cell {
                    true => adj == 1,
                    false => adj == 1 || adj == 2,
                };
            }
        }
        next
    }

    fn next_recursive(&self, outer: &Grid, inner: &Grid) -> Grid {
        let mut next = self.clone();
        for (x, row) in self.cells.iter().enumerate() {
            for (y, cell) in row.iter().enumerate() {
                if (x, y) == (2, 2) {
                    continue;
                }

                let mut adj = self.adj_bugs((x, y), true);

                {
                    let mut outer_checks = vec![];
                    if x == 0 {
                        outer_checks.push((1, 2));
                    }
                    if y == 0 {
                        outer_checks.push((2, 1));
                    }
                    if x == 4 {
                        outer_checks.push((3, 2));
                    }
                    if y == 4 {
                        outer_checks.push((2, 3));
                    }
                    adj += outer.bugs(&outer_checks);
                }

                {
                    let mut inner_checks = vec![];
                    if (x, y) == (1, 2) {
                        inner_checks.append(&mut vec![(0, 0), (0, 1), (0, 2), (0, 3), (0, 4)]);
                    }
                    if (x, y) == (2, 1) {
                        inner_checks.append(&mut vec![(0, 0), (1, 0), (2, 0), (3, 0), (4, 0)]);
                    }
                    if (x, y) == (3, 2) {
                        inner_checks.append(&mut vec![(4, 0), (4, 1), (4, 2), (4, 3), (4, 4)]);
                    }
                    if (x, y) == (2, 3) {
                        inner_checks.append(&mut vec![(0, 4), (1, 4), (2, 4), (3, 4), (4, 4)]);
                    }
                    adj += inner.bugs(&inner_checks);
                }

                next.cells[x][y] = match cell {
                    true => adj == 1,
                    false => adj == 1 || adj == 2,
                };
            }
        }
        next
    }

    fn adj_bugs(&self, (x, y): (usize, usize), filter_center: bool) -> usize {
        let mut count = 0;
        for [xd, yd] in [[0, 1], [0, -1], [-1, 0], [1, 0]] {
            let (x, y) = (x as isize + xd, y as isize + yd);
            if x >= 0
                && y >= 0
                && x < 5
                && y < 5
                && (!filter_center || ((x, y) != (2, 2)))
                && self.cells[x as usize][y as usize]
            {
                count += 1;
            }
        }
        count
    }

    fn biodiversity(&self) -> usize {
        self.cells
            .iter()
            .flatten()
            .enumerate()
            .filter_map(|(i, &bug)| {
                if bug {
                    Some(2_i32.pow(i as u32) as usize)
                } else {
                    None
                }
            })
            .sum()
    }

    fn bug_count(&self) -> usize {
        self.cells.iter().flatten().filter(|&&c| c).count()
    }

    fn bugs(&self, locs: &[(usize, usize)]) -> usize {
        locs.iter().filter(|(x, y)| self.cells[*x][*y]).count()
    }
}
