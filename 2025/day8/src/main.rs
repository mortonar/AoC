use anyhow::{Error, Result};
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};
use std::env::args;
use std::io::{BufRead, stdin};
use std::str::FromStr;

fn main() -> Result<()> {
    let (total_boxes, mut box_heap) = parse_junction_boxes()?;
    let mut conn_to_make: isize = args().nth(1).unwrap_or("1000".to_string()).parse()?;
    let mut circuits: Vec<HashSet<JunctionBox>> = Vec::new();

    loop {
        let JunctionBoxPair { box1: b1, box2: b2 } = box_heap.pop().unwrap();
        let x_prod = b1.coords.0 * b2.coords.0;
        let c1 = circuits.iter().position(|c| c.contains(&b1));
        let c2 = circuits.iter().position(|c| c.contains(&b2));

        match (c1, c2) {
            (Some(c1), None) => {
                circuits[c1].insert(b2);
            }
            (None, Some(c2)) => {
                circuits[c2].insert(b1);
            }
            (Some(c1), Some(c2)) => {
                if c1 != c2 {
                    let c2_boxes: Vec<_> = circuits[c2].drain().collect();
                    for c in c2_boxes {
                        circuits[c1].insert(c);
                    }
                    circuits.remove(c2);
                }
            }
            (None, None) => {
                let mut new_set = HashSet::new();
                new_set.insert(b1);
                new_set.insert(b2);
                circuits.push(new_set);
            }
        }

        conn_to_make -= 1;

        let mut sizes: Vec<_> = circuits.iter().map(|c| c.len()).collect();
        sizes.sort();

        if conn_to_make == 0 {
            let ans = sizes.iter().rev().take(3).product::<usize>();
            println!("Part 1: {ans}");
        }

        if sizes.len() == 1 && sizes[0] == total_boxes {
            println!("Part 2: {x_prod}");
            break;
        }
    }

    Ok(())
}

fn parse_junction_boxes() -> Result<(usize, BinaryHeap<JunctionBoxPair>)> {
    let mut heap = BinaryHeap::new();
    let mut boxes: Vec<JunctionBox> = Vec::new();

    for line in stdin().lock().lines() {
        let new_box: JunctionBox = line?.parse()?;
        for existing_box in boxes.iter() {
            heap.push(JunctionBoxPair::new(existing_box.clone(), new_box.clone()));
        }
        boxes.push(new_box);
    }

    Ok((boxes.len(), heap))
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct JunctionBox {
    coords: (usize, usize, usize),
}

impl FromStr for JunctionBox {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let tokens: Vec<_> = s.split(",").collect();
        Ok(JunctionBox::new(
            tokens[0].parse()?,
            tokens[1].parse()?,
            tokens[2].parse()?,
        ))
    }
}

impl JunctionBox {
    fn new(x: usize, y: usize, z: usize) -> Self {
        Self { coords: (x, y, z) }
    }

    fn distance(&self, other: &Self) -> usize {
        let (x1, y1, z1) = self.coords;
        let (x2, y2, z2) = other.coords;
        let dx = x1 as f64 - x2 as f64;
        let dy = y1 as f64 - y2 as f64;
        let dz = z1 as f64 - z2 as f64;
        (dx * dx + dy * dy + dz * dz).sqrt() as usize
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct JunctionBoxPair {
    box1: JunctionBox,
    box2: JunctionBox,
}

impl JunctionBoxPair {
    fn new(box1: JunctionBox, box2: JunctionBox) -> Self {
        Self { box1, box2 }
    }

    fn distance(&self) -> usize {
        self.box1.distance(&self.box2)
    }
}

impl Ord for JunctionBoxPair {
    fn cmp(&self, other: &Self) -> Ordering {
        other.distance().cmp(&self.distance())
    }
}

impl PartialOrd<Self> for JunctionBoxPair {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
