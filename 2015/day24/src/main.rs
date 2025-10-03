use anyhow::Result;
use itertools::Itertools;
use std::io;
use std::io::BufRead;

fn main() -> Result<()> {
    let mut packages: Vec<usize> = vec![];
    for line in io::stdin().lock().lines() {
        let line = line?;
        packages.push(line.trim().parse()?);
    }
    packages.sort();
    packages.reverse();
    println!("Part 1: {}", first_qe(&packages, 3)?);
    println!("Part 2: {}", first_qe(&packages, 4)?);

    Ok(())
}

//  I've yet to find a general solution for this. The hints and solutions
//  I've seen so far rely on the input being overly...nice. Probably because
//  this is an NP-hard problem in general.
//
//  The "nice" property being any minimally sized group 1 leaves the
//  remaining packages dividable into 2 or 3 other groups each of the
//  desired size. And the first group you find has the minimum QE. How convenient!
fn first_qe(packages: &[usize], groups: usize) -> Result<usize> {
    let target = packages.iter().sum::<usize>() / groups;
    for i in 0..packages.len() {
        for comb in packages.iter().combinations(i) {
            if comb.iter().map(|p| *p).sum::<usize>() == target {
                return Ok(comb.iter().map(|p| *p).product::<usize>())
            }
        }
    }
    Err(anyhow::Error::msg(format!("No combination found for {} groups", groups)))
}