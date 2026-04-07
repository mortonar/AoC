use anyhow::{Error, Result};
use std::io::stdin;
use std::str::FromStr;
use text_io::try_scan;
use z3::ast::Int;
use z3::{Optimize, SatResult};

fn main() -> Result<()> {
    let nanobots = parse_input()?;

    let largest_by_radius = nanobots
        .iter()
        .max_by_key(|n| n.radius)
        .ok_or(Error::msg("None with max radius"))?;
    let in_range = nanobots
        .iter()
        .filter(|n| largest_by_radius.in_range(n))
        .count();
    println!("Part 1: {in_range}");

    let part2 = solve_part2(&nanobots);
    println!("Part 2: {part2}");

    Ok(())
}

fn solve_part2(nanobots: &[NanoBot]) -> i64 {
    let opt = Optimize::new();

    let x = Int::new_const("x");
    let y = Int::new_const("y");
    let z = Int::new_const("z");
    let zero = Int::from_i64(0);
    let one = Int::from_i64(1);

    // For each nanobot, create a boolean (0 or 1) indicating whether (x,y,z) is in range
    let mut in_range_sum = Int::from_i64(0);
    for bot in nanobots {
        let bx = Int::from_i64(bot.x as i64);
        let by = Int::from_i64(bot.y as i64);
        let bz = Int::from_i64(bot.z as i64);
        let br = Int::from_i64(bot.radius as i64);

        // |x - bx| + |y - by| + |z - bz| <= radius
        let dx = z3_abs(&Int::sub(&[&x, &bx]));
        let dy = z3_abs(&Int::sub(&[&y, &by]));
        let dz = z3_abs(&Int::sub(&[&z, &bz]));
        let dist = Int::add(&[&dx, &dy, &dz]);

        let in_range = dist.le(&br).ite(&one, &zero);
        in_range_sum = Int::add(&[&in_range_sum, &in_range]);
    }

    // Distance from origin
    let dist_from_origin = Int::add(&[&z3_abs(&x), &z3_abs(&y), &z3_abs(&z)]);

    // First priority: maximize number of nanobots in range
    opt.maximize(&in_range_sum);
    // Second priority: minimize distance from origin
    opt.minimize(&dist_from_origin);

    assert_eq!(opt.check(&[]), SatResult::Sat);
    let model = opt.get_model().unwrap();

    let result = model.eval(&dist_from_origin, true).unwrap();
    result.as_i64().unwrap()
}

fn z3_abs(val: &Int) -> Int {
    let zero = Int::from_i64(0);
    val.ge(&zero).ite(val, &Int::sub(&[&zero, val]))
}

fn parse_input() -> Result<Vec<NanoBot>> {
    stdin().lines().map(|l| l?.parse()).collect()
}

#[derive(Debug, Default)]
struct NanoBot {
    x: isize,
    y: isize,
    z: isize,
    radius: usize,
}

impl FromStr for NanoBot {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let (x, y, z, radius): (_, _, _, _);
        try_scan!(s.bytes() => "pos=<{},{},{}>, r={}", x, y, z, radius);
        Ok(NanoBot { x, y, z, radius })
    }
}

impl NanoBot {
    fn in_range(&self, other: &NanoBot) -> bool {
        self.distance_to(other) <= self.radius
    }

    fn distance_to(&self, other: &NanoBot) -> usize {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y) + self.z.abs_diff(other.z)
    }
}
