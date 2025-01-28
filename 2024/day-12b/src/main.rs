use std::collections::HashSet;
use std::io::stdin;

fn main() {
    let mut grid: Vec<Vec<char>> = stdin()
        .lines()
        .flatten()
        .map(|l| l.chars().collect())
        .collect();

    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut regions = Vec::new();
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            let start = (i, j);
            if !visited.contains(&start) {
                regions.push(define_region(start, &mut grid, &mut visited));
            }
        }
    }
    let total: u64 = regions.iter().map(|r| r.price()).sum();
    println!("{}", total);
}

#[derive(Debug)]
struct Plant {
    label: char,
    perimeter: u8,
    loc: (usize, usize),
}

impl Plant {
    fn new(label: char, loc: (usize, usize)) -> Plant {
        Plant {
            label,
            perimeter: 4,
            loc,
        }
    }
}

#[derive(Debug, Default)]
struct Region {
    plants: Vec<Plant>,
}

impl Region {
    fn add_plant(&mut self, plant: Plant) {
        self.plants.push(plant);
    }

    fn get_plant(&self, loc: (isize, isize)) -> Option<&Plant> {
        if loc.0 < 0 || loc.1 < 0 {
            return None;
        }
        let loc = (loc.0 as usize, loc.1 as usize);
        self.plants.iter().find(|plant| plant.loc == loc)
    }

    // A very gross and long inside & outside corner check noting that # corners == # sides
    fn price(&self) -> u64 {
        let mut num_sides = 0;
        for (x, y) in self.plants.iter().map(|plant| plant.loc) {
            let (x, y) = (x as isize, y as isize);
            let mut corners = 0;

            // Outside Top Left
            // CC
            // C
            if self.get_plant((x, y - 1)).is_none() && self.get_plant((x - 1, y)).is_none() {
                corners += 1;
            }

            // Outside Top Right
            // CC
            //  C
            if self.get_plant((x - 1, y)).is_none() && self.get_plant((x, y + 1)).is_none() {
                corners += 1;
            }

            // Outside Bottom Left
            // C
            // CC
            if self.get_plant((x, y - 1)).is_none() && self.get_plant((x + 1, y)).is_none() {
                corners += 1;
            }

            // Outside Bottom Right
            //  C
            // CC
            if self.get_plant((x, y + 1)).is_none() && self.get_plant((x + 1, y)).is_none() {
                corners += 1;
            }

            // Inside Bottom Right
            //  C
            // CC
            if self.get_plant((x - 1, y)).is_some()
                && self.get_plant((x - 1, y - 1)).is_none()
                && self.get_plant((x, y - 1)).is_some()
            {
                corners += 1;
            }

            // Inside Top Right
            // CC
            //  C
            if self.get_plant((x, y - 1)).is_some()
                && self.get_plant((x + 1, y - 1)).is_none()
                && self.get_plant((x + 1, y)).is_some()
            {
                corners += 1;
            }

            // Inside Top Left
            // CC
            // C
            if self.get_plant((x, y + 1)).is_some()
                && self.get_plant((x + 1, y + 1)).is_none()
                && self.get_plant((x + 1, y)).is_some()
            {
                corners += 1;
            }

            // Inside Bottom Left
            // C
            // CC
            if self.get_plant((x - 1, y)).is_some()
                && self.get_plant((x - 1, y + 1)).is_none()
                && self.get_plant((x, y + 1)).is_some()
            {
                corners += 1;
            }

            num_sides += corners;
        }
        num_sides * self.plants.len() as u64
    }
}

fn define_region(
    start: (usize, usize),
    grid: &mut Vec<Vec<char>>,
    visited: &mut HashSet<(usize, usize)>,
) -> Region {
    let mut region = Region::default();

    let mut queue = Vec::new();
    let plant_start = Plant::new(grid[start.0][start.1], start);
    queue.push(plant_start);
    visited.insert(start);

    while let Some(mut plant) = queue.pop() {
        for &[x_offset, y_offset] in ORIENTATIONS {
            let new_pos = (
                plant.loc.0 as isize + x_offset,
                plant.loc.1 as isize + y_offset,
            );
            if new_pos.0 >= 0
                && new_pos.0 < grid.len() as isize
                && new_pos.1 >= 0
                && new_pos.1 < grid[0].len() as isize
            {
                let new_pos = (new_pos.0 as usize, new_pos.1 as usize);
                let label = grid[new_pos.0][new_pos.1];
                if label == plant.label {
                    plant.perimeter -= 1;
                    if !visited.contains(&new_pos) {
                        queue.push(Plant::new(label, new_pos));
                        visited.insert(new_pos);
                    }
                }
            }
        }
        region.add_plant(plant);
    }

    region
}

const ORIENTATIONS: [&[isize; 2]; 4] = [&[0, 1], &[1, 0], &[0, -1], &[-1, 0]];
