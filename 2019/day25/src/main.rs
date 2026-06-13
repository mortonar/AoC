use anyhow::{Result, bail};
use intcode::{IntcodeComputer, Program, RunState, parse_program};
use std::collections::HashSet;

fn main() -> Result<()> {
    let program = parse_input()?;
    solve(&program)?;
    Ok(())
}

fn parse_input() -> Result<Program> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line)?;
    parse_program(&line)
}

fn solve(program: &Program) -> Result<()> {
    let mut droid = IntcodeComputer::new(program);

    let mut initial_output = String::new();
    loop {
        match droid.run() {
            RunState::Halted => break,
            RunState::ProducedOutput => {
                initial_output.push(droid.output.pop_front().unwrap() as u8 as char);
            }
            RunState::AwaitingInput => break,
        }
    }

    let mut visited: HashSet<String> = HashSet::new();
    let mut collected_items: Vec<String> = Vec::new();
    let mut checkpoint_path: Vec<String> = Vec::new();
    let mut security_dir: Option<String> = None;
    let mut path = Vec::new();
    explore(
        &mut droid,
        &initial_output,
        &mut path,
        &mut visited,
        &mut collected_items,
        &mut checkpoint_path,
        &mut security_dir,
    );

    for dir in &checkpoint_path {
        send_command(&mut droid, dir);
    }

    let security_dir = security_dir.unwrap_or_else(|| {
        for dir in &["north", "south", "east", "west"] {
            let test_output = send_command(&mut droid, dir);
            if test_output.contains("Pressure-Sensitive Floor") {
                return dir.to_string();
            }
            let (room, _, _) = parse_room(&test_output);
            if !room.is_empty() && room != "Security Checkpoint" {
                send_command(&mut droid, opposite_dir(dir));
            }
        }
        "north".to_string()
    });

    let n = collected_items.len();

    for item in &collected_items {
        send_command(&mut droid, &format!("drop {}", item));
    }

    for mask in 0..(1 << n) {
        let mut current_items: Vec<&String> = Vec::new();
        for (i, item) in collected_items.iter().enumerate() {
            if mask & (1 << i) != 0 {
                send_command(&mut droid, &format!("take {}", item));
                current_items.push(item);
            }
        }

        let result = send_command(&mut droid, &security_dir);

        if !result.contains("lighter") && !result.contains("heavier") {
            println!("{}", result);
            return Ok(());
        }

        for item in &current_items {
            send_command(&mut droid, &format!("drop {}", item));
        }
    }

    bail!("Could not find correct item combination")
}

fn explore(
    droid: &mut IntcodeComputer,
    current_output: &str,
    path: &mut Vec<String>,
    visited: &mut HashSet<String>,
    collected_items: &mut Vec<String>,
    checkpoint_path: &mut Vec<String>,
    security_dir: &mut Option<String>,
) {
    let (room_name, doors, items) = parse_room(current_output);

    if room_name.is_empty() || visited.contains(&room_name) {
        return;
    }
    visited.insert(room_name.clone());

    for item in &items {
        if !DANGEROUS_ITEMS.contains(&item.as_str()) {
            let cmd = format!("take {}", item);
            send_command(droid, &cmd);
            collected_items.push(item.clone());
        }
    }

    if room_name == "Security Checkpoint" {
        *checkpoint_path = path.clone();
        for door in &doors {
            if door != opposite_dir(path.last().unwrap_or(&String::new())) {
                let test_output = send_command(droid, door);
                if test_output.contains("Pressure-Sensitive Floor")
                    || test_output.contains("Security Checkpoint")
                {
                    *security_dir = Some(door.clone());
                }
            }
        }
        return;
    }

    for door in &doors {
        path.push(door.clone());
        let new_output = send_command(droid, door);

        let (new_room, _, _) = parse_room(&new_output);
        if !new_room.is_empty() && !visited.contains(&new_room) {
            explore(
                droid,
                &new_output,
                path,
                visited,
                collected_items,
                checkpoint_path,
                security_dir,
            );

            send_command(droid, opposite_dir(door));
        } else if !new_room.is_empty() && new_room != room_name {
            send_command(droid, opposite_dir(door));
        }
        path.pop();
    }
}

const DANGEROUS_ITEMS: &[&str] = &[
    "giant electromagnet",
    "infinite loop",
    "molten lava",
    "photons",
    "escape pod",
];

fn send_command(droid: &mut IntcodeComputer, command: &str) -> String {
    for c in command.chars() {
        droid.input.push_back(c as isize);
    }
    droid.input.push_back('\n' as isize);

    let mut output = String::new();
    loop {
        match droid.run() {
            RunState::Halted => break,
            RunState::ProducedOutput => {
                output.push(droid.output.pop_front().unwrap() as u8 as char);
            }
            RunState::AwaitingInput => break,
        }
    }
    output
}

fn parse_room(output: &str) -> (String, Vec<String>, Vec<String>) {
    let mut room_name = String::new();
    let mut doors = Vec::new();
    let mut items = Vec::new();

    let mut in_doors = false;
    let mut in_items = false;

    for line in output.lines() {
        if line.starts_with("== ") && line.ends_with(" ==") {
            room_name = line[3..line.len() - 3].to_string();
            in_doors = false;
            in_items = false;
        } else if line == "Doors here lead:" {
            in_doors = true;
            in_items = false;
        } else if line == "Items here:" {
            in_doors = false;
            in_items = true;
        } else if let Some(stripped) = line.strip_prefix("- ") {
            let item = stripped.to_string();
            if in_doors {
                doors.push(item);
            } else if in_items {
                items.push(item);
            }
        } else if line.is_empty() {
            in_doors = false;
            in_items = false;
        }
    }

    (room_name, doors, items)
}

fn opposite_dir(dir: &str) -> &str {
    match dir {
        "north" => "south",
        "south" => "north",
        "east" => "west",
        "west" => "east",
        _ => panic!("Unknown direction"),
    }
}
