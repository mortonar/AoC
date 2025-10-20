use anyhow::{Result, anyhow};
use std::io;
use std::io::BufRead;

fn main() -> Result<()> {
    let mut args = std::env::args().skip(1);
    let width_limit = args.next().unwrap_or("50".to_owned()).parse::<usize>()?;
    let height_limit = args.next().unwrap_or("6".to_owned()).parse::<usize>()?;

    let mut screen = Screen::new(width_limit, height_limit);

    for line in io::stdin().lock().lines() {
        let operation = line?;
        let tokens: Vec<_> = operation.split_whitespace().collect();
        match tokens.as_slice() {
            ["rect", dims] => {
                let (w, h) = dims
                    .split_once('x')
                    .ok_or_else(|| anyhow!("Invalid rect dims: {}", dims))?;
                screen.rect(h.parse()?, w.parse()?);
            }
            ["rotate", kind, rc, "by", shifts] => {
                let idx = rc
                    .split_once('=')
                    .ok_or_else(|| anyhow!("Invalid rotate: {}", rc))?
                    .1
                    .parse::<usize>()?;
                let shifts = shifts.parse::<usize>()?;
                match *kind {
                    "row" => screen.rotate_row(idx, shifts),
                    "column" => screen.rotate_column(idx, shifts),
                    _ => return Err(anyhow!("Unknown rotate kind: {}", kind)),
                }
            }
            _ => return Err(anyhow!("Unknown operation: {}", operation)),
        }
    }

    println!("Part 1: {}", screen.lit_count());
    println!("Part 2:");
    screen.print();

    Ok(())
}

struct Screen {
    pixels: Vec<Vec<bool>>,
}

impl Screen {
    fn new(width_limit: usize, height_limit: usize) -> Self {
        Self {
            pixels: vec![vec![false; width_limit]; height_limit],
        }
    }

    fn rect(&mut self, height: usize, width: usize) {
        for i in 0..height {
            for j in 0..width {
                self.pixels[i][j] = true;
            }
        }
    }

    fn rotate_row(&mut self, row_num: usize, shifts: usize) {
        let row = self.pixels[row_num].clone();
        let width = row.len();
        for (col, val) in row.into_iter().enumerate() {
            self.pixels[row_num][(col + shifts) % width] = val;
        }
    }

    fn rotate_column(&mut self, col_num: usize, shifts: usize) {
        let height = self.pixels.len();
        let mut column = vec![false; height];
        for (row, val) in column.iter_mut().enumerate() {
            *val = self.pixels[row][col_num];
        }
        for (row, val) in column.into_iter().enumerate() {
            self.pixels[(row + shifts) % height][col_num] = val;
        }
    }

    fn print(&self) {
        for row in 0..self.pixels.len() {
            for col in 0..self.pixels[row].len() {
                let char = if self.pixels[row][col] { '#' } else { '.' };
                print!("{}", char);
            }
            println!();
        }
    }

    fn lit_count(&self) -> usize {
        self.pixels.iter().flatten().filter(|&&c| c).count()
    }
}
