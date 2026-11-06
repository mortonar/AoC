use std::io::{BufRead, Error, stdin};

use anyhow::{Result, anyhow, bail};

fn main() -> Result<()> {
    let (seeds, maps) = parse_input()?;

    println!("Part 1: {}", part1(&seeds, &maps));
    println!("Part 2: {}", part2(&seeds, &maps));

    Ok(())
}

fn parse_input() -> Result<(Vec<usize>, Vec<Map>)> {
    let mut lines = stdin().lock().lines();

    let seeds = parse_seeds(&mut lines)?;
    let _blank = lines
        .next()
        .ok_or(anyhow!("post-seed blank line missing"))??;

    let mut maps = Vec::new();
    while let Some(map) = parse_map(&mut lines)? {
        maps.push(map);
    }

    Ok((seeds, maps))
}

fn parse_seeds(lines: &mut impl Iterator<Item = Result<String, Error>>) -> Result<Vec<usize>> {
    let line = lines.next().ok_or(anyhow!("seed line missing"))??;

    let Some((_prefix, seeds)) = line.split_once("seeds: ") else {
        bail!("invalid seed line: {line}");
    };

    seeds
        .split_ascii_whitespace()
        .map(|s| Ok(s.parse()?))
        .collect::<Result<Vec<_>>>()
}

fn parse_map(lines: &mut impl Iterator<Item = Result<String, Error>>) -> Result<Option<Map>> {
    match lines.next() {
        Some(map_header) => {
            let _header = map_header?;
        }
        None => return Ok(None),
    }

    let mut ranges = Vec::new();
    for line in lines.by_ref() {
        let line = line?;
        if line.is_empty() {
            break;
        }

        let [dest, source, len] = line
            .split_ascii_whitespace()
            .map(|n| Ok(n.parse()?))
            .collect::<Result<Vec<usize>>>()?
            .try_into()
            .map_err(|v: Vec<usize>| anyhow!("expected 3 numbers, got {}", v.len()))?;

        ranges.push(Range {
            source: (source, source + len - 1),
            dest: (dest, dest + len - 1),
        })
    }

    Ok(Some(Map { ranges }))
}

#[derive(Debug)]
struct Map {
    ranges: Vec<Range>,
}

#[derive(Debug)]
struct Range {
    source: (usize, usize),
    dest: (usize, usize),
}

fn part1(seeds: &[usize], maps: &[Map]) -> usize {
    seeds
        .iter()
        .map(|&s| maps.iter().fold(s, |v, m| m.map(v)))
        .min()
        .unwrap()
}

fn part2(seeds: &[usize], maps: &[Map]) -> usize {
    let ranges: Vec<_> = seeds.chunks(2).map(|c| (c[0], c[0] + c[1])).collect();
    maps.iter()
        .fold(ranges, |rs, m| m.map_ranges(&rs))
        .into_iter()
        .map(|(lo, _hi)| lo)
        .min()
        .unwrap()
}

impl Map {
    fn map(&self, val: usize) -> usize {
        self.ranges
            .iter()
            .filter_map(|r| r.map(val))
            .next()
            .unwrap_or(val)
    }

    fn map_ranges(&self, ranges: &[(usize, usize)]) -> Vec<(usize, usize)> {
        // Ranges are now treated as half open

        let mut mapped = Vec::new();
        let mut to_map = ranges.to_vec();

        for Range { source, dest } in &self.ranges {
            let (src_lo, src_hi) = (source.0, source.1 + 1);
            let dst_lo = dest.0;

            let mut leftover = Vec::new();
            for (r_lo, r_hi) in to_map {
                // before range start
                if r_lo < src_lo.min(r_hi) {
                    leftover.push((r_lo, src_lo.min(r_hi)));
                }

                // overlap
                let (over_lo, over_hi) = (r_lo.max(src_lo), r_hi.min(src_hi));
                if over_lo < over_hi {
                    mapped.push((over_lo - src_lo + dst_lo, over_hi - src_lo + dst_lo));
                }

                // after range end
                if src_hi.max(r_lo) < r_hi {
                    leftover.push((src_hi.max(r_lo), r_hi));
                }
            }
            to_map = leftover;
        }

        mapped.extend(to_map);
        mapped
    }
}

impl Range {
    fn map(&self, val: usize) -> Option<usize> {
        (val >= self.source.0 && val <= self.source.1).then(|| self.dest.0 + val - self.source.0)
    }
}
