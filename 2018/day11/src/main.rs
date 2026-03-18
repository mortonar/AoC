use anyhow::Result;
use std::collections::HashMap;
use std::env;

const MAX: isize = 300;

fn main() -> Result<()> {
    let serial: isize = env::args().nth(1).unwrap_or("5093".to_string()).parse()?;

    // (X,Y,S) -> total power of cells in a SxS square starting at top-left corner X,Y
    let mut cache = HashMap::new();

    let (corner, _power) = largest_power(serial, 3, &mut cache);
    println!("Part 1: {},{}", corner.0, corner.1);

    let (square, (corner, _power)) = (1..=300)
        .map(|square| (square, largest_power(serial, square, &mut cache)))
        .max_by(|(_sq1, (_c1, p1)), (_sq2, (_c2, p2))| p1.cmp(p2))
        .unwrap();
    println!("Part 2: {},{},{}", corner.0, corner.1, square,);

    Ok(())
}

fn largest_power(
    serial: isize,
    square: isize,
    cache: &mut HashMap<(isize, isize, isize), isize>,
) -> ((isize, isize), isize) {
    (1..=MAX)
        .flat_map(|x| (1..=MAX).map(move |y| (x, y)))
        .filter(|&(x, y)| x < MAX - square + 2 && y < MAX - square + 2)
        .map(|(x_tl, y_tl)| {
            (
                (x_tl, y_tl),
                power_level((x_tl, y_tl), square, serial, cache),
            )
        })
        .max_by(|(_c1, p_1), (_c_2, p_2)| p_1.cmp(p_2))
        .unwrap()
}

fn power_level(
    (x_tl, y_tl): (isize, isize),
    square: isize,
    serial: isize,
    cache: &mut HashMap<(isize, isize, isize), isize>,
) -> isize {
    if let Some(cached) = cache.get(&(x_tl, y_tl, square)) {
        *cached
    } else if let Some(cached) = cache.get(&(x_tl, y_tl, square - 1)) {
        // Take "inside" square (S - 1) and add the outside sides (right column and bottom row)
        let right_col = (y_tl..=y_tl + square - 1).map(|y| (x_tl + square - 1, y));
        let bottom_row = (x_tl..=x_tl + square - 1).map(|x| (x, y_tl + square - 1));
        let new = right_col
            .chain(bottom_row)
            .map(|fc| fc.power_level(serial))
            .sum::<isize>()
            // Don't double count bottom right corner
            - (x_tl + square - 1, y_tl + square - 1).power_level(serial)
            + cached;
        cache.insert((x_tl, y_tl, square), new);
        new
    } else {
        let new = (x_tl..=x_tl + square - 1)
            .flat_map(|x| (y_tl..=y_tl + square - 1).map(move |y| (x, y)))
            .map(|fc| fc.power_level(serial))
            .sum::<isize>();
        cache.insert((x_tl, y_tl, square), new);
        new
    }
}

trait FuelCell {
    fn power_level(&self, serial: isize) -> isize;
}

impl FuelCell for (isize, isize) {
    fn power_level(&self, serial: isize) -> isize {
        let rack_id = self.0 + 10;
        let pow = (rack_id * self.1 + serial) * rack_id;
        ((pow / 100) % 10) - 5
    }
}
