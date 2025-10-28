use anyhow::Result;
use itertools::Itertools;
use std::collections::{HashSet, VecDeque};
use std::io::BufRead;

// * Generators (G) of a certain type pair with Microchips (M) of the same type.
// * Ms can only generate shield if powered by their corresponding G.
//   * An M will be 'fried' if in the presence of other Gs unless it's powered by its corresponding G
//   * Keep Ms powered if they're in the same room as other Gs or otherwise in different room.
// * Elevator can carry at most yourself and two Gs or Ms in any combination
//   * Elevator won't function unless it has one G or M
//   * Elevator always stops on each floor to recharge, and this takes long enough that the items
//     within it and the items on that floor can irradiate each other. This is preventable if a G is
//     present to charge its corresponding M. G with another G is fine though. The Ms need protected.
//
// * GOAL: Starting on the first floor, bring all Ms and Gs to the 4th floor safely
//         Get minimum number of steps
fn main() -> Result<()> {
    let mut floors: Vec<HashSet<Item>> = vec![HashSet::new(); 4];

    for line in std::io::stdin().lock().lines() {
        let line = line?;
        let tokens = line.split_whitespace().collect::<Vec<_>>();

        let floor = match tokens[1] {
            "first" => 0,
            "second" => 1,
            "third" => 2,
            "fourth" => 3,
            _ => return Err(anyhow::anyhow!("invalid floor {}", tokens[1])),
        };

        for (i, t) in tokens.iter().enumerate().skip(1) {
            let mut label = tokens[i - 1];
            if label.ends_with("-compatible") {
                label = label.strip_suffix("-compatible").unwrap();
            }
            let label = label.to_string();

            if t.contains("generator") {
                floors[floor].insert(Item::Generator(label));
            } else if t.contains("microchip") {
                floors[floor].insert(Item::Microchip(label));
            }
        }
    }
    println!("Part 1: {}", min_steps_bfs(&floors)?);

    Ok(())
}

fn min_steps_bfs(initial_config: &[HashSet<Item>]) -> Result<usize> {
    let mut queue = VecDeque::new();
    queue.push_back(Context {
        elevator: 0,
        floor_config: initial_config.to_owned(),
        steps: 0,
    });
    let mut visited: Vec<Context> = Vec::new();

    while let Some(config) = queue.pop_front() {
        // config.print();

        if config.floor_config.is_goal() {
            return Ok(config.steps);
        }

        // TODO This "branch checking" check on visited will become more and more expensive as the search goes on since it's a growing linear search.
        //      The config/state representation is too bulky and not hashable since it contains inner hashsets.
        //      Let's try to encode the states as bitmasks for the items so we have something hashable.
        if visited
            .iter()
            .any(|v| v.eq(&config) && v.elevator == config.elevator && v.steps < config.steps)
        {
            continue;
        }

        let mut taking = config.floor_config[config.elevator]
            .iter()
            .combinations(1)
            .collect::<Vec<_>>();
        let mut take_two = config.floor_config[config.elevator]
            .iter()
            .combinations(2)
            .collect::<Vec<_>>();
        taking.append(&mut take_two);
        // dbg!(&taking);
        for taking in taking {
            let taking: Vec<Item> = taking.iter().map(|i| (*i).clone()).collect();
            let mut new_config = config.clone();
            new_config.floor_config[config.elevator].retain(|i| !taking.contains(i));
            new_config.steps += 1;

            if config.elevator < 3 {
                let mut up_config = new_config.clone();
                up_config.elevator += 1;
                for t in &taking {
                    up_config.floor_config[up_config.elevator].insert(t.clone());
                }
                if up_config.floor_config.is_valid_state() {
                    visited.push(up_config.clone());
                    queue.push_back(up_config);
                } else {
                    // println!("Invalid:");
                    // up_config.print();
                }
            }
            if config.elevator > 0 {
                let mut down_config = new_config.clone();
                down_config.elevator -= 1;
                for t in &taking {
                    down_config.floor_config[down_config.elevator].insert(t.clone());
                }
                if down_config.floor_config.is_valid_state() {
                    visited.push(down_config.clone());
                    queue.push_back(down_config)
                } else {
                    // println!("Invalid:");
                    // down_config.print();
                }
            }
        }
    }

    Err(anyhow::anyhow!("No paths found"))
}

#[derive(Debug, Clone)]
struct Context {
    elevator: usize,
    floor_config: Vec<HashSet<Item>>,
    steps: usize,
}

impl PartialEq<Self> for Context {
    fn eq(&self, other: &Self) -> bool {
        self.floor_config
            .iter()
            .zip(other.floor_config.iter())
            .all(|(l, r)| l.eq(r))
    }
}

impl Context {
    fn print(&self) {
        println!("------------");
        println!("elevator: {}", self.elevator);
        println!("steps: {}", self.steps);
        for (i, floor) in self.floor_config.iter().enumerate().rev() {
            print!("F{} ", i + 1);
            for item in floor {
                match item {
                    Item::Generator(l) => print!("G-{l} "),
                    Item::Microchip(l) => print!("M-{l} "),
                }
            }
            println!();
        }
        println!("------------");
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum Item {
    Generator(String),
    Microchip(String),
}

trait ValidState {
    fn is_valid_state(&self) -> bool;
}

impl ValidState for HashSet<Item> {
    fn is_valid_state(&self) -> bool {
        if !self.iter().any(|i| matches!(i, Item::Generator(_))) {
            return true;
        }

        for item in self.iter() {
            if let Item::Microchip(m_label) = item {
                let mut matching_gen = false;
                for item2 in self.iter() {
                    if let Item::Generator(g_label) = item2 {
                        if m_label.eq(g_label) {
                            matching_gen = true;
                            break;
                        }
                    }
                }
                if !matching_gen {
                    return false;
                }
            }
        }

        true
    }
}

impl ValidState for Vec<HashSet<Item>> {
    fn is_valid_state(&self) -> bool {
        self.iter().all(|l| l.is_valid_state())
    }
}

trait IsGoal {
    fn is_goal(&self) -> bool;
}

impl IsGoal for Vec<HashSet<Item>> {
    fn is_goal(&self) -> bool {
        self.iter().rev().skip(1).all(|floor| floor.is_empty())
    }
}
