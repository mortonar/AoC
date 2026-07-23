use anyhow::{Result, anyhow};
use std::io::{BufRead, stdin};

fn main() -> Result<()> {
    let (tiles, side_len) = parse_input()?;

    let (placement, corner_product) = part1(&tiles, side_len)?;
    println!("Part 1: {}", corner_product);
    println!("Part 2: {}", part2(&placement, side_len));

    Ok(())
}

fn parse_input() -> Result<(Vec<Tile>, usize)> {
    let mut tiles = Vec::new();
    let mut pixels = Vec::new();
    let mut id = 0;
    for line in stdin().lock().lines() {
        let line = line?;
        if line.trim().is_empty() {
            tiles.push(Tile::new(id, pixels));
            id = 0;
            pixels = Vec::new();
            continue;
        }

        if line.starts_with("Tile") {
            let tokens: Vec<_> = line.trim().split_ascii_whitespace().collect();
            let id_str = tokens[1];
            // Remove ':' suffix
            id = id_str[0..id_str.len() - 1].parse()?;
        } else {
            pixels.push(line.trim().chars().map(|c| c == '#').collect());
        }
    }
    tiles.push(Tile::new(id, pixels));

    let side_len = tiles.len().isqrt();

    Ok((tiles, side_len))
}

#[derive(Clone, Debug, Eq, PartialEq, Default)]
struct Tile {
    id: usize,
    pixels: Vec<Vec<bool>>,
}

fn part1(tiles: &[Tile], side_len: usize) -> Result<(Placement, usize)> {
    let placement =
        arrange(tiles, Placement::new(side_len)).ok_or_else(|| anyhow!("No placement found"))?;

    let corner_product: usize = [
        (0, 0),
        (0, side_len - 1),
        (side_len - 1, 0),
        (side_len - 1, side_len - 1),
    ]
    .iter()
    .map(|&(r, c)| placement.placed[r][c].id)
    .product();
    Ok((placement, corner_product))
}

fn part2(placement: &Placement, side_len: usize) -> usize {
    let sea_monster: Vec<Vec<_>> = [
        "..................#.",
        "#....##....##....###",
        ".#..#..#..#..#..#...",
    ]
    .iter()
    .map(|l| l.chars().map(|c| c == '#').collect())
    .collect();

    let (mut perm, monster_coords) = compose_image(placement, side_len)
        .permutations()
        .into_iter()
        .map(|perm| (perm.clone(), find_sea_monsters(&perm, &sea_monster)))
        .find(|(_perm, coords)| !coords.is_empty())
        .unwrap();

    roughness(&mut perm, monster_coords, &sea_monster)
}

fn arrange(tiles: &[Tile], current: Placement) -> Option<Placement> {
    if current.tile_idx == tiles.len() {
        return Some(current);
    }

    for next in tiles.iter() {
        if current.placed.iter().flatten().any(|t| t.id == next.id) {
            continue;
        }

        for placed in next
            .permutations()
            .iter()
            .filter_map(|perm| current.place(perm))
        {
            if let Some(found) = arrange(tiles, placed) {
                return Some(found);
            }
        }
    }

    None
}

#[derive(Clone)]
struct Placement {
    side_len: usize,
    tile_idx: usize,
    placed_idx: (usize, usize),
    placed: Vec<Vec<Tile>>,
}

impl Tile {
    fn new(id: usize, pixels: Vec<Vec<bool>>) -> Tile {
        Tile { id, pixels }
    }

    fn nth_column(&self, n: usize) -> Vec<bool> {
        self.pixels.iter().map(|row| row[n]).collect()
    }

    fn nth_row(&self, n: usize) -> Vec<bool> {
        self.pixels[n].clone()
    }

    fn permutations(&self) -> Vec<Tile> {
        self.pixels
            .permutations()
            .into_iter()
            .map(|pixels| Self {
                id: self.id,
                pixels,
            })
            .collect()
    }
}

impl Placement {
    fn new(side_len: usize) -> Self {
        Self {
            side_len,
            tile_idx: 0,
            placed_idx: (0, 0),
            placed: vec![vec![Tile::default(); side_len]; side_len],
        }
    }

