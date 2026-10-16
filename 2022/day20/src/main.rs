use std::{
    collections::VecDeque,
    io::{BufRead, stdin},
};

use anyhow::{Error, Result};

fn main() -> Result<()> {
    let numbers = parse_input()?;

    println!("Part 1: {}", part1(&numbers));
    println!("Part 2: {}", part2(&numbers));

    Ok(())
}

fn parse_input() -> Result<Vec<isize>> {
    stdin()
        .lock()
        .lines()
        .map(|l| l?.parse().map_err(Error::from))
        .collect()
}

fn part1(numbers: &[isize]) -> isize {
    mix(numbers, 1, 1)
}

fn part2(numbers: &[isize]) -> isize {
    mix(numbers, 811589153, 10)
}

fn mix(numbers: &[isize], key: isize, rounds: usize) -> isize {
    let scaled: Vec<isize> = numbers.iter().map(|&n| n * key).collect();
    let mut list: VecDeque<(usize, isize)> = scaled.iter().copied().enumerate().collect();

    for _ in 0..rounds {
        for orig_pos in 0..scaled.len() {
            move_element(&mut list, orig_pos);
        }
    }

    grove_coordinates(&list)
}

fn move_element(list: &mut VecDeque<(usize, isize)>, orig_pos: usize) {
    while list.back().unwrap().0 != orig_pos {
        let front = list.pop_front().unwrap();
        list.push_back(front);
    }
    let (pos, value) = list.pop_back().unwrap();

    // Moving `value` steps forward is equivalent to moving `value.rem_euclid(modulus)`
    // steps forward, since only the remaining elements (after removing this one)
    // define the cycle it rotates through. This also naturally handles negative
    // `value` (a leftward move), since rem_euclid folds it into the equivalent
    // rightward move.
    let modulus = list.len() as isize;
    let shift = value.rem_euclid(modulus);
    for _ in 0..shift {
        let front = list.pop_front().unwrap();
        list.push_back(front);
    }
    list.push_back((pos, value));
}

fn grove_coordinates(list: &VecDeque<(usize, isize)>) -> isize {
    let len = list.len();
    let zero_pos = list.iter().position(|&(_, v)| v == 0).unwrap();
    [1000, 2000, 3000]
        .iter()
        .map(|offset| list[(zero_pos + offset) % len].1)
        .sum()
}
