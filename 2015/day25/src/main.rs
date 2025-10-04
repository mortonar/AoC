use regex::Regex;
use std::io;
use std::io::BufRead;

fn main() -> anyhow::Result<()> {
    let target = parse_input()?;
    let mut rc = (1, 1);
    // Save max row so we know where to wrap to
    let mut rmax = 1;
    let mut code: usize = 20151125;
    loop {
        if rc == target {
            println!("Part 1: {}", code);
            return Ok(());
        }
        rc.0 -= 1;
        rc.1 += 1;
        // Wrap around
        if rc.0 == 0 {
            rmax += 1;
            rc = (rmax, 1);
        }
        code = (code * 252533) % 33554393;
    }
}

fn parse_input() -> Result<(usize, usize), anyhow::Error> {
    let regex = Regex::new(r#"(\d+)\D+(\d+)"#)?;
    let line = io::stdin()
        .lock()
        .lines()
        .next()
        .ok_or(anyhow::anyhow!("No input line"))??;
    let caps = regex
        .captures(&line)
        .ok_or(anyhow::anyhow!("Invalid input"))?;
    let row = caps[1].parse::<usize>()?;
    let col = caps[2].parse::<usize>()?;
    Ok((row, col))
}
