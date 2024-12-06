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
            if cross_check(&word_search, (row, col)) {
                total += 1;
            }
        }
    }
    println!("{}", total);
    Ok(())
}

fn cross_check(word_search: &Vec<Vec<char>>, start: (usize, usize)) -> bool {
    if word_search[start.0][start.1] != 'A' {
        return false;
    }

    let mut cross_words: Vec<String> = Vec::with_capacity(2);

    for cross_orient in ORIENTATIONS.chunks(2) {
        let mut word = String::with_capacity(3);
        for co in cross_orient {
            let curr_x = start.0 as isize + co[0];
            if curr_x < 0 || curr_x > ((word_search.len() - 1) as isize) {
                break;
            }

            let curr_y = start.1 as isize + co[1];
            if curr_y < 0 || curr_y > ((word_search[0].len() - 1) as isize) {
                break;
            }

            word.push(word_search[curr_x as usize][curr_y as usize]);
        }

        if word.len() == 2 {
            word.insert(1, 'A');
        }

        cross_words.push(word);
    }

    cross_words.iter().all(|s| s == "MAS" || s == "SAM")
}

// Pairwise in "cross" order: \, /
const ORIENTATIONS: [&[isize]; 4] = [&[-1, -1], &[1, 1], &[1, -1], &[-1, 1]];
