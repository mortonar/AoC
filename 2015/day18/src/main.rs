use anyhow::Result;
use std::env;
use std::io::BufRead;

fn main() -> Result<()> {
    let steps: usize = env::args().nth(1).unwrap_or("100".to_string()).parse()?;

    let mut lights = Vec::new();
    for line in std::io::stdin().lock().lines() {
        let line = line?;
        let mut row: Vec<Light> = Vec::with_capacity(line.trim().len());
        for c in line.trim().chars() {
            row.push(c.into())
        }
        lights.push(row);
    }
    let mut lights = Lights::new(lights);

    lights.animate(steps, false);
    println!("Part 1: {}", lights.count_on());

    lights.reset();
    lights.light_corners();
    lights.animate(steps, true);
    println!("Part 2: {}", lights.count_on());

    Ok(())
}

struct Light {
    initial: bool,
    current: bool,
    next: bool,
}

impl From<char> for Light {
    fn from(value: char) -> Self {
        let on = value == '#';
        Self {
            initial: on,
            current: on,
            next: false,
        }
    }
}

struct Lights {
    lights: Vec<Vec<Light>>,
}

impl Lights {
    fn new(lights: Vec<Vec<Light>>) -> Self {
        Self { lights }
    }

    fn reset(&mut self) {
        for i in 0..self.lights.len() {
            for j in 0..self.lights[i].len() {
                self.lights[i][j].current = self.lights[i][j].initial;
            }
        }
    }

    fn light_corners(&mut self) {
        self.lights[0][0].current = true;
        self.lights[0].last_mut().unwrap().current = true;
        self.lights.last_mut().unwrap()[0].current = true;
        self.lights.last_mut().unwrap().last_mut().unwrap().current = true;
    }

    fn animate(&mut self, cycles: usize, corners_stuck: bool) {
        for _ in 0..cycles {
            self.animate_cycle(corners_stuck);
        }
    }

    fn animate_cycle(&mut self, corners_stuck: bool) {
        for i in 0..self.lights.len() {
            for j in 0..self.lights[i].len() {
                let self_on = self.lights[i][j].current;

                let mut neighbors_on = 0;
                for &[id, jd] in ORIENTATIONS {
                    let (i_n, j_n) = (i as isize + id, j as isize + jd);
                    if i_n >= 0
                        && i_n < self.lights.len() as isize
                        && j_n >= 0
                        && j_n < self.lights[i_n as usize].len() as isize
                    {
                        if self.lights[i_n as usize][j_n as usize].current {
                            neighbors_on += 1;
                        }
                    }
                }

                self.lights[i][j].next = if self_on {
                    neighbors_on == 2 || neighbors_on == 3
                } else {
                    neighbors_on == 3
                };
            }
        }

        let i_max = self.lights.len() - 1;
        let j_max = self.lights[0].len() - 1;
        for i in 0..self.lights.len() {
            for j in 0..self.lights[i].len() {
                if !corners_stuck
                    || ((i, j) != (0, 0)
                        && (i, j) != (0, j_max)
                        && (i, j) != (i_max, 0)
                        && (i, j) != (i_max, j_max))
                {
                    self.lights[i][j].current = self.lights[i][j].next;
                }
            }
        }
    }

    fn count_on(&self) -> usize {
        self.lights
            .iter()
            .flat_map(|l| l.iter())
            .filter(|l| l.current)
            .count()
    }
}

const ORIENTATIONS: [&[isize; 2]; 8] = [
    &[-1, -1],
    &[-1, 0],
    &[-1, 1],
    &[0, 1],
    &[1, 1],
    &[1, 0],
    &[1, -1],
    &[0, -1],
];
