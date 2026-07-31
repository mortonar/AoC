use anyhow::{Result, anyhow, bail};
use std::io::{BufRead, stdin};

fn main() -> Result<()> {
    let (card, door) = parse_input()?;

    println!("Part 1: {}", part1(card, door)?);

    Ok(())
}

fn parse_input() -> Result<(usize, usize)> {
    let mut lines = stdin().lock().lines();
    let mut take_num = || -> Result<usize> {
        Ok(lines
            .next()
            .ok_or_else(|| anyhow!("Expected number"))?
            .map_err(|_| anyhow!("Invalid number"))?
            .parse::<usize>()?)
    };
    Ok((take_num()?, take_num()?))
}

fn part1(card: usize, door: usize) -> Result<usize> {
    let (c_loop, d_loop) = (find_loop_size(card), find_loop_size(door));
    let c_key = calc_encryption_key(door, c_loop);
    let d_key = calc_encryption_key(card, d_loop);
    if c_key == d_key {
        Ok(c_key)
    } else {
        bail!("Keys don't match")
    }
}

fn find_loop_size(pub_key: usize) -> usize {
    let mut val = 1;
    let mut loop_size = 0;
    loop {
        loop_size += 1;
        val = (val * 7) % 20201227;
        if val == pub_key {
            return loop_size;
        }
    }
}

fn calc_encryption_key(subject: usize, loop_size: usize) -> usize {
    let mut val = 1;
    for _ in 0..loop_size {
        val = (val * subject) % 20201227;
    }
    val
}
