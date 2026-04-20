use anyhow::{Context, Result, anyhow};
use std::env;

fn main() -> Result<()> {
    let pixels = parse_input()?;
    let [width, height] = parse_dimensions()?;

    let (mut min_zeros, mut prod_ones_twos) = (usize::MAX, 0);

    let mut image = vec![vec![Color::Transparent; width]; height];

    for layer in pixels.chunks(width * height) {
        let mut counts = [0; 10];
        layer.iter().for_each(|&p| counts[p] += 1);
        if counts[0] < min_zeros {
            min_zeros = counts[0];
            prod_ones_twos = counts[1] * counts[2];
        }

        for i in 0..height {
            for j in 0..width {
                if image[i][j] == Color::Transparent {
                    image[i][j] = Color::from(layer[i * width + j]);
                }
            }
        }
    }

    println!("Part 1: {}", prod_ones_twos);

    println!("Part 2:");
    for row in image.iter() {
        for pixel in row.iter() {
            let c = match pixel {
                Color::White => "#",
                _ => " ",
            };
            print!("{c}");
        }
        println!();
    }

    Ok(())
}

fn parse_input() -> Result<Vec<usize>> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line)?;

    let mut pixels = Vec::with_capacity(line.trim().len());
    for c in line.trim().chars() {
        let digit = c
            .to_digit(10)
            .ok_or_else(|| anyhow!("Input contains non-digit character: {c:?}"))?;
        pixels.push(digit as usize);
    }

    Ok(pixels)
}

fn parse_dimensions() -> Result<[usize; 2]> {
    let dimensions = env::args()
        .skip(1)
        .take(2)
        .map(|d| {
            d.parse::<usize>()
                .context("image dimensions must be positive integers")
        })
        .collect::<Result<Vec<_>>>()?;

    match dimensions.as_slice() {
        [width, height] => Ok([*width, *height]),
        d => anyhow::bail!(
            "Expected exactly 2 dimensions, got {}: {:?}",
            dimensions.len(),
            d
        ),
    }
}

#[repr(usize)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Color {
    Black = 0,
    White = 1,
    Transparent = 2,
}

impl From<usize> for Color {
    fn from(value: usize) -> Self {
        match value {
            0 => Color::Black,
            1 => Color::White,
            2 => Color::Transparent,
            v => panic!("Invalid color value: {v}"),
        }
    }
}
