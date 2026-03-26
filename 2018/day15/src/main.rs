use anyhow::{Error, Result, bail};
use std::cmp::PartialEq;
use std::collections::{HashSet, VecDeque};
use std::hash::Hash;
use std::io::stdin;

fn main() -> Result<()> {
    let original = parse_input()?;

    let mut cave = original.clone();
    let outcome = cave.combat(false)?;
    println!("Part 1: {outcome} ({})", cave.combat_rounds);

    for ap in 4.. {
        let mut cave = original.clone();
        cave.buff_elves(ap);
        if let Ok(outcome) = cave.combat(true) {
            println!("Part 2: {outcome} ({})", cave.combat_rounds);
            break;
        }
    }

    Ok(())
}

fn parse_input() -> Result<Cave> {
    let mut tiles = Vec::new();
    let mut units = Vec::new();

    for (row, line) in stdin().lines().enumerate() {
        let line = line?;
        let mut tile_row = Vec::new();
        for (column, c) in line.trim().chars().enumerate() {
            match c {
                '.' | '#' => tile_row.push(c),
                'G' | 'E' => {
                    tile_row.push('.');
                    units.push(Unit::new(row, column, c.try_into()?))
                }
                _ => bail!("Unrecognized tile char: '{c}'"),
            }
        }
        tiles.push(tile_row);
    }

    Ok(Cave {
        tiles,
        units,
        combat_rounds: 0,
    })
}

#[derive(Clone, Debug)]
struct Cave {
    tiles: Vec<Vec<char>>,
    units: Vec<Unit>,
    combat_rounds: usize,
}

impl Cave {
    fn combat(&mut self, no_elf_losses: bool) -> Result<usize> {
        'outer: loop {
            if no_elf_losses && self.lost_elf() {
                bail!("Lost an elf")
            }

            self.units.retain(|u| !u.is_dead());

            let types: HashSet<_> = self.units.iter().map(|u| u.u_type).collect();
            if types.len() == 1 {
                break;
            }

            self.units
                .sort_by(|u1, u2| u1.row.cmp(&u2.row).then_with(|| u1.column.cmp(&u2.column)));

            for i in 0..self.units.len() {
                if self.units[i].is_dead() {
                    continue;
                }

                let targets = self.find_all_alive_targets(i);
                if targets.is_empty() {
                    break 'outer;
                }

                if !self.maybe_unit_attack(i) {
                    self.maybe_unit_move(i, &targets);
                    self.maybe_unit_attack(i);
                }
            }

            self.combat_rounds += 1;
        }

        if no_elf_losses && self.lost_elf() {
            bail!("Lost an elf")
        }

