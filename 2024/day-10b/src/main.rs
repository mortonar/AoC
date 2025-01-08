use std::io::stdin;

fn main() {
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
                total += score(&grid, (i, j));
            }
        }
    }
    println!("{}", &total);
}

fn score(grid: &Vec<Vec<u8>>, current_pos: (usize, usize)) -> u64 {
    let current_val = grid[current_pos.0][current_pos.1];
    if current_val == 9 {
        return 1;
    }

    let mut sum = 0;
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
                sum += score(grid, new_pos);
            }
        }
    }

    sum
}

const ORIENTATIONS: [&[isize; 2]; 4] = [&[-1, 0], &[0, 1], &[1, 0], &[0, -1]];
