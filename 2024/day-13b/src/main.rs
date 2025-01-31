use anyhow::{Context, Error, Result};
use mathru::{
    algebra::linear::{
        matrix::{General, Solve},
        vector::Vector,
    },
    matrix, vector,
};
use regex::Regex;
use std::io::{Lines, StdinLock};

// This can be cast as a matrix / linear algebra problem.
// Given:
// c = #A moves
// d = #B moves
// Solve this system of equations:
// c*AX + d*BX = X
// c*AY + d*BY = Y
// NOTE: c and d which must both be positive whole numbers
// ---------------------------------------------------------
//    Let:
//      A = [[AX, BX], [AY, BY]]
//      X = [[c], [d]]
//      B = [[X], [Y]]
// Solve for X in: AX = B
// See https://rustmath.gitlab.io/mathru/documentation/algebra/linear/matrix/#linear-system-resolution
fn main() -> Result<()> {
    let mut lines = std::io::stdin().lines();
    let parser = Parser::new()?;
    let mut total = 0;
    loop {
        let (ax, ay) = get_coords(&mut lines, &parser)?;
        let (bx, by) = get_coords(&mut lines, &parser)?;
        let (px, py) = get_coords(&mut lines, &parser)?;
        let px = px + 10000000000000.0;
        let py = py + 10000000000000.0;

        let a = matrix![ax, bx; ay, by];
        let b: Vector<f64> = vector![px; py];
        let x1: Vector<f64> = a.solve(&b).unwrap();
        let a_presses = x1.data[[0, 0]].round();
        let b_presses = x1.data[[1, 0]].round();

        // Since this results in fractional approximations, sub back into the equation to check.
        let x_new = a_presses * ax + b_presses * bx;
        let y_new = a_presses * ay + b_presses * by;
        if x_new == px && y_new == py {
            total += (a_presses * 3.0 + b_presses) as u64;
        }

        if let None = lines.next() {
            break;
        }
    }
    println!("{}", total);

    Ok(())
}

fn get_coords(lines: &mut Lines<StdinLock>, parser: &Parser) -> Result<(f64, f64)> {
    if let Some(res) = lines.next() {
        parser.parse(res?)
    } else {
        Err(Error::msg("Line expected"))
    }
}

struct Parser {
    regex: Regex,
}

impl Parser {
    fn new() -> Result<Self> {
        Ok(Parser {
            regex: Regex::new(r"\D+(\d+)\D+(\d+)$")?,
        })
    }

    fn parse<S: AsRef<str>>(&self, line: S) -> Result<(f64, f64)> {
        let (_full, [x, y]) = self
            .regex
            .captures(line.as_ref())
            .context("Regex doesn't match")?
            .extract();
        Ok((x.parse()?, y.parse()?))
    }
}
