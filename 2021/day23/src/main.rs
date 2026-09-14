use anyhow::Result;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashSet};
use std::io::stdin;

// #############
// #...........#
// ###B#C#B#D###
//   #A#D#C#A#
//   #########
// vvvvvvvvvvvvv
// * Moving out costs 2-3x (depending if on bottom slot in room or top)
// * Moves in have to be final: must be with nobody else or someone who's already in their place
//
// A room only ever needs to eject its top amphipod (if any occupant is a
// stranger) or accept a hallway amphipod into its top free slot (once every
// current occupant already matches). Direct room-to-room moves are never
// generated explicitly: splitting them into an out-move followed by an
// in-move via any valid hallway stop costs exactly the same, so Dijkstra
// still finds the true shortest path without needing that extra move type.

const ROOM_COUNT: usize = 4;
const HALLWAY_LEN: usize = 11;
const ROOM_COLS: [usize; ROOM_COUNT] = [2, 4, 6, 8];

// The two extra rows folded into each room for part 2, read top-to-bottom.
const EXTRA_ROW1: [char; ROOM_COUNT] = ['D', 'C', 'B', 'A'];
const EXTRA_ROW2: [char; ROOM_COUNT] = ['D', 'B', 'A', 'C'];

fn main() -> Result<()> {
    let stacks = parse_input()?;

    println!("Part 1: {}", part1(&stacks));
    println!("Part 2: {}", part2(&stacks));

    Ok(())
}

// Supplied in transformed stacks to make this easier to parse: e.g. AB CD CB AD
// Each stack lists a room bottom-to-top, e.g. "AB" is bottom A, top B.
fn parse_input() -> Result<Vec<Vec<char>>> {
    let mut line = String::new();
    stdin().read_line(&mut line)?;
    let stacks = line
        .trim()
        .split_ascii_whitespace()
        .map(|stack| stack.chars().collect())
        .collect();
    Ok(stacks)
}

fn part1(stacks: &[Vec<char>]) -> usize {
    sort_dij(stacks)
}

fn part2(stacks: &[Vec<char>]) -> usize {
    let unfolded: Vec<Vec<char>> = stacks
        .iter()
        .enumerate()
        .map(|(i, stack)| {
            let mut room = vec![stack[0], EXTRA_ROW2[i], EXTRA_ROW1[i]];
            room.extend(&stack[1..]);
            room
        })
        .collect();
    sort_dij(&unfolded)
}

fn sort_dij(stacks: &[Vec<char>]) -> usize {
    let depth = stacks[0].len();
    let init = State {
        rooms: stacks.to_owned(),
        hallway: [None; HALLWAY_LEN],
    };
    let mut queue = BinaryHeap::from([(Reverse(0usize), init)]);
    let mut visited: HashSet<State> = HashSet::new();

    while let Some((Reverse(cost), state)) = queue.pop() {
        if state.solved(depth) {
            return cost;
        }

        if !visited.insert(state.clone()) {
            continue;
        }

        for (next, step_cost) in state.moves(depth) {
            if !visited.contains(&next) {
                queue.push((Reverse(cost + step_cost), next));
            }
        }
    }

    unreachable!("priority queue exhausted without finding a solution")
}

fn target_room(amphipod: char) -> usize {
    (amphipod as u8 - b'A') as usize
}

fn step_cost(amphipod: char) -> usize {
    10usize.pow(target_room(amphipod) as u32)
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
struct State {
    rooms: Vec<Vec<char>>,
    hallway: [Option<char>; HALLWAY_LEN],
}

impl State {
    fn solved(&self, depth: usize) -> bool {
        self.hallway.iter().all(Option::is_none)
            && self
                .rooms
                .iter()
                .enumerate()
                .all(|(i, room)| room.len() == depth && room.iter().all(|&c| target_room(c) == i))
    }

    fn moves(&self, depth: usize) -> Vec<(State, usize)> {
        let mut moves = Vec::new();
        self.room_to_hallway_moves(depth, &mut moves);
        self.hallway_to_room_moves(depth, &mut moves);
        moves
    }

    // An amphipod may leave a room only if a stranger is in there somewhere;
    // it always leaves from the top, and may stop on any clear, valid spot.
    fn room_to_hallway_moves(&self, depth: usize, moves: &mut Vec<(State, usize)>) {
        for (room_idx, room) in self.rooms.iter().enumerate() {
            if room.is_empty() || room.iter().all(|&c| target_room(c) == room_idx) {
                continue;
            }

            let amphipod = *room.last().unwrap();
            let steps_out = depth - room.len() + 1;
            let room_col = ROOM_COLS[room_idx];

            for hall_col in hallway_stops() {
                if self.path_clear(room_col, hall_col) {
                    let steps = steps_out + room_col.abs_diff(hall_col);
                    let mut next = self.clone();
                    next.rooms[room_idx].pop();
                    next.hallway[hall_col] = Some(amphipod);
                    moves.push((next, steps * step_cost(amphipod)));
                }
            }
        }
    }

    // An amphipod may enter its destination room only once every occupant
    // already in there matches it, and it always settles into the lowest
    // free slot.
    fn hallway_to_room_moves(&self, depth: usize, moves: &mut Vec<(State, usize)>) {
        for (hall_col, occupant) in self.hallway.iter().enumerate() {
            let Some(amphipod) = occupant else {
                continue;
            };

            let room_idx = target_room(*amphipod);
            let room = &self.rooms[room_idx];
            if room.len() >= depth || !room.iter().all(|&c| c == *amphipod) {
                continue;
            }

            let room_col = ROOM_COLS[room_idx];
            if !self.path_clear(hall_col, room_col) {
                continue;
            }

            let steps = (depth - room.len()) + room_col.abs_diff(hall_col);
            let mut next = self.clone();
            next.hallway[hall_col] = None;
            next.rooms[room_idx].push(*amphipod);
            moves.push((next, steps * step_cost(*amphipod)));
        }
    }

    // Whether the hallway is unobstructed between `from` (exclusive, since
    // that's where the mover currently stands) and `to` (inclusive).
    fn path_clear(&self, from: usize, to: usize) -> bool {
        let (lo, hi) = (from.min(to), from.max(to));
        (lo..=hi).all(|c| c == from || self.hallway[c].is_none())
    }
}

fn hallway_stops() -> impl Iterator<Item = usize> {
    (0..HALLWAY_LEN).filter(|c| !ROOM_COLS.contains(c))
}
