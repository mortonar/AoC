use anyhow::{Error, Result};
use std::collections::HashSet;
use std::io::{BufRead, stdin};
use std::str::FromStr;

fn main() -> Result<()> {
    let (points, folds) = parse_input()?;

    println!("Part 1: {}", part1(&points, &folds));
    part2(&points, &folds);

    Ok(())
}

fn parse_input() -> Result<(HashSet<Point>, Vec<Fold>)> {
    let mut lines = stdin().lock().lines();

    let mut points = HashSet::new();
    for line in lines.by_ref() {
        let line = line?;
        if line.is_empty() {
            break;
        }
        points.insert(line.parse()?);
    }

    let mut folds = Vec::new();
    for line in lines.by_ref() {
        folds.push(line?.parse()?);
    }

    Ok((points, folds))
}

fn part1(points: &HashSet<Point>, folds: &[Fold]) -> usize {
    let mut points = points.clone();
    points.fold(&folds[0]);
    points.len()
}

fn part2(points: &HashSet<Point>, folds: &[Fold]) {
    let mut points = points.clone();
    folds.iter().for_each(|fold| points.fold(fold));
    points.print();
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Point {
    x: usize,
    y: usize,
}

impl FromStr for Point {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let (x, y) = s
            .split_once(',')
            .ok_or_else(|| Error::msg("invalid point"))?;
        let (x, y) = (x.parse()?, y.parse()?);
        Ok(Point { x, y })
    }
}

trait FoldPaper {
    fn fold(&mut self, fold: &Fold);
}

impl FoldPaper for HashSet<Point> {
    fn fold(&mut self, fold: &Fold) {
        *self = self.iter().map(|p| fold.apply(p)).collect();
    }
}

trait Print {
    fn print(&self);
}

impl Print for HashSet<Point> {
    fn print(&self) {
        let (max_x, max_y) = self
            .iter()
            .fold((0, 0), |(max_x, max_y), p| (max_x.max(p.x), max_y.max(p.y)));
        for y in 0..=max_y {
            for x in 0..=max_x {
                if self.contains(&Point { x, y }) {
                    print!("#");
                } else {
                    print!(" ");
                }
            }
            println!();
        }
    }
}

#[derive(Debug, Clone)]
enum Fold {
    X(usize),
    Y(usize),
}

impl FromStr for Fold {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let tokens: Vec<_> = s.split(&[' ', '=']).collect();
        let value = tokens[3].parse()?;
        match tokens[2] {
            "x" => Ok(Fold::X(value)),
            "y" => Ok(Fold::Y(value)),
            _ => Err(Error::msg("invalid fold")),
        }
    }
}

impl Fold {
    fn apply(&self, point: &Point) -> Point {
        let (x, y) = match *self {
            Fold::X(v) if point.x > v => (v - (point.x - v), point.y),
            Fold::Y(v) if point.y > v => (point.x, v - (point.y - v)),
            _ => (point.x, point.y),
        };
        Point { x, y }
    }
}
