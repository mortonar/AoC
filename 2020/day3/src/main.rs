use anyhow::Result;
use std::io::stdin;

fn main() -> Result<()> {
    let map = parse_input()?;

    let p1 = map.trees_hit(1, 3);
    println!("Part 1: {p1}");
    let p2 = [(1, 1), (1, 5), (1, 7), (2, 1)]
        .iter()
        .map(|&(xd, yd)| map.trees_hit(xd, yd))
        .product::<usize>()
        * p1;
    println!("Part 2: {p2}");

    Ok(())
}

fn parse_input() -> Result<Map> {
    let cells = stdin()
        .lines()
        .map(|l| Ok(l?.trim().chars().map(|ch| ch == '#').collect::<Vec<_>>()))
        .collect::<Result<Vec<Vec<bool>>>>()?;
    Ok(Map { cells })
}

#[derive(Debug)]
struct Map {
    /// true -> tree, false -> open
    cells: Vec<Vec<bool>>,
}

impl Map {
    fn trees_hit(&self, xd: usize, yd: usize) -> usize {
        let mut trees = 0;
        let (mut x, mut y) = (0, 0);
        let cols = self.cells[0].len();
        let rows = self.cells.len();

        while x < rows {
            y %= cols;

            if self.cells[x][y] {
                trees += 1;
            }

            x += xd;
            y += yd;
        }

        trees
    }
}
