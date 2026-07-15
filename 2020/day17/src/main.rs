use anyhow::Result;
use std::cmp::{max, min};
use std::collections::HashSet;
use std::io::BufRead;

fn main() -> Result<()> {
    let grid = parse_input()?;

    println!("Part 1: {}", part1(&grid));
    println!("Part 2: {}", part2(&grid));

    Ok(())
}

fn parse_input() -> Result<Grid> {
    let mut cubes = HashSet::new();
    let z = 0;
    for (x, line) in std::io::stdin().lock().lines().enumerate() {
        for (y, c) in line?.chars().enumerate() {
            if c == '#' {
                cubes.insert(vec![x as isize, y as isize, z]);
            }
        }
    }
    Ok(Grid { cubes })
}

fn part1(grid: &Grid) -> usize {
    let grid = run_cycles(grid, 6);
    grid.cubes.len()
}

fn part2(grid: &Grid) -> usize {
    let w = 0;
    let grid = Grid {
        cubes: grid
            .cubes
            .iter()
            .map(|c| {
                let mut original = c.clone();
                original.push(w);
                original
            })
            .collect(),
    };
    let grid = run_cycles(&grid, 6);
    grid.cubes.len()
}

fn run_cycles(grid: &Grid, cycles: usize) -> Grid {
    let mut grid = grid.clone();
    for _cycle in 0..cycles {
        grid = grid.cycle();
    }
    grid
}

#[derive(Debug, Clone)]
struct Grid {
    /// Active cubes at (x, y, z...)
    cubes: HashSet<Vec<isize>>,
}

impl Grid {
    fn cycle(&self) -> Self {
        let bounds = self.get_bounds();
        let mut next_grid = Self {
            cubes: HashSet::new(),
        };

        self.fill_next_grid(&bounds, 0, &mut Vec::new(), &mut next_grid);
        next_grid
    }

    /// Get min-1/max+1 per dimension
    fn get_bounds(&self) -> Vec<(isize, isize)> {
        if self.cubes.is_empty() {
            return vec![];
        }

        let dims = self.cubes.iter().next().unwrap().len();
        let mut bounds = vec![(isize::MAX, isize::MIN); dims];

        for cube in &self.cubes {
            for (i, &coord) in cube.iter().enumerate() {
                bounds[i].0 = min(coord, bounds[i].0);
                bounds[i].1 = max(coord, bounds[i].1);
            }
        }

        for (min, max) in &mut bounds {
            *min -= 1;
            *max += 1;
        }

        bounds
    }

    fn fill_next_grid(
        &self,
        bounds: &[(isize, isize)],
        dim: usize,
        current: &mut Vec<isize>,
        next_grid: &mut Grid,
    ) {
        if dim == bounds.len() {
            let active_neighbors = self.active_neighbors(current);
            let is_active = self.cubes.contains(current);

            if active_neighbors == 3 || (is_active && active_neighbors == 2) {
                next_grid.cubes.insert(current.clone());
            }
        } else {
            let (min, max) = bounds[dim];
            for coord in min..=max {
                current.push(coord);
                self.fill_next_grid(bounds, dim + 1, current, next_grid);
                current.pop();
            }
        }
    }

    fn active_neighbors(&self, cube: &[isize]) -> usize {
        let mut active_count = 0;
        self.generate_neighbors(cube, 0, &mut Vec::new(), &mut active_count);
        active_count
    }

    fn generate_neighbors(
        &self,
        original: &[isize],
        dim: usize,
        current: &mut Vec<isize>,
        active_count: &mut usize,
    ) {
        if dim == original.len() {
            if current != original && self.cubes.contains(current) {
                *active_count += 1;
            }
        } else {
            for offset in -1..=1 {
                current.push(original[dim] + offset);
                self.generate_neighbors(original, dim + 1, current, active_count);
                current.pop();
            }
        }
    }
}
