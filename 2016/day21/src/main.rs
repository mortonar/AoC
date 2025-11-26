use anyhow::{Error, Result, anyhow};
use std::collections::HashSet;
use std::io::{BufRead, stdin};
use std::str::FromStr;

fn main() -> Result<()> {
    let mut scrambler = Scrambler::default();
    for line in stdin().lock().lines() {
        scrambler.add_instruction(line?.parse()?);
    }

    let scrambled = scrambler.scramble("abcdefgh")?;
    println!("Part 1: {}", &scrambled);

    // Part 2 is less straightforward since the letter-based rotate instruction is non-reversible.
    // All the other instructions are reversible though. Treat this like a search problem instead.
    // Undoing the other instructions is a branch of 1 but branch for all possible rotations when
    // undoing the letter-based rotate instruction.
    let scrambled = "fbgdceah";
    let unscrambled = scrambler.unscramble_dfs(scrambled)?;
    println!("Part 2: {}", &unscrambled);

    Ok(())
}

#[derive(Default)]
struct Scrambler {
    instructions: Vec<Instruction>,
}

impl Scrambler {
    fn add_instruction(&mut self, instruction: Instruction) {
        self.instructions.push(instruction);
    }

    fn scramble(&self, password: &str) -> Result<String> {
        let mut password: Vec<_> = password.chars().collect();
        for ins in self.instructions.iter() {
            ins.apply(&mut password)?;
        }
        Ok(password.iter().collect())
    }

