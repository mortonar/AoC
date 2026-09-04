use anyhow::{Error, Result, anyhow};
use std::collections::{HashMap, HashSet};
use std::io::{BufRead, stdin};

fn main() -> Result<()> {
    let scanners = parse_input()?;
    let resolved = resolve(&scanners);

    println!("Part 1: {}", part1(&resolved));
    println!("Part 2: {}", part2(&resolved));

    Ok(())
}

fn resolve(scanners: &[Scanner]) -> Vec<(Point, Vec<Point>)> {
    let origin = Point { x: 0, y: 0, z: 0 };
    let mut resolved = vec![(origin, scanners[0].beacons.clone())];
    let mut pending = scanners[1..].iter().collect::<Vec<_>>();
    let mut reference = 0;

    while reference < resolved.len() && !pending.is_empty() {
        let aligned = resolved[reference].1.clone();
        let mut unmatched = Vec::new();

        for scanner in pending {
            match align(&aligned, &scanner.beacons) {
                Some(found) => resolved.push(found),
                None => unmatched.push(scanner),
            }
        }

        pending = unmatched;
        reference += 1;
    }

    resolved
}

fn part1(resolved: &[(Point, Vec<Point>)]) -> usize {
    resolved
        .iter()
        .flat_map(|(_, beacons)| beacons)
        .collect::<HashSet<_>>()
        .len()
}

fn part2(resolved: &[(Point, Vec<Point>)]) -> isize {
    let mut max = 0;
    for (scanner_a, _) in resolved {
        for (scanner_b, _) in resolved {
            max = max.max(
                (scanner_a.x - scanner_b.x).abs()
                    + (scanner_a.y - scanner_b.y).abs()
                    + (scanner_a.z - scanner_b.z).abs(),
            );
        }
    }
    max
}

fn parse_input() -> Result<Vec<Scanner>> {
    let mut scanners = Vec::new();
    let mut beacons = Vec::new();
    for line in stdin().lock().lines() {
        let line = line?;

        if line.contains("scanner") {
            continue;
        }

        if line.is_empty() {
            scanners.push(Scanner { beacons });
            beacons = Vec::new();
            continue;
        }

        beacons.push(line.parse()?);
    }

    scanners.push(Scanner { beacons });

    Ok(scanners)
}

#[derive(Debug, Clone)]
struct Scanner {
    beacons: Vec<Point>,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Point {
    x: isize,
    y: isize,
    z: isize,
}

const ORIENTATIONS: usize = 24;
const OVERLAP: usize = 12;

impl Point {
    fn orient(self, i: usize) -> Point {
        let Point { x, y, z } = self;
        let (x, y, z) = match i {
            0 => (x, y, z),
            1 => (x, -y, -z),
            2 => (-x, y, -z),
            3 => (-x, -y, z),
            4 => (y, z, x),
            5 => (y, -z, -x),
            6 => (-y, z, -x),
            7 => (-y, -z, x),
            8 => (z, x, y),
            9 => (z, -x, -y),
            10 => (-z, x, -y),
            11 => (-z, -x, y),
            12 => (-y, x, z),
            13 => (y, -x, z),
            14 => (y, x, -z),
            15 => (-y, -x, -z),
            16 => (-x, z, y),
            17 => (x, -z, y),
            18 => (x, z, -y),
            19 => (-x, -z, -y),
            20 => (-z, y, x),
            21 => (z, -y, x),
            22 => (z, y, -x),
            23 => (-z, -y, -x),
            _ => unreachable!("orientation index out of range: {i}"),
        };
        Point { x, y, z }
    }
}

impl std::ops::Add for Point {
    type Output = Point;

    fn add(self, rhs: Point) -> Point {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl std::ops::Sub for Point {
    type Output = Point;

    fn sub(self, rhs: Point) -> Point {
        Point {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl std::str::FromStr for Point {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let parts = s
            .split(',')
            .map(|p| p.parse().map_err(Error::from))
            .collect::<Result<Vec<_>>>()?;
        if parts.len() != 3 {
            Err(anyhow!("Invalid point format: {s}"))
        } else {
            let (x, y, z) = (parts[0], parts[1], parts[2]);
            Ok(Point { x, y, z })
        }
    }
}

fn align(fixed: &[Point], candidate: &[Point]) -> Option<(Point, Vec<Point>)> {
    for orient in 0..ORIENTATIONS {
        let rotated = candidate
            .iter()
            .map(|c| c.orient(orient))
            .collect::<Vec<_>>();
        let mut offsets: HashMap<Point, usize> = HashMap::new();
        for f in fixed.iter() {
            for c in rotated.iter() {
                let offset = *f - *c;
                let count = offsets.entry(offset).or_insert(0);
                *count += 1;
                if *count == OVERLAP {
                    let aligned = rotated.iter().map(|c| *c + offset).collect();
                    return Some((offset, aligned));
                }
            }
        }
    }

    None
}
