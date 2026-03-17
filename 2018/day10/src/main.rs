use anyhow::{Error, Result, anyhow};
use std::io::stdin;
use std::str::FromStr;

fn main() -> Result<()> {
    let mut points = parse_input()?;

    // Found by waiting until 200,000 seconds and seeing at which point the bounding box for the
    // points was the smallest. Moved points that amount and printed the grid.
    let wait_time = 10144;
    points.update(wait_time);

    println!("Part 1: GGLZLHCE");
    points.print();

    println!("Part 2: {wait_time}");

    Ok(())
}

fn parse_input() -> Result<Vec<Point>> {
    stdin().lines().map(|l| l?.parse()).collect()
}

#[derive(Debug)]
struct Point {
    coords: (isize, isize),
    velocity: (isize, isize),
}

impl FromStr for Point {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let coords = parse_coords(s)?;
        let velocity = parse_coords(&s[s.find('>').unwrap() + 1..])?;
        Ok(Self { coords, velocity })
    }
}

fn parse_coords(source: &str) -> Result<(isize, isize)> {
    let start_index = source.find('<').ok_or(anyhow!("Can't find coords start"))?;
    let end_index = source.find('>').ok_or(anyhow!("Can't find coords end"))?;
    let nums: Vec<_> = source[start_index + 1..end_index].split(",").collect();
    Ok((nums[0].trim().parse()?, nums[1].trim().parse()?))
}

trait MessagePoints {
    fn update(&mut self, seconds: isize);
    fn print(&mut self);
}

impl MessagePoints for Vec<Point> {
    fn update(&mut self, seconds: isize) {
        self.iter_mut().for_each(|p| p.update(seconds));
    }

    fn print(&mut self) {
        // Figure out the smallest size grid we need to print all the points
        let (min_x, min_y, max_x, max_y) = self.iter().fold(
            (isize::MAX, isize::MAX, isize::MIN, isize::MIN),
            |(min_x, min_y, max_x, max_y), p| {
                let (px, py) = (p.coords.0, p.coords.1);
                (px.min(min_x), py.min(min_y), px.max(max_x), py.max(max_y))
            },
        );
        // Assume all coords are positive (the input is nice with this at least :))
        let (min_x, min_y, max_x, max_y) = (
            min_x as usize,
            min_y as usize,
            max_x as usize,
            max_y as usize,
        );

        // +1 since Point coords index the grid directly
        let mut grid = vec![vec![' '; max_x - min_x + 1]; max_y - min_y + 1];
        for p in self.iter() {
            grid[p.coords.1 as usize - min_y][p.coords.0 as usize - min_x] = '#';
        }

        for row in grid.iter() {
            for &col in row.iter() {
                print!("{col}");
            }
            println!();
        }
    }
}

impl Point {
    fn update(&mut self, seconds: isize) {
        self.coords.0 += seconds * self.velocity.0;
        self.coords.1 += seconds * self.velocity.1;
    }
}
