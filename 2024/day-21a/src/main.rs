use anyhow::Result;
use std::collections::HashMap;
use std::env;
use std::io::BufRead;

// Num keypad <-- Robot 1 (Dir keypad) <-Robot 2 (Dir keypad) <- Robot 3 (Dir keypad) <- User
// Algorithm:
// * Find the shortest paths for the numpad
// * For each of these paths: find the shortest paths on the dir pad.
// * For each of these paths: find the shortest paths on the next dir pad.
// * etc.
// * Compute the complexity based on the final dir pad.
// Finding shortest paths for a sequence of input can be a greedy DFS between each pair of input keys
//
// NOTE: The robots can't use empty gaps on key/dir pads in their pathing. It doesn't really make
//       sense for the user to do so either.
//
// +---+---+---+
// | 7 | 8 | 9 |
// +---+---+---+
// | 4 | 5 | 6 |
// +---+---+---+
// | 1 | 2 | 3 |
// +---+---+---+
//     | 0 | A |
//     +---+---+
//
// Directional keypad:
//     +---+---+     XXXXX
//     | ^ | A | --> XX^AX
// +---+---+---+     X<v>X
// | < | v | > |     XXXXX
// +---+---+---+
fn main() -> Result<()> {
    let mut codes = Vec::new();
    for line in std::io::stdin().lock().lines() {
        codes.push(line?.trim().chars().collect::<Vec<char>>());
    }

    let args: Vec<String> = env::args().collect();
    let robot_dirpad_count = args[1].parse::<usize>()?;
    let mut keypads = Vec::with_capacity(robot_dirpad_count + 1);
    keypads.push(Keypad::new_numeric());
    (0..robot_dirpad_count).for_each(|_| keypads.push(Keypad::new_directional()));

    let sum = codes
        .iter()
        .map(|code| Keypad::complexity(&code, &mut keypads[..]))
        .sum::<usize>();
    println!("{sum}");

    Ok(())
}

struct Keypad {
    grid: HashMap<char, (usize, usize)>,
}

impl Keypad {
    fn new_numeric() -> Keypad {
        Self {
            #[rustfmt::skip]
            grid: HashMap::from([
                ('7', (0, 0)), ('8', (0, 1)), ('9', (0, 2)),
                ('4', (1, 0)), ('5', (1, 1)), ('6', (1, 2)),
                ('1', (2, 0)), ('2', (2, 1)), ('3', (2, 2)),
                               ('0', (3, 1)), ('A', (3, 2)),
            ]),
        }
    }

    fn new_directional() -> Keypad {
        Self {
            #[rustfmt::skip]
            grid: HashMap::from([
                               ('^', (0, 1)), ('A', (0, 2)),
                ('<', (1, 0)), ('v', (1, 1)), ('>', (1, 2)),
            ]),
        }
    }

    fn complexity(code: &[char], keypads: &mut [Keypad]) -> usize {
        let numpad = &keypads[0];
        let init_key_seq = numpad.greedy_sequence(code);

        let robots = &mut keypads[1..];
        let shortest_seq = Self::find_shortest_sequence(init_key_seq, robots);

        let num: usize = code[0..code.len() - 1]
            .iter()
            .map(|c| *c)
            .collect::<String>()
            .parse()
            .expect("Can't parse code");

        num * shortest_seq.len()
    }

    fn find_shortest_sequence(sequence: Vec<char>, robots: &mut [Keypad]) -> Vec<char> {
        if robots.len() == 0 {
            return sequence;
        }
        let new_seq = robots[0].greedy_sequence(&sequence);
        Self::find_shortest_sequence(new_seq, &mut robots[1..])
    }

    fn greedy_sequence(&self, seq: &[char]) -> Vec<char> {
        let mut to_translate = Vec::new();
        // Paths always start at 'A'
        to_translate.push('A');
        seq.iter().for_each(|c| to_translate.push(*c));

        to_translate
            .windows(2)
            .flat_map(|w| self.greedy_subsequence(w[0], w[1]))
            .collect()
    }

    fn greedy_subsequence(&self, from: char, to: char) -> Vec<char> {
        if !self.grid.contains_key(&from) || !self.grid.contains_key(&to) {
            panic!("{from} and {to} must both be on the keypad");
        }

        let (fx, fy) = self.grid[&from];
        let (tx, ty) = self.grid[&to];

        if fx == tx {
            // Same row
            let c = if fy > ty { '<' } else { '>' };
            let repeat = fy.abs_diff(ty);
            Self::build_seq(vec![(c, repeat)])
        } else if fy == ty {
            // Same column
            let c = if fx > tx { '^' } else { 'v' };
            let repeat = fx.abs_diff(tx);
            Self::build_seq(vec![(c, repeat)])
        } else {
            // Diagonal move: where the "hole" is in the keypad matters now
            let num_layout = self.grid.contains_key(&'9');
            if num_layout {
                let lr = if fy < ty { '>' } else { '<' };
                let lr_repeat = fy.abs_diff(ty);
                let ud = if fx < tx { 'v' } else { '^' };
                let ud_repeat = fx.abs_diff(tx);

                if fx == 3 && ty == 0 {
                    // If on bottom row and going to the left column: updo,leri
                    Self::build_seq(vec![(ud, ud_repeat), (lr, lr_repeat)])
                } else if fy == 0 && tx == 3 {
                    // If on far left column and traveling to bottom row: leri,updo
                    Self::build_seq(vec![(lr, lr_repeat), (ud, ud_repeat)])
                } else {
                    // Diagonal rules
                    match Diagonal::from_coords((fx, fy), (tx, ty)) {
                        Diagonal::UpLeft | Diagonal::DownLeft => {
                            Self::build_seq(vec![(lr, lr_repeat), (ud, ud_repeat)])
                        }
                        Diagonal::UpRight | Diagonal::DownRight => {
                            Self::build_seq(vec![(ud, ud_repeat), (lr, lr_repeat)])
                        }
                    }
                }
            } else {
                if (tx, ty) == (1, 0) {
                    let l_repeat = fy.abs_diff(ty);
                    Self::build_seq(vec![('v', 1), ('<', l_repeat)])
                } else if (fx, fy) == (1, 0) {
                    let r_repeat = fy.abs_diff(ty);
                    Self::build_seq(vec![('>', r_repeat), ('^', 1)])
                } else {
                    match Diagonal::from_coords((fx, fy), (tx, ty)) {
                        Diagonal::UpLeft => Self::build_seq(vec![('<', 1), ('^', 1)]),
                        Diagonal::DownLeft => Self::build_seq(vec![('<', 1), ('v', 1)]),
                        Diagonal::UpRight => Self::build_seq(vec![('^', 1), ('>', 1)]),
                        Diagonal::DownRight => Self::build_seq(vec![('v', 1), ('>', 1)]),
                    }
                }
            }
        }
    }

    fn build_seq(spec: Vec<(char, usize)>) -> Vec<char> {
        let mut sequence = Vec::new();
        for (c, repeat) in spec {
            (0..repeat).for_each(|_| sequence.push(c));
        }
        sequence.push('A');
        sequence
    }
}

enum Diagonal {
    UpLeft,
    UpRight,
    DownLeft,
    DownRight,
}

impl Diagonal {
    fn from_coords(from: (usize, usize), to: (usize, usize)) -> Self {
        let (fx, fy) = from;
        let (tx, ty) = to;
        if fx > tx {
            if fy > ty {
                Self::UpLeft
            } else {
                Self::UpRight
            }
        } else {
            if fy > ty {
                Self::DownLeft
            } else {
                Self::DownRight
            }
        }
    }
}
