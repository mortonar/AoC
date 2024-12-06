use anyhow::Result;
use std::io;

fn main() -> Result<()> {
    let word_search: Vec<Vec<char>> = io::stdin()
        .lines()
        .flatten()
        .map(|l| l.chars().collect())
        .collect();

    let mut total = 0;
    for (row, line) in word_search.iter().enumerate() {
        for (col, _letter) in line.iter().enumerate() {
            total += search(&word_search, (row, col));
        }
    }
    println!("{}", total);
    Ok(())
}

fn search(word_search: &Vec<Vec<char>>, start: (usize, usize)) -> usize {
    let mut found = 0;
    if word_search[start.0][start.1] != 'X' {
        return found;
    }

    for orient in ORIENTATIONS {
        let mut word = String::with_capacity(4);
        word.push('X');

        let mut curr_x = start.0 as isize;
        let mut curr_y = start.1 as isize;
        for _ in 0..3 {
            curr_x += orient[0];
            if curr_x < 0 || curr_x > ((word_search.len() - 1) as isize) {
                break;
            }
            curr_y += orient[1];
            if curr_y < 0 || curr_y > ((word_search[0].len() - 1) as isize) {
                break;
            }
            word.push(word_search[curr_x as usize][curr_y as usize]);
        }
        if word == "XMAS" {
            found += 1;
        }
    }
    found
}

const ORIENTATIONS: [&[isize]; 8] = [
    &[-1, -1],
    &[-1, 0],
    &[-1, 1],
    &[0, 1],
    &[1, 1],
    &[1, 0],
    &[1, -1],
    &[0, -1],
];
