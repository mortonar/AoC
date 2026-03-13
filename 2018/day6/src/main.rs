use anyhow::Result;
use std::collections::{HashMap, HashSet};
use std::env;
use std::io::stdin;

fn main() -> Result<()> {
    let coords = parse_input()?;
    let safe_threshold: usize = env::args().nth(1).unwrap_or("10000".to_string()).parse()?;

    // * Iterate over a "bounding box" of the coords ((min-c + 1, min-r + 1) to (max-c + 1, max-r + 1))
    // * Mark each coord in the box with the original coord it belongs to (closest to that owner and no other coord is tied in dist)
    // * Get the largest area but discounting the "infinite" ones (areas with coords on bounding box edges)
    let (max_col, max_row, min_col, min_row) = coords.iter().fold(
        (0, 0, usize::MAX, usize::MAX),
        |(max_c, max_r, min_c, min_r), &(x, y)| {
            (max_c.max(x), max_r.max(y), min_c.min(x), min_r.min(y))
        },
    );
    // Original cord -> cells within its area
    let mut areas: HashMap<(usize, usize), Vec<(usize, usize)>> = HashMap::new();
    // Size of the safe area
    let mut safe_area = 0;
    for col in min_col..max_col {
        for row in min_row..max_row {
            if let Some(owner) = coords.owner((col, row)) {
                areas
                    .entry(owner)
                    .and_modify(|area| {
                        area.push((col, row));
                    })
                    .or_insert(vec![(col, row)]);
            }

            if coords
                .iter()
                .map(|c| c.manhattan_dist((row, col)))
                .sum::<usize>()
                < safe_threshold
            {
                safe_area += 1;
            }
        }
    }
    let largest = areas
        .iter()
        .filter(|(_owner, area_cells)| {
            area_cells
                .iter()
                .all(|&(c, r)| c != min_col && c != max_col && r != min_row && r != max_row)
        })
        .max_by(|(_o1, a1), (_o2, a2)| a1.len().cmp(&a2.len()))
        .unwrap()
        .1
        .len();
    println!("Part 1: {largest}");

    println!("Part 2: {safe_area}");

    Ok(())
}

fn parse_input() -> Result<HashSet<(usize, usize)>> {
    stdin().lines().map(|l| l?.to_coord()).collect()
}

trait ToCoord {
    fn to_coord(&self) -> Result<(usize, usize)>;
}

impl ToCoord for String {
    fn to_coord(&self) -> Result<(usize, usize)> {
        let tokens: Vec<_> = self.split(", ").collect();
        Ok((tokens[0].parse()?, tokens[1].parse()?))
    }
}

trait Owner {
    fn owner(&self, coord: (usize, usize)) -> Option<(usize, usize)>;
}

impl Owner for HashSet<(usize, usize)> {
    fn owner(&self, coord: (usize, usize)) -> Option<(usize, usize)> {
        // ((c, r), dist from coord)
        let mut distances: Vec<_> = self.iter().map(|&c| (c, c.manhattan_dist(coord))).collect();
        // Sort by dist from coord
        distances.sort_by(|cd1, cd2| cd1.1.cmp(&cd2.1));
        if distances[0].1 != distances[1].1 {
            Some(distances[0].0)
        } else {
            None
        }
    }
}

trait Manhattan {
    fn manhattan_dist(&self, other: (usize, usize)) -> usize;
}

impl Manhattan for (usize, usize) {
    fn manhattan_dist(&self, other: (usize, usize)) -> usize {
        self.0.abs_diff(other.0) + self.1.abs_diff(other.1)
    }
}