    fn unscramble_dfs(&self, scrambled: &str) -> Result<String> {
        // This actually started as a BFS solution until I realized DFS runs MUCH quicker.
        // As I understand it, we know our solution is going to be on the leaves of the search tree.
        // So the quicker we can direct the search there, the better.
        // NOTE: The only difference between BFS and iterative DFS is a queue vs a stack :)
        let mut stack = Vec::new();
        stack.push(Node::new(scrambled, self.instructions.len()));
        let mut visited = HashSet::new();

        while let Some(current) = stack.pop() {
            if current.instruction_num == 0 {
                let password: String = current.password.iter().collect();
                if let Ok(s) = self.scramble(&password)
                    && s.eq(scrambled)
                {
                    return Ok(password);
                }
                continue;
            }

            if !visited.insert(current.clone()) {
                continue;
            }

            current
                .neighbors(self)
                .into_iter()
                .for_each(|n| stack.push(n));
        }

        Err(anyhow!("Path not found"))
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Node {
    password: Vec<char>,
    // - 1 for index into list
    instruction_num: usize,
}

impl Node {
    fn neighbors(&self, scrambler: &Scrambler) -> Vec<Node> {
        match &scrambler.instructions[self.instruction_num - 1] {
            Instruction::SwapPos(x, y) => {
                let mut neighbor = self.clone();
                neighbor.instruction_num -= 1;
                neighbor.password.swap(*y, *x);
                vec![neighbor]
            }
            Instruction::SwapLetter(x, y) => {
                let mut neighbor = self.clone();
                neighbor.instruction_num -= 1;
                for c in neighbor.password.iter_mut() {
                    if *c == *y {
                        *c = *x;
                    } else if *c == *x {
                        *c = *y;
                    }
                }
                vec![neighbor]
            }
            Instruction::RotateDir(dir, steps) => {
                let mut neighbor = self.clone();
                neighbor.instruction_num -= 1;
                let rev = Instruction::RotateDir(dir.reverse(), *steps);
                rev.apply(&mut neighbor.password).unwrap();
                vec![neighbor]
            }
            Instruction::RotateLetter(_) => (0..self.password.len())
                .map(|i| {
                    let mut neighbor = self.clone();
                    neighbor.instruction_num -= 1;
                    let rotate = Instruction::RotateDir(Dir::Left, i);
                    rotate.apply(&mut neighbor.password).unwrap();
                    neighbor
                })
                .collect(),
            Instruction::RevRange(x, y) => {
                let mut neighbor = self.clone();
                neighbor.instruction_num -= 1;
                neighbor.password[*x..=*y].reverse();
                vec![neighbor]
            }
            Instruction::Move(x, y) => {
                let mut neighbor = self.clone();
                neighbor.instruction_num -= 1;
                let c = neighbor.password.remove(*y);
                if *x == neighbor.password.len() {
                    neighbor.password.push(c);
                } else {
                    neighbor.password.insert(*x, c);
                }
                vec![neighbor]
            }
        }
    }
}

impl Node {
    fn new(password: &str, instruction: usize) -> Self {
        Self {
            password: password.chars().collect(),
            instruction_num: instruction,
        }
    }
}

#[derive(Clone)]
enum Instruction {
    SwapPos(usize, usize),
    SwapLetter(char, char),
    RotateDir(Dir, usize),
    RotateLetter(char),
    RevRange(usize, usize),
    Move(usize, usize),
}

#[derive(Clone)]
enum Dir {
    Left,
    Right,
}

impl Dir {
    fn reverse(&self) -> Self {
        match self {
            Dir::Left => Dir::Right,
            Dir::Right => Dir::Left,
        }
    }
}

impl Instruction {
    fn apply(&self, password: &mut Vec<char>) -> Result<()> {
        match self {
            Instruction::SwapPos(x, y) => password.swap(*x, *y),
            Instruction::SwapLetter(x, y) => {
                for c in password.iter_mut() {
                    if *c == *x {
                        *c = *y;
                    } else if *c == *y {
                        *c = *x;
                    }
                }
            }
            Instruction::RotateDir(dir, steps) => {
                let steps = steps % password.len();
                match dir {
                    Dir::Left => password.rotate_left(steps),
                    Dir::Right => password.rotate_right(steps),
                }
            }
            Instruction::RotateLetter(letter) => {
                if let Some(index) = password.iter().position(|c| *c == *letter) {
                    let mut steps = index + 1;
                    if index >= 4 {
                        steps += 1;
                    }
                    steps %= password.len();
                    password.rotate_right(steps);
                } else {
                    return Err(anyhow!("{} does not exist in password", letter));
                }
            }
            Instruction::RevRange(x, y) => password[*x..=*y].reverse(),
            Instruction::Move(x, y) => {
                let c = password.remove(*x);
                if *y >= password.len() {
                    password.push(c);
                } else {
                    password.insert(*y, c);
                }
            }
        }
        Ok(())
    }
}

impl FromStr for Dir {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let s = s.trim().to_ascii_lowercase();
        if s == "left" {
            Ok(Dir::Left)
        } else if s == "right" {
            Ok(Dir::Right)
        } else {
            Err(anyhow!("Unrecognized direction: {}", s))
        }
    }
}

impl FromStr for Instruction {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let tokens: Vec<_> = s.split_ascii_whitespace().collect();
        match tokens.as_slice() {
            ["swap", "position", x, "with", "position", y] => {
                Ok(Instruction::SwapPos(x.parse()?, y.parse()?))
            }
            ["swap", "letter", x, "with", "letter", y] => {
                Ok(Instruction::SwapLetter(x.parse()?, y.parse()?))
            }
            ["rotate", dir, steps, _steps] => {
                Ok(Instruction::RotateDir(dir.parse()?, steps.parse()?))
            }
            ["rotate", "based", "on", "position", "of", "letter", letter] => {
                Ok(Instruction::RotateLetter(letter.parse()?))
            }
            ["reverse", "positions", x, "through", y] => {
                Ok(Instruction::RevRange(x.parse()?, y.parse()?))
            }
            ["move", "position", x, "to", "position", y] => {
                Ok(Instruction::Move(x.parse()?, y.parse()?))
            }
            _ => Err(anyhow!("Unrecognized instruction: {}", s)),
        }
    }
}
