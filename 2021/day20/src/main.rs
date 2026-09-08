use anyhow::Result;
use std::io::{BufRead, stdin};

fn main() -> Result<()> {
    let (enhance, image) = parse_input()?;

    println!("Part 1: {}", part1(&enhance, &image));
    println!("Part 2: {}", part2(&enhance, &image));

    Ok(())
}

fn parse_input() -> Result<(Vec<bool>, Image)> {
    let mut lines = stdin().lock().lines();

    let enhance = lines
        .next()
        .ok_or_else(|| anyhow::anyhow!("Missing enhance line"))??
        .chars()
        .map(|c| c == '#')
        .collect::<Vec<_>>();

    let _blank = lines
        .next()
        .ok_or_else(|| anyhow::anyhow!("Missing blank line"))??;

    let mut pixels = Vec::new();
    for line in lines {
        let row = line?.chars().map(|c| c == '#').collect::<Vec<_>>();
        pixels.push(row);
    }

    Ok((
        enhance,
        Image {
            pixels,
            background: false,
        },
    ))
}

fn part1(enhance: &[bool], image: &Image) -> usize {
    let mut image = image.clone();
    (0..2).for_each(|_| image.enhance(enhance));
    image.lit()
}

fn part2(enhance: &[bool], image: &Image) -> usize {
    let mut image = image.clone();
    (0..50).for_each(|_| image.enhance(enhance));
    image.lit()
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Image {
    pixels: Vec<Vec<bool>>,
    background: bool,
}

impl Image {
    fn enhance(&mut self, enhance: &[bool]) {
        let mut new_pixels = vec![vec![false; self.pixels[0].len() + 2]; self.pixels.len() + 2];
        for (i, row) in new_pixels.iter_mut().enumerate() {
            for (j, pixel) in row.iter_mut().enumerate() {
                *pixel = enhance[self.get_enhance_index((i as isize - 1, j as isize - 1))];
            }
        }

        self.pixels = new_pixels;
        self.background = if self.background {
            enhance[enhance.len() - 1]
        } else {
            enhance[0]
        };
    }

    fn get_enhance_index(&self, (i, j): (isize, isize)) -> usize {
        let mut index = 0;
        for row in i - 1..=i + 1 {
            for col in j - 1..=j + 1 {
                index = (index << 1) | self.pixel_index((row, col));
            }
        }
        index
    }

    fn pixel_index(&self, (i, j): (isize, isize)) -> usize {
        if i < 0
            || i >= self.pixels.len() as isize
            || j < 0
            || j >= self.pixels[i as usize].len() as isize
        {
            self.background as usize
        } else {
            self.pixels[i as usize][j as usize] as usize
        }
    }

    fn lit(&self) -> usize {
        self.pixels
            .iter()
            .flat_map(|row| row.iter())
            .filter(|&&pixel| pixel)
            .count()
    }
}
