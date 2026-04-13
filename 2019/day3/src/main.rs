use anyhow::Result;
use std::collections::{HashMap, HashSet};
use std::io::stdin;

const GRID_SIZE: isize = 50_000;
const ORIGIN: (isize, isize) = (GRID_SIZE / 2, GRID_SIZE / 2);

fn main() -> Result<()> {
    let wires = parse_input()?;

    let intersections = intersection_points(&wires);
    let closest_dist = intersections
        .iter()
        .map(|int| ORIGIN.0.abs_diff(int.0) + ORIGIN.1.abs_diff(int.1))
        .min()
        .unwrap();
    println!("Part 1: {closest_dist}");

    println!("Part 2: {}", min_signal_delay(&wires, &intersections));

    Ok(())
}

fn parse_input() -> Result<Vec<WireDirections>> {
    let mut wires = Vec::new();
    for line in stdin().lines() {
        let line = line?;
        let mut wire = Vec::new();
        for dir in line.trim().split(",") {
            wire.push((dir.chars().next().unwrap().diff(), dir[1..].parse()?));
        }
        wires.push(wire);
    }
    Ok(wires)
}

type WireDirections = Vec<((isize, isize), usize)>;

trait Diff {
    fn diff(&self) -> (isize, isize);
}

impl Diff for char {
    fn diff(&self) -> (isize, isize) {
        match self {
            'U' => (-1, 0),
            'D' => (1, 0),
            'L' => (0, -1),
            'R' => (0, 1),
            unrecognized => panic!("Unrecognized direction: {unrecognized}"),
        }
    }
}

// Plot wires and return the intersection closest to the origin
fn intersection_points(wires: &[Vec<((isize, isize), usize)>]) -> HashSet<(isize, isize)> {
    let mut intersections = HashSet::new();

    // grid[i][j] = bitmask where bit #n = wire #n
    // Allocate grid on the heap since this will break the stack size limit
    let mut grid = vec![vec![0u8; GRID_SIZE as usize]; GRID_SIZE as usize];

    for (wire_idx, wire) in wires.iter().enumerate() {
        let wire_bit = 1 << wire_idx;
        let mut current = ORIGIN;
        for &(diff, amount) in wire {
            for _ in 0..amount {
                current = (current.0 + diff.0, current.1 + diff.1);

                let cell = &mut grid[current.0 as usize][current.1 as usize];
                let was_visited_by_other = *cell & !wire_bit;
                *cell |= wire_bit;
                if was_visited_by_other != 0 {
                    intersections.insert(current);
                }
            }
        }
    }

    intersections
}

// Re-trace the wires but count their steps to intersection points
fn min_signal_delay(
    wires: &[Vec<((isize, isize), usize)>],
    intersections: &HashSet<(isize, isize)>,
) -> usize {
    // [intersection][wire #] = # of steps to first encounter with that intersection
    let mut int_step_totals: HashMap<(isize, isize), Vec<usize>> = HashMap::new();
    for int in intersections {
        int_step_totals.insert(*int, vec![0; wires.len()]);
    }

    for (wire_idx, wire) in wires.iter().enumerate() {
        let mut current = ORIGIN;
        let mut steps = 0;
        for &(diff, amount) in wire {
            for _ in 0..amount {
                current = (current.0 + diff.0, current.1 + diff.1);
                steps += 1;

                if let Some(intersect) = int_step_totals.get_mut(&current)
                    && intersect[wire_idx] == 0
                {
                    intersect[wire_idx] = steps;
                }
            }
        }
    }

    int_step_totals
        .values()
        .map(|steps| steps.iter().sum())
        .min()
        .unwrap()
}
