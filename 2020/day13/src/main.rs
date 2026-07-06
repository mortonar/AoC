use anyhow::{Error, Result, bail};
use std::io::{BufRead, stdin};

fn main() -> Result<()> {
    let (start, bus_ids) = parse_input()?;

    let (time, bid) = earliest_bus(start, &bus_ids)?;
    println!("Part 1: {}", (time - start) * bid);
    println!("Part 2: {}", contest_timestamp(&bus_ids)?);

    Ok(())
}

fn parse_input() -> Result<(usize, Vec<(usize, usize)>)> {
    let mut lines = stdin().lock().lines();
    let start = lines.next().unwrap()?.parse()?;
    let Some(bus_ids) = lines.next() else {
        bail!("Expected bus IDs");
    };
    let bus_ids = bus_ids?
        .trim()
        .split(',')
        .enumerate()
        .filter_map(|(offset, ch)| if ch != "x" { Some((offset, ch)) } else { None })
        .map(|(offset, id)| Ok((offset, id.parse().map_err(Error::from)?)))
        .collect::<Result<Vec<_>>>()?;
    Ok((start, bus_ids))
}

fn earliest_bus(start: usize, bus_ids: &[(usize, usize)]) -> Result<(usize, usize)> {
    for time in start.. {
        if let Some(&(_offset, bid)) = bus_ids.iter().find(|&&(_offset, bid)| time % bid == 0) {
            return Ok((time, bid));
        }
    }
    bail!("No bus will work");
}

/// Some math I don't understand... from Reddit :)
fn contest_timestamp(bus_ids: &[(usize, usize)]) -> Result<i128> {
    let mut x = 0_i128;
    let mut modulus = 1_i128;

    for &(offset, id) in bus_ids {
        let m = id as i128;
        let a = (m - (offset as i128 % m)) % m;

        let rhs = (a - x).rem_euclid(m);
        let n_mod_m = modulus.rem_euclid(m);
        let inv = mod_inverse(n_mod_m, m).ok_or_else(|| {
            Error::msg(format!(
                "No inverse for {n_mod_m} mod {m}; inputs not coprime"
            ))
        })?;
        let k = (rhs * inv).rem_euclid(m);

        x += modulus * k;
        modulus *= m;
        x = x.rem_euclid(modulus);
    }

    Ok(x)
}

fn mod_inverse(a: i128, m: i128) -> Option<i128> {
    let (g, x, _y) = extended_gcd(a, m);
    if g != 1 { None } else { Some(x.rem_euclid(m)) }
}

fn extended_gcd(a: i128, b: i128) -> (i128, i128, i128) {
    if b == 0 {
        (a.abs(), a.signum(), 0)
    } else {
        let (g, x1, y1) = extended_gcd(b, a.rem_euclid(b));
        let q = a.div_euclid(b);
        (g, y1, x1 - q * y1)
    }
}
