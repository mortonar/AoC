use anyhow::{Error, Result};
use std::collections::{HashSet, VecDeque};
use std::io::{BufRead, stdin};
use std::str::FromStr;
use z3::{Optimize, SatResult, ast::Int};

fn main() -> Result<()> {
    let machines = parse_input()?;

    let min_steps = machines.iter().map(min_configure_lights_bfs).sum::<usize>();
    println!("Part 1: {min_steps}");

    let min_steps = machines
        .iter()
        .map(min_configure_joltages_z3)
        .sum::<usize>();
    println!("Part 2: {min_steps}");

    Ok(())
}

fn parse_input() -> Result<Vec<Machine>> {
    stdin().lock().lines().map(|line| line?.parse()).collect()
}

fn min_configure_lights_bfs(machine: &Machine) -> usize {
    let mut queue = VecDeque::new();
    queue.push_back(Node {
        machine: machine.clone(),
        button_presses: 0,
    });
    let mut visited = HashSet::new();

    while let Some(current) = queue.pop_front() {
        if current.machine.is_configured_lights() {
            return current.button_presses;
        }

        if visited.insert(current.machine.lights.clone()) {
            current
                .neighbors()
                .into_iter()
                .for_each(|n| queue.push_back(n));
        }
    }

    0
}

/// Cast the problem as a system of indeterminate equations: one equation for each joltage position,
/// one variable per button potentially involved in modifying joltage at position.
///
/// For example:
/// ```[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}```
///            A    B    C    D     E     F
///  E + F = 3
///  B + F = 5
///  C + D + E = 4
///  A + B + D = 7
///
/// Borrowing from some z3 examples :), run through z3 to find an optimized solution.
fn min_configure_joltages_z3(machine: &Machine) -> usize {
    let opt = Optimize::new();
    let total = Int::fresh_const("total");

    let button_vars: Vec<_> = (0..machine.buttons.len())
        .map(|b| Int::fresh_const(&format!("button-{b}")))
        .collect();

    button_vars.iter().for_each(|bv| opt.assert(&bv.ge(0)));

    for (jolt_pos, &jolt_target) in machine.joltages_goal.iter().enumerate() {
        let mut button_terms = Vec::new();

        for (button_idx, button) in machine.buttons.iter().enumerate() {
            // Button impacts this joltage position
            if button.contains(&jolt_pos) {
                button_terms.push(button_vars[button_idx].clone());
            }
        }

        let sum = Int::add(&button_terms.iter().collect::<Vec<&Int>>());
        opt.assert(&sum.eq(Int::from_u64(jolt_target as u64)));
    }

    opt.assert(&total.eq(Int::add(&button_vars)));
    opt.minimize(&total);

    match opt.check(&[]) {
        SatResult::Sat => opt
            .get_model()
            .unwrap()
            .eval(&total, true)
            .and_then(|t| t.as_u64())
            .unwrap() as usize,
        _ => panic!("Error: No solution found"),
    }
}

struct Node {
    machine: Machine,
    button_presses: usize,
}

impl Node {
    fn neighbors(&self) -> Vec<Node> {
        (0..self.machine.buttons.len())
            .map(|button| Node {
                machine: self.machine.press_button(button),
                button_presses: self.button_presses + 1,
            })
            .collect()
    }
}

#[derive(Clone, Debug)]
struct Machine {
    lights_goal: Vec<bool>,
    lights: Vec<bool>,
    buttons: Vec<Vec<usize>>,
    joltages_goal: Vec<usize>,
}

impl Machine {
    fn is_configured_lights(&self) -> bool {
        self.lights == self.lights_goal
    }

    fn press_button(&self, button: usize) -> Self {
        let mut clone = self.clone();

        let button = self.buttons[button].as_slice();
        for to_toggle in button.iter() {
            clone.lights[*to_toggle] = !clone.lights[*to_toggle];
        }

        clone
    }
}

impl FromStr for Machine {
    type Err = Error;

    // e.g.
    // [.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
    // [...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
    // [.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let tokens: Vec<_> = s.split_ascii_whitespace().collect();

        let lights_goal: Vec<_> = tokens[0]
            .chars()
            .filter(|c| matches!(c, '.' | '#'))
            .map(|c| c == '#')
            .collect();

        let lights = vec![false; lights_goal.len()];

        let buttons: Result<Vec<Vec<usize>>> = tokens
            .iter()
            .skip(1)
            .take(tokens.len() - 2)
            .map(|t| {
                t[1..t.len() - 1]
                    .split(",")
                    .map(|b| b.parse())
                    .collect::<Result<Vec<usize>, _>>()
                    .map_err(Error::from)
            })
            .collect();

        let joltages_goal = tokens.last().unwrap();
        let joltages_goal: Result<Vec<usize>> = joltages_goal[1..joltages_goal.len() - 1]
            .split(",")
            .map(|j| j.parse().map_err(Error::from))
            .collect();

        Ok(Self {
            lights_goal,
            lights,
            buttons: buttons?,
            joltages_goal: joltages_goal?,
        })
    }
}
