use anyhow::{Error, Result, anyhow};
use std::io::{BufRead, stdin};
use std::ops::AddAssign;
use std::str::FromStr;

fn main() -> Result<()> {
    let line_segments = parse_input()?;

    println!("Part 1: {}", part1(&line_segments));
    println!("Part 2: {}", part2(&line_segments));

    Ok(())
}

fn parse_input() -> Result<Vec<LineSegment>> {
    stdin().lock().lines().map(|l| l?.parse()).collect()
}

fn part1(line_segments: &[LineSegment]) -> usize {
    count_overlaps(line_segments, true)
}

fn part2(line_segments: &[LineSegment]) -> usize {
    count_overlaps(line_segments, false)
}

fn count_overlaps(line_segments: &[LineSegment], filter_diag: bool) -> usize {
    let line_segments: Vec<_> = if filter_diag {
        line_segments
            .iter()
            .filter(|&ls| ls.is_vertical() || ls.is_horizontal())
            .collect()
    } else {
        line_segments.iter().collect()
    };

    let (max_y, max_x) =
        line_segments
            .iter()
            .fold((isize::MIN, isize::MIN), |(max_y, max_x), seg| {
                (
                    max_y.max(seg.start.y).max(seg.end.y),
                    max_x.max(seg.start.x).max(seg.end.x),
                )
            });
    let (max_y, max_x) = (max_y as usize, max_x as usize);
    let mut grid = vec![vec![0; max_x + 1]; max_y + 1];

    for segment in line_segments.iter() {
        let mut current = segment.start;
        let diff = segment.diff();
        loop {
            grid[current.y as usize][current.x as usize] += 1;

            if current == segment.end {
                break;
            }

            current += diff;
        }
    }

    grid.iter().flatten().filter(|&&l| l > 1).count()
}

#[derive(Debug)]
struct LineSegment {
    start: Point,
    end: Point,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Point {
    x: isize,
    y: isize,
}

impl FromStr for LineSegment {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let (start, end) = s
            .trim()
            .split_once(" -> ")
            .ok_or_else(|| anyhow!("Invalid LineSegment format: {s}"))?;
        let (start, end) = (start.parse()?, end.parse()?);
        Ok(LineSegment { start, end })
    }
}

impl FromStr for Point {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let (x, y) = s
            .trim()
            .split_once(",")
            .ok_or_else(|| anyhow!("Invalid Point format: {s}"))?;
        let (x, y) = (x.parse()?, y.parse()?);
        Ok(Point { x, y })
    }
}

impl LineSegment {
    fn is_horizontal(&self) -> bool {
        self.start.y == self.end.y
    }

    fn is_vertical(&self) -> bool {
        self.start.x == self.end.x
    }

    fn diff(&self) -> Point {
        let (y, x) = (
            self.end.y.cmp(&self.start.y) as isize,
            self.end.x.cmp(&self.start.x) as isize,
        );
        Point { x, y }
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, rhs: Self) {
        self.y += rhs.y;
        self.x += rhs.x;
    }
}
