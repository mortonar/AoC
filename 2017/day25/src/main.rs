use anyhow::{Result, anyhow};
use std::io::stdin;

fn main() -> Result<()> {
    let mut turing_machine = parse_input()?;

    let checksum = turing_machine.run_til_diag();
    println!("Part 1: {checksum}");

    Ok(())
}

#[derive(Default, Debug)]
struct TuringMachine {
    tape: Vec<bool>,
    state: char,
    cursor: usize,
    states: [State; 26],
    diag_steps: usize,
}

const TAPE_LEN: usize = 20_000_000;

impl TuringMachine {
    /// A hack: place us in the middle of a tape big enough to never require resizing on writes.
    fn new(begin_state: char, states: [State; 26], diag_steps: usize) -> Self {
        Self {
            state: begin_state,
            tape: vec![false; TAPE_LEN],
            cursor: TAPE_LEN / 2,
            states,
            diag_steps,
        }
    }

    fn run_til_diag(&mut self) -> usize {
        for _i in 0..self.diag_steps {
            self.states[self.state as usize - 'A' as usize]
                // I'm too lazy to refactor this but the clone is cheap
                .clone()
                .apply(self);
        }
        self.tape.iter().filter(|t| **t).count()
    }
}

#[derive(Copy, Clone, Default, Debug)]
struct State {
    zero_rule: Rule,
    one_rule: Rule,
}

impl State {
    fn apply(&self, machine: &mut TuringMachine) {
        if machine.tape[machine.cursor] {
            self.one_rule.apply(machine);
        } else {
            self.zero_rule.apply(machine);
        }
    }
}

#[derive(Copy, Clone, Default, Debug)]
struct Rule {
    val: bool,
    movement: isize,
    next: char,
}

impl Rule {
    fn apply(&self, machine: &mut TuringMachine) {
        machine.tape[machine.cursor] = self.val;

        if self.movement == -1 {
            machine.cursor -= 1
        } else {
            machine.cursor += 1
        }

        machine.state = self.next;
    }
}

// Ignore this abhorrent parsing code, please...
fn parse_input() -> Result<TuringMachine> {
    use std::io::BufRead;
    let mut lines = stdin()
        .lock()
        .lines()
        .map(|l| l.map(|s| s.trim().to_string()));

    let begin_state = parse_last_char(
        &lines
            .next()
            .transpose()?
            .ok_or_else(|| anyhow!("Missing begin state"))?,
    )?;
    let diag_line = lines
        .next()
        .transpose()?
        .ok_or_else(|| anyhow!("Missing diagnostic line"))?;
    let diag_steps: usize = diag_line
        .split_ascii_whitespace()
        .nth(5)
        .ok_or_else(|| anyhow!("Missing diagnostic steps"))?
        .parse()?;
    let _blank = lines.next();

    let mut states = [State::default(); 26];
    while let Some(state_line) = lines.next().transpose()? {
        let state = parse_last_char(&state_line)?;
        let zero_rule = parse_rule(&mut lines)?;
        let one_rule = parse_rule(&mut lines)?;
        let _blank = lines.next();
        let idx = state as usize - 'A' as usize;
        states[idx].zero_rule = zero_rule;
        states[idx].one_rule = one_rule;
    }

    Ok(TuringMachine::new(begin_state, states, diag_steps))
}

fn parse_rule<I>(lines: &mut I) -> Result<Rule>
where
    I: Iterator<Item = std::io::Result<String>>,
{
    let _marker = lines
        .next()
        .transpose()?
        .ok_or_else(|| anyhow!("Missing rule marker"))?;
    let val = parse_last_char(
        &lines
            .next()
            .transpose()?
            .ok_or_else(|| anyhow!("Missing value line"))?,
    )? == '1';
    let movement = if parse_last_token(
        &lines
            .next()
            .transpose()?
            .ok_or_else(|| anyhow!("Missing movement line"))?,
    )? == "left."
    {
        -1
    } else {
        1
    };
    let next = parse_last_char(
        &lines
            .next()
            .transpose()?
            .ok_or_else(|| anyhow!("Missing next state line"))?,
    )?;
    Ok(Rule {
        val,
        movement,
        next,
    })
}

/// Parse first char of last token
fn parse_last_char(line: &str) -> Result<char> {
    parse_last_token(line)
        .and_then(|l| l.chars().next().ok_or_else(|| anyhow!("Can't parse state")))
}

/// Parse the last token in a line
fn parse_last_token(line: &str) -> Result<&str> {
    line.split_ascii_whitespace()
        .last()
        .ok_or_else(|| anyhow!("No last token found"))
}
