use anyhow::Result;
use std::io;

fn main() -> Result<()> {
    let mut result = 0;
    for l in io::stdin().lines() {
        let l = l?;
        let (equation, operands) = l.split_once(":").unwrap();
        let total: u64 = equation.parse()?;
        let operands: Vec<u64> = operands
            .split_ascii_whitespace()
            .map(|op| op.parse().unwrap())
            .collect();

        let (&accum, operands) = operands.split_first().unwrap();

        if calibrate(total, accum, operands) {
            result += total;
        }
    }
    println!("{}", result);

    Ok(())
}

fn calibrate(total: u64, accum: u64, operands: &[u64]) -> bool {
    if operands.is_empty() || accum > total {
        return accum == total;
    }

    let (&op, operands) = operands.split_first().unwrap();
    Operation::apply_all(accum, op)
        .iter()
        .any(|&new_acc| calibrate(total, new_acc, operands))
}

enum Operation {
    Add(u64, u64),
    Mul(u64, u64),
}

impl Operation {
    fn apply(&self) -> u64 {
        match self {
            Operation::Add(op1, op2) => op1 + op2,
            Operation::Mul(op1, op2) => op1 * op2,
        }
    }

    fn apply_all(op1: u64, op2: u64) -> Vec<u64> {
        vec![Operation::Add(op1, op2), Operation::Mul(op1, op2)]
            .iter()
            .map(Operation::apply)
            .collect()
    }
}
