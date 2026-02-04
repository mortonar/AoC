use anyhow::{Error, Result};
use std::io::{BufRead, stdin};
use std::str::FromStr;

fn main() -> Result<()> {
    let mut firewall = parse_input()?;
    println!("Part 1: {}", firewall.trip_through_severity());

    let min_delay = (0usize..)
        .find(|delay| {
            firewall.layers.iter().all(|layer| {
                let arrival = delay + layer.depth;
                !layer.occupying_top_at(arrival)
            })
        })
        .unwrap();
    println!("Part 2: {min_delay}");

    Ok(())
}

fn parse_input() -> Result<Firewall> {
    let layers: Vec<Layer> = stdin()
        .lock()
        .lines()
        .map(|l| l?.parse())
        .collect::<Result<_>>()?;
    Ok(Firewall { layers })
}

#[derive(Debug)]
struct Firewall {
    layers: Vec<Layer>,
}

impl Firewall {
    fn trip_through_severity(&mut self) -> usize {
        let mut pos = 0;
        let mut trip_severity = 0;
        while pos <= self.layers.last().unwrap().depth {
            if let Some(severity) = self.caught_severity(pos) {
                trip_severity += severity;
            }
            pos += 1;
            self.advance(1);
        }
        trip_severity
    }

    fn advance(&mut self, steps: usize) {
        for _ in 0..steps {
            self.layers.iter_mut().for_each(Layer::advance)
        }
    }

    fn caught_severity(&self, pos: usize) -> Option<usize> {
        self.layers
            .iter()
            .find(|layer| layer.depth == pos && layer.scanner == 0)
            .map(|layer| layer.depth * layer.range)
    }
}

#[derive(Debug)]
struct Layer {
    depth: usize,
    range: usize,
    scanner: usize,
    down: bool,
}

impl FromStr for Layer {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let tokens: Vec<_> = s.split(": ").collect();
        Ok(Layer {
            depth: tokens[0].parse()?,
            range: tokens[1].parse()?,
            scanner: 0,
            down: true,
        })
    }
}

impl Layer {
    fn advance(&mut self) {
        if self.down {
            self.scanner += 1
        } else {
            self.scanner -= 1
        }

        if self.scanner == 0 || self.scanner == self.range - 1 {
            self.down = !self.down;
        }
    }

    // Given a particular range R, a scanner occupies the top slot every time P % (R - 1) == 0
    // where P is the current # of picoseconds.
    //  0   1   2   3   4   5   6
    // [ ] [S] ... ... [ ] ... [ ]
    // [ ] [ ]         [ ]     [ ]
    // [S]             [S]     [S]
    //                 [ ]     [ ]
    //                 [ ]
    //                 [ ]
    //                 [ ]
    // Range R to picoseconds P on position 0:
    // 0 = N/A
    // 2 = P % 2 == 0
    // 3 = P % 4 == 0
    // 4 = P % 6 == 0
    // 5 = P % 8 == 0
    // 6 = P % 10 == 0
    // 7 = P % 12 == 0
    fn occupying_top_at(&self, time: usize) -> bool {
        time.is_multiple_of((self.range - 1) * 2)
    }
}
