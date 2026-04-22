use anyhow::Result;
use std::collections::{HashMap, HashSet};
use std::env;
use std::io::stdin;

fn main() -> Result<()> {
    let mut asteroid_field = parse_input()?;

    let (best_station, best_viewable) = asteroid_field.best_monitor_location();
    println!("Part 1: {best_viewable}");

    // Handle samples where station location is given rather than calculated
    let station = asteroid_field.station.unwrap_or(best_station);
    asteroid_field.station = Some(station);

    let n = env::args()
        .nth(1)
        .unwrap_or("200".to_string())
        .parse::<usize>()?;
    let nth_destroyed = asteroid_field.nth_destroyed(n);
    println!("Part 2: {}", nth_destroyed.x * 100 + nth_destroyed.y);

    Ok(())
}

fn parse_input() -> Result<AsteroidField> {
    let mut asteroids = HashSet::new();
    let mut max = Coords { x: 0, y: 0 };
    let mut station = None;
    for (y, line) in stdin().lines().enumerate() {
        for (x, c) in line?.trim().chars().enumerate() {
            let pos = Coords {
                x: x as isize,
                y: y as isize,
            };
            if c == '#' {
                asteroids.insert(pos);
            }
            if c == 'X' {
                station = Some(pos);
                asteroids.insert(pos);
            }
            max = pos;
        }
    }
    Ok(AsteroidField {
        asteroids,
        max,
        station,
    })
}

#[derive(Debug)]
struct AsteroidField {
    asteroids: HashSet<Coords>,
    max: Coords,
    station: Option<Coords>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Coords {
    x: isize,
    y: isize,
}

impl AsteroidField {
    fn best_monitor_location(&self) -> (Coords, usize) {
        let mut viewable: HashMap<Coords, HashSet<Coords>> = HashMap::new();
        for &a in &self.asteroids {
            viewable.insert(
                a,
                self.asteroids
                    .difference(&HashSet::from([a]))
                    .copied()
                    .collect(),
            );
        }

        for a1 in &self.asteroids {
            for a2 in &self.asteroids {
                if a1 == a2 {
                    continue;
                }

                let diff = Coords {
                    x: a2.x - a1.x,
                    y: a2.y - a1.y,
                };
                let gcd = gcd(diff.x, diff.y);
                let unit_dir_step = Coords {
                    x: diff.x / gcd,
                    y: diff.y / gcd,
                };

                for i in 1.. {
                    let behind = Coords {
                        x: a2.x + unit_dir_step.x * i,
                        y: a2.y + unit_dir_step.y * i,
                    };
                    if behind.x > self.max.x
                        || behind.y > self.max.y
                        || behind.x < 0
                        || behind.y < 0
                    {
                        break;
                    }
                    viewable.get_mut(a1).unwrap().remove(&behind);
                }
            }
        }

        let (coords, viewable) = viewable.iter().max_by_key(|(_coords, v)| v.len()).unwrap();
        (*coords, viewable.len())
    }

    fn nth_destroyed(&mut self, n: usize) -> Coords {
        let mut vaporized = 0;
        let station = self.station.unwrap();
        self.asteroids.remove(&station);
        loop {
            let mut viewable = self.viewable(&station);
            viewable.sort_by(|a, b| {
                clockwise_angle(station, *a)
                    .partial_cmp(&clockwise_angle(station, *b))
                    .unwrap()
            });
            for v in viewable {
                self.asteroids.remove(&v);
                vaporized += 1;
                if vaporized == n {
                    return v;
                }
            }
        }
    }

    fn viewable(&self, station: &Coords) -> Vec<Coords> {
        // Unit dir -> (closest asteroid dist^2 and coords)
        let mut nearest: HashMap<Coords, (isize, Coords)> = HashMap::new();

        for &a in &self.asteroids {
            if &a == station {
                continue;
            }

            let dx = a.x - station.x;
            let dy = a.y - station.y;
            let gcd = gcd(dx, dy);
            let unit_dir = Coords {
                x: dx / gcd,
                y: dy / gcd,
            };
            // Distance but squared
            let dist = dx * dx + dy * dy;

            match nearest.get(&unit_dir) {
                Some((best, _asteroid_coords)) if *best <= dist => {}
                _ => {
                    nearest.insert(unit_dir, (dist, a));
                }
            }
        }

        nearest.into_values().map(|(_, c)| c).collect()
    }
}

fn gcd(mut a: isize, mut b: isize) -> isize {
    while b != 0 {
        a %= b;
        std::mem::swap(&mut a, &mut b);
    }
    a.abs()
}

fn clockwise_angle(station: Coords, target: Coords) -> f64 {
    // positive = right
    let dx = (target.x - station.x) as f64;
    // positive = down
    let dy = (target.y - station.y) as f64;

    // gives clockwise from north (up)
    let angle = f64::atan2(dx, -dy);

    if angle < 0.0 {
        angle + 2.0 * std::f64::consts::PI
    } else {
        angle
    }
}
