use anyhow::{Result, anyhow};
use std::io;

fn main() -> Result<()> {
    let elves: usize = io::stdin()
        .lines()
        .next()
        .ok_or_else(|| anyhow!("Number of elves must be supplied"))??
        .parse()?;

    // See this awesome Numberphile video on The Josephus Problem :)
    // https://www.youtube.com/watch?v=uCsD3ZGzMgE
    let shift_width = usize::BITS - elves.leading_zeros() - 1;
    let sig_bit = elves >> (shift_width);
    let mask = !(sig_bit << shift_width);
    let ans = ((elves & mask) << 1) | sig_bit;
    println!("Part 1: {}", ans);

    // After several solutions that were way too slow, I consulted Reddit on the pattern.
    // TODO: We'll come back to this one :)
    let mut ans = 1;
    while ans * 3 < elves {
        ans *= 3;
    }
    println!("Part 2: {}", elves - ans);

    Ok(())
}