        Ok(self.combat_rounds
            * self
                .units
                .iter()
                .filter(|u| !u.is_dead())
                .map(|u| u.hp as usize)
                .sum::<usize>())
    }

    fn find_all_alive_targets(&self, unit_idx: usize) -> Vec<usize> {
        (0..self.units.len())
            .filter(|&j| {
                j != unit_idx
                    && !self.units[j].is_dead()
                    && self.units[unit_idx].u_type != self.units[j].u_type
            })
            .collect()
    }

    fn maybe_unit_move(&mut self, unit_idx: usize, targets: &[usize]) {
        let possible_dests: HashSet<_> = targets
            .iter()
            .flat_map(|&t| self.units[t].adjacent_spaces())
            .filter(|&(adj_r, adj_c)| self.is_open((adj_r, adj_c)))
            .collect();

        if possible_dests.is_empty() {
            return;
        }

        let start = self.unit_coords(unit_idx);
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        // (current position, first step from start, distance)
        for adj in start
            .adjacent_spaces()
            .into_iter()
            .filter(|s| self.is_open(*s))
        {
            queue.push_back((adj, adj, 1));
            visited.insert(adj);
        }
        let mut found_distance = None;
        let mut found_steps = Vec::new();
        while let Some((pos, first_step, dist)) = queue.pop_front() {
            if let Some(fd) = found_distance
                && dist > fd
            {
                break;
            }
            if possible_dests.contains(&pos) {
                found_distance = Some(dist);
                found_steps.push((pos, first_step));
                continue;
            }
            for next in pos
                .adjacent_spaces()
                .into_iter()
                .filter(|s| self.is_open(*s))
            {
                if visited.insert(next) {
                    queue.push_back((next, first_step, dist + 1));
                }
            }
        }
        if found_steps.is_empty() {
            return;
        }
        // Sort by destination reading order, then pick the first step in reading order
        found_steps.sort_by(|a, b| a.0.cmp(&b.0));
        let next_step = found_steps[0].1;
        let unit = &mut self.units[unit_idx];
        unit.row = next_step.0;
        unit.column = next_step.1;
    }

    fn maybe_unit_attack(&mut self, unit_idx: usize) -> bool {
        let adjacent_spaces = self.units[unit_idx].adjacent_spaces();
        let mut adjacent_targets: Vec<_> = self
            .find_all_alive_targets(unit_idx)
            .iter()
            .filter_map(|&t| {
                let t_coords = (self.units[t].row, self.units[t].column);
                if adjacent_spaces.contains(&t_coords) {
                    Some(t)
                } else {
                    None
                }
            })
            .collect();

        if !adjacent_targets.is_empty() {
            adjacent_targets.sort_by(|&t1, &t2| {
                let (t1, t2) = (&self.units[t1], &self.units[t2]);
                t1.hp
                    .cmp(&t2.hp)
                    .then_with(|| t1.row.cmp(&t2.row))
                    .then_with(|| t1.column.cmp(&t2.column))
            });
            let damage = self.units[unit_idx].ap;
            self.units[adjacent_targets[0]].hp -= damage;
            true
        } else {
            false
        }
    }

    fn is_open(&self, (row, column): (usize, usize)) -> bool {
        self.tiles[row][column] == '.'
            && self
                .units
                .iter()
                .filter(|u| !u.is_dead())
                .all(|u| (u.row, u.column) != (row, column))
    }

    fn unit_coords(&self, unit_idx: usize) -> (usize, usize) {
        let unit = &self.units[unit_idx];
        (unit.row, unit.column)
    }

    fn lost_elf(&self) -> bool {
        self.units
            .iter()
            .any(|u| u.is_dead() && u.u_type == UnitType::Elf)
    }

    fn buff_elves(&mut self, new_ap: isize) {
        self.units
            .iter_mut()
            .filter(|u| u.u_type == UnitType::Elf)
            .for_each(|u| u.ap = new_ap);
    }
}

#[derive(Clone, Debug)]
struct Unit {
    row: usize,
    column: usize,
    ap: isize,
    hp: isize,
    u_type: UnitType,
}

impl Unit {
    fn new(row: usize, column: usize, u_type: UnitType) -> Self {
        Self {
            row,
            column,
            ap: 3,
            hp: 200,
            u_type,
        }
    }

    fn is_dead(&self) -> bool {
        self.hp <= 0
    }

    fn adjacent_spaces(&self) -> Vec<(usize, usize)> {
        (self.row, self.column).adjacent_spaces()
    }
}

trait AdjacentSpaces {
    fn adjacent_spaces(&self) -> Vec<(usize, usize)>;
}

impl AdjacentSpaces for (usize, usize) {
    fn adjacent_spaces(&self) -> Vec<(usize, usize)> {
        // Reading order: up, left, right, down
        vec![
            (self.0 - 1, self.1),
            (self.0, self.1 - 1),
            (self.0, self.1 + 1),
            (self.0 + 1, self.1),
        ]
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
enum UnitType {
    Elf,
    Goblin,
}

impl TryFrom<char> for UnitType {
    type Error = Error;

    fn try_from(value: char) -> std::result::Result<Self, Self::Error> {
        match value {
            'E' => Ok(Self::Elf),
            'G' => Ok(Self::Goblin),
            _ => bail!("Unrecognized UnitType: '{value}'"),
        }
    }
}
