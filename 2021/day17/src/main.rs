use anyhow::{Error, Result};
use std::cmp::Ordering;
use std::io::{BufRead, stdin};
use std::str::FromStr;

fn main() -> Result<()> {
    let target_area = parse_input()?;

    let (success_velocities, max_y) = experiment(&target_area);
    println!("Part 1: {}", max_y);
    println!("Part 2: {}", success_velocities);

    Ok(())
}

fn parse_input() -> Result<Area> {
    let mut line = String::new();
    stdin().lock().read_line(&mut line)?;
    line.parse()
}

fn experiment(target_area: &Area) -> (usize, isize) {
    let mut total_max_y = isize::MIN;
    let mut success_velocities = 0;

    // Check a fixed plausible range of velocities to check since target area is always down-right.
    for vx in 1..250 {
        for vy in -250..250 {
            let mut probe = Probe::new((vx, vy));
            let mut max_y = isize::MIN;

            loop {
                probe.step();
                max_y = max_y.max(probe.y);

                match target_area.eval(&probe) {
                    TargetResult::Before => {}
                    TargetResult::Inside => {
                        total_max_y = total_max_y.max(max_y);
                        success_velocities += 1;
                        break;
                    }
                    TargetResult::After => break,
                }
            }
        }
    }

    (success_velocities, total_max_y)
}

#[derive(Debug)]
struct Area {
    x: (isize, isize),
    y: (isize, isize),
}

#[derive(Debug)]
struct Probe {
    x: isize,
    y: isize,
    vx: isize,
    vy: isize,
}

impl Probe {
    fn new((vx, vy): (isize, isize)) -> Self {
        let (x, y) = (0, 0);
        Self { x, y, vx, vy }
    }

    fn step(&mut self) {
        self.x += self.vx;
        self.y += self.vy;

        match self.vx.cmp(&0) {
            Ordering::Less => self.vx += 1,
            Ordering::Equal => {}
            Ordering::Greater => self.vx -= 1,
        }

        self.vy -= 1;
    }
}

impl Area {
    fn eval(&self, probe: &Probe) -> TargetResult {
        let x_eval = match probe.x.cmp(&self.x.0) {
            Ordering::Less => TargetResult::Before,
            Ordering::Equal | Ordering::Greater if probe.x <= self.x.1 => TargetResult::Inside,
            _ => TargetResult::After,
        };

        match probe.y.cmp(&self.y.0) {
            Ordering::Greater | Ordering::Equal if probe.y <= self.y.1 => x_eval,
            Ordering::Greater => {
                if x_eval != TargetResult::Inside {
                    x_eval
                } else {
                    TargetResult::Before
                }
            }
            _ => TargetResult::After,
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
enum TargetResult {
    Before,
    Inside,
    After,
}

impl FromStr for Area {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let tokens: Vec<_> = s.trim().split(&[' ', ',', '.', '=']).collect();
        let x = (tokens[3].parse()?, tokens[5].parse()?);
        let y = (tokens[8].parse()?, tokens[10].parse()?);
        Ok(Area { x, y })
    }
}
