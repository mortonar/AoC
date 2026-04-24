use anyhow::Result;
use num_integer::lcm;
use std::env;
use std::io::{BufRead, stdin};
use text_io::try_scan;

fn main() -> Result<()> {
    let mut moons = parse_input()?;
    let steps = env::args().nth(1).unwrap_or("1000".to_string()).parse()?;

    println!("Part 1: {}", simulate(&mut moons.clone(), Some(steps)));
    println!("Part 2: {}", simulate(&mut moons, None));

    Ok(())
}

fn parse_input() -> Result<Vec<Moon>> {
    let mut moons = Vec::new();

    for line in stdin().lock().lines() {
        let line = line?;

        let (x, y, z): (isize, isize, isize);
        try_scan!(line.bytes() => "<x={}, y={}, z={}>", x, y, z);

        let position = Coords::new(x, y, z);
        let velocity = Coords::default();
        moons.push(Moon { position, velocity });
    }

    Ok(moons)
}

/// Simulate:
/// * The given steps returning total energy OR
/// * Until repeat of each initial axis state returning steps needed to reach initial state
fn simulate(moons: &mut [Moon], steps: Option<isize>) -> isize {
    // Each axis moves independently and each next state has exactly one previous state
    let (x_initial, y_initial, z_initial) = (moons.x_state(), moons.y_state(), moons.z_state());
    let (mut x_repeat, mut y_repeat, mut z_repeat) = (None, None, None);

    for step in 1..=steps.unwrap_or(isize::MAX) {
        let mut gravity_changes = vec![Coords::default(); moons.len()];
        for (i, m1) in moons.iter().enumerate() {
            for (j, m2) in moons.iter().enumerate() {
                if i == j {
                    continue;
                }
                gravity_changes[i].apply(&m1.gravity_change(m2));
            }
        }

        moons.iter_mut().enumerate().for_each(|(i, moon)| {
            moon.velocity.apply(&gravity_changes[i]);
            moon.apply_velocity();
        });

        if moons.x_state() == x_initial && x_repeat.is_none() {
            x_repeat = Some(step);
        }
        if moons.y_state() == y_initial && y_repeat.is_none() {
            y_repeat = Some(step);
        }
        if moons.z_state() == z_initial && z_repeat.is_none() {
            z_repeat = Some(step);
        }
        if let Some(x_repeat) = x_repeat
            && let Some(y_repeat) = y_repeat
            && let Some(z_repeat) = z_repeat
        {
            return lcm(lcm(x_repeat, y_repeat), z_repeat);
        }
    }

    if steps.is_none() {
        panic!("Couldn't find a repeat solution");
    }

    moons.iter().map(|moon| moon.total_energy()).sum()
}

#[derive(Clone, Debug)]
struct Moon {
    position: Coords,
    velocity: Coords,
}

impl Moon {
    fn gravity_change(&self, other: &Self) -> Coords {
        self.position.gravity_change(&other.position)
    }

    fn apply_velocity(&mut self) {
        self.position.apply(&self.velocity);
    }

    fn total_energy(&self) -> isize {
        self.velocity.energy() * self.position.energy()
    }
}

#[derive(Debug, Clone, Default)]
struct Coords {
    x: isize,
    y: isize,
    z: isize,
}

impl Coords {
    fn new(x: isize, y: isize, z: isize) -> Self {
        Self { x, y, z }
    }

    fn gravity_change(&self, other: &Self) -> Coords {
        Coords::new(
            -(self.x.cmp(&other.x) as isize),
            -(self.y.cmp(&other.y) as isize),
            -(self.z.cmp(&other.z) as isize),
        )
    }

    fn apply(&mut self, change: &Coords) {
        self.x += change.x;
        self.y += change.y;
        self.z += change.z;
    }

    fn energy(&self) -> isize {
        self.x.abs() + self.y.abs() + self.z.abs()
    }
}

trait AxisState {
    fn x_state(&self) -> Vec<isize>;
    fn y_state(&self) -> Vec<isize>;
    fn z_state(&self) -> Vec<isize>;
}

impl AxisState for [Moon] {
    fn x_state(&self) -> Vec<isize> {
        self.iter()
            .flat_map(|moon| [moon.position.x, moon.velocity.x])
            .collect()
    }

    fn y_state(&self) -> Vec<isize> {
        self.iter()
            .flat_map(|moon| [moon.position.y, moon.velocity.y])
            .collect()
    }

    fn z_state(&self) -> Vec<isize> {
        self.iter()
            .flat_map(|moon| [moon.position.z, moon.velocity.z])
            .collect()
    }
}
