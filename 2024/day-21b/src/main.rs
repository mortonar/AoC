use anyhow::Result;
use std::collections::HashMap;
use std::env;
use std::io::BufRead;

fn main() -> Result<()> {
    let mut codes = Vec::new();
    for line in std::io::stdin().lock().lines() {
        codes.push(line?.trim().chars().collect::<Vec<char>>());
    }

    let args: Vec<String> = env::args().collect();
    let robot_dirpad_count = args
        .get(1)
        .expect("Must give a dirpad count")
        .parse::<usize>()?;
    let mut keypads = Vec::with_capacity(robot_dirpad_count + 1);
    keypads.push(Keypad::new_numeric());
    (0..robot_dirpad_count).for_each(|_| keypads.push(Keypad::new_directional()));

    // A cache which takes a sub-sequence at a given robot level and returns the length of what
    // the final version of that sub-sequence will become.
    let mut cache: HashMap<(Vec<char>, usize), usize> = HashMap::new();

    let sum = codes
        .iter()
        .map(|code| Keypad::complexity(&code, &mut keypads[..], &mut cache))
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

    fn complexity(
        code: &[char],
        keypads: &mut [Keypad],
        cache: &mut HashMap<(Vec<char>, usize), usize>,
    ) -> usize {
        let shortest_seq_len = Self::find_shortest_sequence(code.into(), keypads, cache);

        let num: usize = code[0..code.len() - 1]
            .iter()
            .collect::<String>()
            .parse()
            .expect("Can't parse code");

        num * shortest_seq_len
    }

    // It took SO many cracks at this problem to figure out how to decompose it in the correct way
    // that both gives correct answers and leaves you with sub-problems that can be memoized.
    // Because memoization is essential for part 2.
    //
    // This ended up being a great solution to reference:
    // https://github.com/tmo1/adventofcode/blob/main/2024/21b.py
    //
    // The idea is to take the sequence at the first numpad and recursively compute what that
    // sequence would become at the next (dir)pad, then the next pad, etc. However, we don't want to
    // translate the FULL sequence because that's not going to be a common sub-problem to solve as
    // the sequence will become very large very fast.
    //
    // Instead, we want to decompose each sequence into (from, to) key moves and treat the path
    // between them as the next sequence to recurse on. This preserves the ordering of the moves in
    // the final sequence to ensure you still end up with the right answer. Additionally, you end up
    // with shorter subsequences that occur more frequently and can be memoized.
    fn find_shortest_sequence(
        sequence: Vec<char>,
        keypads: &[Keypad],
        cache: &mut HashMap<(Vec<char>, usize), usize>,
    ) -> usize {
        if keypads.len() == 0 {
            return sequence.len();
        }

        if let Some(cached) = cache.get(&(sequence.clone(), keypads.len())) {
            return *cached;
        }

        let mut sublen = 0;
        for (i, &to) in sequence.iter().enumerate() {
            // Every bot needs to hit 'A' in between keys on the pad it's controlling (not using)
            // Every pad input also starts at 'A'
            let from = if i == 0 { 'A' } else { sequence[i - 1] };
            let new_seq = keypads[0].greedy_subsequence(from, to);
            sublen += Self::find_shortest_sequence(new_seq, &keypads[1..], cache);
        }
        cache.insert((sequence.clone(), keypads.len()), sublen);
        sublen
    }

    // Folks (who are WAY smarter than me) figured out that there are optimal paths for every case
    // that are actually globally optimal solutions no matter how many robots you add.
    // https://www.reddit.com/r/adventofcode/comments/1hjgyps/2024_day_21_part_2_i_got_greedyish/
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
