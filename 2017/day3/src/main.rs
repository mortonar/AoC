use anyhow::{Result, anyhow};
use std::io::{BufRead, stdin};

/// For Part 1, I started writing a parallel grid for distances of each value:
///   65 64 63  62  61  60  59 58 57
///   66 37 36  35  34  33  32 31 56                 6 5 4 3 4 5 6
///   67 38 17  16  15  14  13 30 55                 5 4 3 2 3 4 5
///   68 39 18   5   4   3  12 29 54                 4 3 2 1 2 3 4
///   69 40 19   6   1   2  11 28 53                 3 2 1 0 1 2 3
///   70 41 20   7   8   9  10 27 52                 4 3 2 1 2 3 4 ...
///   71 42 21  22  23  24  25 26 51                 5 4 3 2 3 4 5 6
///   72 43 44  45  46  47  48 49 50                 6 5 4 3 4 5 6 7
///   73 74 75  76  77  78  79 80 81
///
/// Then, for each layer, I made note of how many numbers are in the layer and what possible
/// manhattan distances you can find in the layer:
/// 0th layer = 1 number   | possible distances: 0              1
/// 1st layer = 8 numbers  | possible distances: 1, 2           2-9
/// 2nd layer = 16 numbers | possible distances: 2, 3, 4        10-25
/// 3rd layer = 24 numbers | possible distances: 3, 4, 5, 6     26-49
/// 4th layer = 32 numbers | possible distances: 4, 5, 6, 7, 8  50-81
/// ...
/// nth layer (n > 0) N = 8 * n numbers with distance ranges of [N, N * 2]
///
/// Then I noticed that the distances always start on the second to last possible distance and
/// sort of oscillate left then right through the whole layer.
///
/// The solution I then came up with was to keep track of the layer start/end and count up from the
/// first layer (just 1) until I get to a layer that contains the given number. Once there, I made
/// an iterator to do the distance oscillation until I get to the desired number
///
/// For part 2, I struggled to see any pattern and looked up some hints. After realizing it's a
/// known sequence, I didn't really feel like implementing the brute force approach (which is the
/// only one I've seen so far). I may revisit this and do so.
fn main() -> Result<()> {
    let location = parse_number()?;

    println!("Part 1: {}", part1(location)?);

    // Part 2 I looked up. This is a known sequence: https://oeis.org/A141481
    // TODO Brute force by filling in the grid.
    Ok(())
}

fn part1(location: usize) -> Result<usize> {
    if location == 0 {
        return Err(anyhow!("Location must be positive"));
    } else if location == 1 {
        return Ok(1);
    }

    let mut layer_num = 0;
    let mut layer_start = 2;
    for i in 1.. {
        let layer_end = layer_start + 8 * i;

        if (layer_start..layer_end).contains(&location) {
            layer_num = i;
            break;
        }

        layer_start = layer_end;
    }

    DistanceIterator::new(layer_num)
        .nth(location - layer_start)
        .ok_or_else(|| anyhow!("Location {location} not found"))
}

struct DistanceIterator {
    distances: Vec<usize>,
    index: usize,
    direction: isize,
}

impl DistanceIterator {
    fn new(layer_num: usize) -> Self {
        let distances: Vec<_> = (layer_num..=layer_num * 2).collect();
        let index = distances.len() - 2;
        let direction = -1;
        Self {
            distances,
            index,
            direction,
        }
    }
}

impl Iterator for DistanceIterator {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let to_return = Some(self.distances[self.index]);

        if self.index == 0 || self.index == self.distances.len() - 1 {
            self.direction *= -1;
        }
        self.index = (self.index as isize + self.direction) as usize;

        to_return
    }
}

fn parse_number() -> Result<usize> {
    stdin()
        .lock()
        .lines()
        .next()
        .ok_or_else(|| anyhow!("No number provided"))??
        .parse()
        .map_err(anyhow::Error::from)
}
