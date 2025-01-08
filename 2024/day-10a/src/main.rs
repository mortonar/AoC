use anyhow::Result;
use std::collections::HashSet;
use std::io::stdin;

fn main() -> Result<()> {
    let grid: Vec<Vec<u8>> = stdin()
        .lines()
        .flatten()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect::<Vec<u8>>()
        })
        .collect();

    let mut total = 0;
    for (i, row) in grid.iter().enumerate() {
        for (j, col) in row.iter().enumerate() {
            if *col == 0 {
                let mut nines = HashSet::new();
                score(&grid, &mut nines, (i, j));
                total += nines.len();
            }
        }
    }
    println!("{}", &total);

    Ok(())
}

fn score(grid: &Vec<Vec<u8>>, nines: &mut HashSet<(usize, usize)>, current_pos: (usize, usize)) {
    let current_val = grid[current_pos.0][current_pos.1];
    if current_val == 9 {
        nines.insert(current_pos);
        return;
    }

    for &[x_offset, y_offset] in ORIENTATIONS {
        let new_pos = (
            current_pos.0 as isize + x_offset,
            current_pos.1 as isize + y_offset,
        );

        if new_pos.0 >= 0
            && new_pos.0 < grid.len() as isize
            && new_pos.1 >= 0
            && new_pos.1 < grid[0].len() as isize
        {
            let new_pos = (new_pos.0 as usize, new_pos.1 as usize);
            if grid[new_pos.0][new_pos.1] == (current_val + 1) {
                score(grid, nines, new_pos);
            }
        }
    }
}

const ORIENTATIONS: [&[isize; 2]; 4] = [&[-1, 0], &[0, 1], &[1, 0], &[0, -1]];
