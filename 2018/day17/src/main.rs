use anyhow::Result;
use std::io::stdin;

fn main() -> Result<()> {
    let mut ground = parse_input()?;
    ground.flow();
    println!("Part 1: {}", ground.count_cells(|c| matches!(c, '|' | '~')));
    println!("Part 2: {}", ground.count_cells(|c| matches!(c, '~')));

    Ok(())
}

fn parse_input() -> Result<Ground> {
    let coords: Vec<_> = stdin()
        .lines()
        .map(|l| parse_coord_range(&l?))
        .collect::<Result<_>>()?;

    let (max_x, max_y, min_y) = coords.iter().fold(
        (0, 0, usize::MAX),
        |(max_x, max_y, min_y), ((_, xr), (yl, yr))| {
            (max_x.max(*xr), max_y.max(*yr), min_y.min(*yl))
        },
    );

    let mut cells = vec![vec!['.'; max_x + 5]; max_y + 1];
    for ((xl, xr), (yl, yr)) in coords {
        for row in &mut cells[yl..=yr] {
            row[xl..=xr].fill('#');
        }
    }

    Ok(Ground {
        cells,
        max_y,
        min_y,
    })
}

fn parse_coord_range(line: &str) -> Result<((usize, usize), (usize, usize))> {
    let [a, b]: [_; 2] = line
        .split(", ")
        .map(parse_coord)
        .collect::<Result<Vec<_>>>()?
        .try_into()
        .map_err(|_| anyhow::anyhow!("Expected exactly two coord ranges"))?;

    if line.starts_with('x') {
        Ok((a, b))
    } else {
        Ok((b, a))
    }
}

fn parse_coord(s: &str) -> Result<(usize, usize)> {
    let value = &s[2..];
    if let Some((start, end)) = value.split_once("..") {
        Ok((start.parse()?, end.parse()?))
    } else {
        let n = value.parse()?;
        Ok((n, n))
    }
}

struct Ground {
    cells: Vec<Vec<char>>,
    max_y: usize,
    min_y: usize,
}

impl Ground {
    fn flow(&mut self) {
        self.cells[0][500] = '+';
        self.fill(500, 1);
    }

    // Returns true if this cell drains (water flows out), false if it's contained
    fn fill(&mut self, x: usize, y: usize) -> bool {
        if y > self.max_y {
            return true;
        }

        if self.cells[y][x] == '#' || self.cells[y][x] == '~' {
            return false;
        }

        if self.cells[y][x] == '|' {
            return true;
        }

        self.cells[y][x] = '|';

        let drains_down = self.fill(x, y + 1);

        if drains_down {
            return true;
        }

        let (left_x, left_drains) = self.spread(x, y, -1);
        let (right_x, right_drains) = self.spread(x, y, 1);

        let drains = left_drains || right_drains;

        if !drains {
            for wx in left_x..=right_x {
                self.cells[y][wx] = '~';
            }
        }

        drains
    }

    // Spread in a direction, returns (edge_x, drains)
    fn spread(&mut self, x: usize, y: usize, dx: isize) -> (usize, bool) {
        let mut cx = x;
        loop {
            let next_x = (cx as isize + dx) as usize;

            if self.cells[y][next_x] == '#' {
                return (cx, false);
            }

            self.cells[y][next_x] = '|';

            // Check if floor disappears
            let below = self.cells[y + 1][next_x];
            if below != '#' && below != '~' {
                let drains = self.fill(next_x, y + 1);
                if drains {
                    return (next_x, true);
                }
            }

            cx = next_x;
        }
    }

    fn count_cells(&self, pred: impl Fn(char) -> bool) -> usize {
        self.cells[self.min_y..=self.max_y]
            .iter()
            .flat_map(|row| row.iter())
            .filter(|c| pred(**c))
            .count()
    }
}
