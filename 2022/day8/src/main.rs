use std::io::{BufRead, stdin};

use anyhow::Result;

fn main() -> Result<()> {
    let forrest = parse_input()?;

    println!("Part 1: {}", part1(&forrest));
    println!("Part 2: {}", part2(&forrest));

    Ok(())
}

fn parse_input() -> Result<Forrest> {
    let trees = stdin()
        .lock()
        .lines()
        .map(|l| {
            Ok(l?
                .chars()
                .map(|c| Tree::new(c.to_digit(10).unwrap() as u8))
                .collect())
        })
        .collect::<Result<Vec<_>>>()?;
    Ok(Forrest { trees })
}

fn part1(forrest: &Forrest) -> usize {
    let mut forrest = forrest.clone();
    forrest.mark_visible();
    forrest.visible_count()
}

fn part2(forrest: &Forrest) -> usize {
    let mut forrest = forrest.clone();
    forrest.calc_scenics();
    forrest.max_scenic()
}

#[derive(Debug, Clone)]
struct Forrest {
    trees: Vec<Vec<Tree>>,
}

impl Forrest {
    fn mark_visible(&mut self) {
        (0..self.trees.len()).for_each(|row| self.mark_row_vis(row));
        (0..self.trees[0].len()).for_each(|col| self.mark_col_vis(col));
    }

    fn mark_row_vis(&mut self, row: usize) {
        let row = (0..self.trees[row].len()).map(|col| (row, col));
        self.mark_visible_iter(row.clone());
        self.mark_visible_iter(row.rev());
    }

    fn mark_col_vis(&mut self, col: usize) {
        let col = (0..self.trees.len()).map(|row| (row, col));
        self.mark_visible_iter(col.clone());
        self.mark_visible_iter(col.rev());
    }

    fn mark_visible_iter(&mut self, iter: impl Iterator<Item = (usize, usize)>) {
        let mut highest = 0;
        let rows = self.trees.len() - 1;
        let cols = self.trees[0].len() - 1;
        for (row, col) in iter {
            let tree = &mut self.trees[row][col];

            let edge = row == 0 || row == rows || col == 0 || col == cols;
            if edge || tree.height > highest {
                tree.visible = true;
            }

            highest = highest.max(tree.height);
        }
    }

    fn visible_count(&self) -> usize {
        self.trees
            .iter()
            .flat_map(|row| row.iter())
            .filter(|t| t.visible)
            .count()
    }

    fn calc_scenics(&mut self) {
        let rows = self.trees.len();
        let cols = self.trees[0].len();
        for r in 0..rows {
            for c in 0..cols {
                for dir in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
                    self.calc_scenic((r, c), dir);
                }
            }
        }
    }

    fn calc_scenic(&mut self, (tree_row, tree_col): (usize, usize), (dr, dc): (isize, isize)) {
        let mut visible = 0;
        let height = self.trees[tree_row][tree_col].height;

        let (mut curr_row, mut curr_col) = (tree_row as isize, tree_col as isize);
        let rows = (self.trees.len() - 1) as isize;
        let cols = (self.trees[0].len() - 1) as isize;

        loop {
            (curr_row, curr_col) = (curr_row + dr, curr_col + dc);
            if curr_row < 0 || curr_row > rows || curr_col < 0 || curr_col > cols {
                break;
            }

            visible += 1;

            if self.trees[curr_row as usize][curr_col as usize].height >= height {
                break;
            }
        }

        self.trees[tree_row][tree_col].scenic_score *= visible
    }

    fn max_scenic(&self) -> usize {
        self.trees
            .iter()
            .flat_map(|row| row.iter())
            .map(|t| t.scenic_score)
            .max()
            .unwrap()
    }
}

#[derive(Debug, Clone)]
struct Tree {
    height: u8,
    visible: bool,
    scenic_score: usize,
}

impl Tree {
    fn new(height: u8) -> Self {
        Self {
            height,
            visible: false,
            scenic_score: 1,
        }
    }
}