    fn place(&self, tile: &Tile) -> Option<Self> {
        if !self.check_above(tile) || !self.check_left(tile) {
            return None;
        }

        let (mut r, mut c) = self.placed_idx;

        let mut placed = self.placed.clone();
        placed[r][c] = tile.clone();

        c += 1;
        if c == self.side_len {
            (r, c) = (r + 1, 0);
        }

        Some(Self {
            side_len: self.side_len,
            tile_idx: self.tile_idx + 1,
            placed_idx: (r, c),
            placed,
        })
    }

    fn check_above(&self, tile: &Tile) -> bool {
        let (r, c) = self.placed_idx;
        if r == 0 {
            return true;
        }
        let above_row = self.placed[r - 1][c].nth_row(tile.pixels.len() - 1);
        above_row == tile.nth_row(0)
    }

    fn check_left(&self, tile: &Tile) -> bool {
        let (r, c) = self.placed_idx;
        if c == 0 {
            return true;
        }
        let left_col = self.placed[r][c - 1].nth_column(tile.pixels[0].len() - 1);
        left_col == tile.nth_column(0)
    }
}

trait Permutations<T> {
    fn permutations(&self) -> Vec<Vec<Vec<T>>>;
    fn flip_y(&self) -> Vec<Vec<T>>;
    fn rotate_right(&self) -> Vec<Vec<T>>;
}

impl<T: Clone + Default> Permutations<T> for Vec<Vec<T>> {
    fn permutations(&self) -> Vec<Vec<Vec<T>>> {
        let mut perms = Vec::new();
        let mut t = self.clone();
        for _ in 0..4 {
            perms.push(t.clone());
            perms.push(t.flip_y());
            t = t.rotate_right();
        }
        perms
    }

    fn flip_y(&self) -> Self {
        self.iter()
            .map(|row| row.iter().rev().cloned().collect())
            .collect()
    }

    fn rotate_right(&self) -> Vec<Vec<T>> {
        let n = self.len();
        let mut rotated = vec![vec![T::default(); n]; n];
        for i in 0..n {
            #[allow(clippy::needless_range_loop)]
            for j in 0..n {
                rotated[j][n - 1 - i] = self[i][j].clone();
            }
        }
        rotated
    }
}

fn compose_image(placement: &Placement, side_len: usize) -> Vec<Vec<bool>> {
    let tile_pixels = &placement.placed[0][0].pixels;
    let (rows, cols) = (tile_pixels.len() - 2, tile_pixels[0].len() - 2);
    let mut composed = vec![vec![false; side_len * cols]; side_len * rows];

    for (r, row) in placement.placed.iter().enumerate() {
        for (c, tile) in row.iter().enumerate() {
            for (tr, t_row) in tile.pixels.iter().enumerate().skip(1).take(rows) {
                for (tc, &t_column) in t_row.iter().enumerate().skip(1).take(cols) {
                    composed[r * rows + tr - 1][c * cols + tc - 1] = t_column;
                }
            }
        }
    }

    composed
}

fn find_sea_monsters(composed: &[Vec<bool>], sea_monster: &[Vec<bool>]) -> Vec<(usize, usize)> {
    let mut found_monsters = Vec::new();

    for tl_r in 0..=composed.len() - sea_monster.len() {
        for tl_c in 0..=composed[0].len() - sea_monster[0].len() {
            let mut found = true;
            'outer: for (sr, s_row) in sea_monster.iter().enumerate() {
                for (sc, &sp) in s_row.iter().enumerate() {
                    if sp && !composed[tl_r + sr][tl_c + sc] {
                        found = false;
                        break 'outer;
                    }
                }
            }
            if found {
                found_monsters.push((tl_r, tl_c))
            }
        }
    }

    found_monsters
}

fn roughness(
    composed: &mut [Vec<bool>],
    monster_coords: Vec<(usize, usize)>,
    sea_monster: &[Vec<bool>],
) -> usize {
    for (mr, mc) in monster_coords {
        for (sr, s_row) in sea_monster.iter().enumerate() {
            for (sc, &sp) in s_row.iter().enumerate() {
                composed[mr + sr][mc + sc] ^= sp;
            }
        }
    }

    composed.iter().flatten().filter(|&&pixel| pixel).count()
}
