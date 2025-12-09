use anyhow::Result;
use std::cmp::{max, min};
use std::io::{BufRead, stdin};

fn main() -> Result<()> {
    let tiles = parse_tiles()?;
    println!("Part 1: {}", largest_rect_area(&tiles));
    println!("Part 2: {}", largest_festive_rect_area(&tiles));
    Ok(())
}

fn parse_tiles() -> Result<Vec<(usize, usize)>> {
    stdin()
        .lock()
        .lines()
        .map(|line| parse_tile(&line?))
        .collect()
}

fn parse_tile(line: &str) -> Result<(usize, usize)> {
    let tokens: Vec<_> = line.split(",").collect();
    Ok((tokens[0].parse()?, tokens[1].parse()?))
}

fn largest_rect_area(tiles: &[(usize, usize)]) -> usize {
    let mut largest = 0;
    for (i, &t1) in tiles.iter().enumerate().take(tiles.len() - 1) {
        for &t2 in tiles.iter().skip(i) {
            let area = (t1.0.abs_diff(t2.0) + 1) * (t1.1.abs_diff(t2.1) + 1);
            if area > largest {
                largest = area;
            }
        }
    }
    largest
}

fn largest_festive_rect_area(tiles: &[(usize, usize)]) -> usize {
    // Grab all consecutive tiles to get the lines / sides of the shape.
    let mut lines_in_shape: Vec<_> = Vec::new();
    for i in 0..tiles.len() {
        lines_in_shape.push((&tiles[i], &tiles[(i + 1) % tiles.len()]));
    }

    let mut largest = 0;
    for (i, &t1) in tiles.iter().enumerate().take(tiles.len() - 1) {
        for &t2 in tiles.iter().skip(i) {
            let area = (t1.0.abs_diff(t2.0) + 1) * (t1.1.abs_diff(t2.1) + 1);
            if area > largest && in_bounds(t1, t2, lines_in_shape.as_slice()) {
                largest = area;
            }
        }
    }
    largest
}

// TODO Would raycasting/point-in-polygon also work?
/// Make sure the sides of this rectangle (defined by corners) are bounded by ALL the shape's lines.
#[allow(clippy::type_complexity)]
fn in_bounds(
    corner1: (usize, usize),
    corner2: (usize, usize),
    lines: &[(&(usize, usize), &(usize, usize))],
) -> bool {
    let max_x = max(corner1.0, corner2.0);
    let min_x = min(corner1.0, corner2.0);
    let max_y = max(corner1.1, corner2.1);
    let min_y = min(corner1.1, corner2.1);
    lines.iter().all(|(line_start, line_end)| {
        // Right side of rect is left of line
        let left_of = max_x <= min(line_start.0, line_end.0);
        // Left side of rect is right of line
        let right_of = min_x >= max(line_start.0, line_end.0);
        // Bottom of rect is above line
        let above = max_y <= min(line_start.1, line_end.1);
        // Top of rect is below line
        let below = min_y >= max(line_start.1, line_end.1);
        left_of || right_of || above || below
    })
}
