use std::{
    collections::HashMap,
    io::{BufRead, stdin},
    str::FromStr,
};

use anyhow::{Error, Result, bail};

fn main() -> Result<()> {
    let monkeys = parse_input()?;

    println!("Part 1: {}", part1(&monkeys));
    println!("Part 2: {}", part2(&monkeys));

    Ok(())
}

fn parse_input() -> Result<Monkeys> {
    let mut equations = HashMap::new();
    for line in stdin().lock().lines() {
        let line = line?;

        let Some((monkey, equation)) = line.split_once(": ") else {
            bail!("illformatted line: {line}");
        };
        equations.insert(monkey.to_string(), equation.parse()?);
    }
    Ok(Monkeys { equations })
}

fn part1(monkeys: &Monkeys) -> isize {
    monkeys.solve_all()["root"]
}

fn part2(monkeys: &Monkeys) -> isize {
    let Equation::BinOp(m1, m2, _) = &monkeys.equations["root"] else {
        panic!("root is not a binary operation");
    };

    // Only one node in the entire tree is "humn"
    if monkeys.contains_humn(m1) {
        monkeys.solve_for_humn(m1, monkeys.eval(m2))
    } else {
        monkeys.solve_for_humn(m2, monkeys.eval(m1))
    }
}

#[derive(Debug)]
struct Monkeys {
    equations: HashMap<String, Equation>,
}

#[derive(Debug)]
enum Equation {
    Value(isize),
    BinOp(String, String, Op),
}

#[derive(Debug)]
enum Op {
    Add,
    Mul,
    Sub,
    Div,
}

impl Monkeys {
    fn solve_all(&self) -> HashMap<String, isize> {
        self.equations
            .keys()
            .map(|m| (m.clone(), self.eval(m)))
            .collect()
    }

    fn eval(&self, monkey: &str) -> isize {
        match &self.equations[monkey] {
            Equation::Value(v) => *v,
            Equation::BinOp(m1, m2, op) => op.apply(self.eval(m1), self.eval(m2)),
        }
    }

    fn contains_humn(&self, monkey: &str) -> bool {
        if monkey == "humn" {
            return true;
        }
        match &self.equations[monkey] {
            Equation::Value(_) => false,
            Equation::BinOp(m1, m2, _) => self.contains_humn(m1) || self.contains_humn(m2),
        }
    }

    fn solve_for_humn(&self, monkey: &str, target: isize) -> isize {
        if monkey == "humn" {
            return target;
        }

        let Equation::BinOp(m1, m2, op) = &self.equations[monkey] else {
            panic!("humn not found in subtree of {monkey}");
        };

        if self.contains_humn(m1) {
            let sub_target = op.solve_lhs(target, self.eval(m2));
            self.solve_for_humn(m1, sub_target)
        } else {
            let sub_target = op.solve_rhs(target, self.eval(m1));
            self.solve_for_humn(m2, sub_target)
        }
    }
}

impl Op {
    fn apply(&self, a: isize, b: isize) -> isize {
        match self {
            Op::Add => a + b,
            Op::Mul => a * b,
            Op::Sub => a - b,
            Op::Div => a / b,
        }
    }

    fn solve_lhs(&self, target: isize, b: isize) -> isize {
        match self {
            Op::Add => target - b,
            Op::Mul => target / b,
            Op::Sub => target + b,
            Op::Div => target * b,
        }
    }

    fn solve_rhs(&self, target: isize, a: isize) -> isize {
        match self {
            Op::Add => target - a,
            Op::Mul => target / a,
            Op::Sub => a - target,
            Op::Div => a / target,
        }
    }
}

impl FromStr for Op {
    type Err = Error;

    fn from_str(s: &str) -> std::prelude::v1::Result<Self, Self::Err> {
        let op = match s {
            "+" => Op::Add,
            "*" => Op::Mul,
            "-" => Op::Sub,
            "/" => Op::Div,
            _ => bail!("unrecognized operator: {s}"),
        };
        Ok(op)
    }
}

impl FromStr for Equation {
    type Err = Error;

    fn from_str(s: &str) -> std::prelude::v1::Result<Self, Self::Err> {
        let tokens: Vec<_> = s.split_ascii_whitespace().collect();

        if let Ok(v) = tokens[0].parse() {
            return Ok(Equation::Value(v));
        }

        let op = tokens[1].parse()?;

        Ok(Equation::BinOp(
            tokens[0].to_string(),
            tokens[2].to_string(),
            op,
        ))
    }
}
