use std::{
    collections::{HashSet, VecDeque},
    io::{BufRead, stdin},
};

use anyhow::Result;

fn main() -> Result<()> {
    let drops = parse_input()?;

    println!("Part 1: {}", part1(&drops));
    println!("Part 2: {}", part2(&drops));

    Ok(())
}

fn parse_input() -> Result<Drops> {
    let mut drops = HashSet::new();
    for line in stdin().lock().lines() {
        let line = line?;
        let tokens: Vec<_> = line.split(',').collect();
        drops.insert((tokens[0].parse()?, tokens[1].parse()?, tokens[2].parse()?));
    }
    Ok(Drops { drops })
}

fn part1(drops: &Drops) -> usize {
    drops.pocketed_surface_area()
}

fn part2(drops: &Drops) -> usize {
    drops.unpocketed_surface_area()
}

// A (1x1x1 cube) "drop" of lava or air
type Drop = (isize, isize, isize);

struct Drops {
    drops: HashSet<Drop>,
}

impl Drops {
    fn pocketed_surface_area(&self) -> usize {
        self.surface_area(&self.drops)
    }

    fn unpocketed_surface_area(&self) -> usize {
        let interior_air = self.get_interior_air();
        let exclude = interior_air.union(&self.drops).copied().collect();
        self.surface_area(&exclude)
    }

    fn surface_area(&self, exclude: &HashSet<Drop>) -> usize {
        self.drops
            .iter()
            .map(|(x, y, z)| {
                let (x, y, z) = (*x, *y, *z);
                [
                    (x - 1, y, z),
                    (x + 1, y, z),
                    (x, y - 1, z),
                    (x, y + 1, z),
                    (x, y, z - 1),
                    (x, y, z + 1),
                ]
                .iter()
                .filter(|adj| !exclude.contains(adj))
                .count()
            })
            .sum()
    }

    fn get_interior_air(&self) -> HashSet<Drop> {
        let (min_x, max_x, min_y, max_y, min_z, max_z) = self.bounding_box();

        let start = (min_x, min_y, min_z);
        let mut queue = VecDeque::from([start]);
        let mut exterior_air = HashSet::from([start]);
        while let Some((x, y, z)) = queue.pop_front() {
            for n in [
                (x - 1, y, z),
                (x + 1, y, z),
                (x, y - 1, z),
                (x, y + 1, z),
                (x, y, z - 1),
                (x, y, z + 1),
            ]
            .into_iter()
            .filter(|&(x, y, z)| {
                !self.drops.contains(&(x, y, z))
                    && x >= min_x
                    && x <= max_x
                    && y >= min_y
                    && y <= max_y
                    && z >= min_z
                    && z <= max_z
            }) {
                if exterior_air.insert(n) {
                    queue.push_back(n);
                }
            }
        }

        let mut interior_air = HashSet::new();
        for x in min_x..=max_x {
            for y in min_y..=max_y {
                for z in min_z..=max_z {
                    let a = (x, y, z);
                    if !self.drops.contains(&a) && !exterior_air.contains(&a) {
                        interior_air.insert(a);
                    }
                }
            }
        }

        interior_air
    }

    fn bounding_box(&self) -> (isize, isize, isize, isize, isize, isize) {
        let (mut min_x, mut max_x, mut min_y, mut max_y, mut min_z, mut max_z) = (
            isize::MAX,
            isize::MIN,
            isize::MAX,
            isize::MIN,
            isize::MAX,
            isize::MIN,
        );
        for &(x, y, z) in self.drops.iter() {
            (min_x, min_y, min_z) = (min_x.min(x), min_y.min(y), min_z.min(z));
            (max_x, max_y, max_z) = (max_x.max(x), max_y.max(y), max_z.max(z));
        }
        (
            min_x - 1,
            max_x + 1,
            min_y - 1,
            max_y + 1,
            min_z - 1,
            max_z + 1,
        )
    }
}
