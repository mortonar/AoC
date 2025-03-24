use anyhow::{Result, anyhow};
use std::collections::HashSet;
use std::io::BufRead;

fn main() -> Result<()> {
    let mut lines = std::io::stdin().lock().lines();

    loop {
        let init_line = lines.next().transpose()?.expect("Expected a line");
        if init_line.trim().is_empty() {
            break;
        }
    }

    let mut highest_z = "z00".to_string();
    let mut connections = HashSet::new();
    while let Some(gate_line) = lines.next() {
        let gate_line = gate_line?;
        let gate_parts: Vec<_> = gate_line.split(" -> ").collect();

        let inputs: Vec<_> = gate_parts[0].split_ascii_whitespace().collect();
        let out_wire = gate_parts[1];
        if out_wire.starts_with("z") && out_wire > &highest_z {
            highest_z = out_wire.to_string();
        }
        connections.insert(Gate::from(&inputs, out_wire)?);
    }

    // Check all gates are valid gates in a ripple-carry adder. See:
    // * https://en.wikipedia.org/wiki/Adder_(electronics)#Ripple-carry_adder
    // * https://en.wikipedia.org/wiki/File:Halfadder.gif
    // * https://en.wikipedia.org/wiki/File:Fulladder.gif
    // https://www.reddit.com/r/adventofcode/comments/1hl698z/comment/m3kt1je/?utm_source=share&utm_medium=web3x&utm_name=web3xcss&utm_term=1&utm_content=share_button
    let mut wrong = HashSet::new();
    let params = vec!['x', 'y', 'z'];
    for conn in connections.iter() {
        if conn.out_wire.starts_with("z")
            && conn.operator != Operator::XOR
            && conn.out_wire != highest_z
        {
            wrong.insert(conn.out_wire.clone());
        } else if conn.operator == Operator::XOR
            && !params.contains(&conn.out_wire.chars().next().unwrap())
            && !params.contains(&conn.in_wire1.chars().next().unwrap())
            && !params.contains(&conn.in_wire2.chars().next().unwrap())
        {
            wrong.insert(conn.out_wire.clone());
        } else if conn.operator == Operator::AND && conn.in_wire1 != "x00" && conn.in_wire2 != "x00"
        {
            for sub_conn in connections.iter() {
                if (conn.out_wire == sub_conn.in_wire1 || conn.out_wire == sub_conn.in_wire2)
                    && sub_conn.operator != Operator::OR
                {
                    wrong.insert(conn.out_wire.clone());
                }
            }
        } else if conn.operator == Operator::XOR {
            for sub_conn in connections.iter() {
                if (conn.out_wire == sub_conn.in_wire1 || conn.out_wire == sub_conn.in_wire2)
                    && sub_conn.operator == Operator::OR
                {
                    wrong.insert(conn.out_wire.clone());
                }
            }
        }
    }

    let mut sorted: Vec<String> = wrong.into_iter().collect();
    sorted.sort();
    let sorted: String = sorted.join(",");
    println!("{sorted}");

    Ok(())
}

#[derive(Debug, Eq, Hash, PartialEq)]
struct Gate {
    in_wire1: String,
    in_wire2: String,
    out_wire: String,
    operator: Operator,
}

impl Gate {
    fn from(input_parts: &[&str], out_wire: &str) -> Result<Self> {
        if input_parts.len() != 3 {
            return Err(anyhow!("Expected three parts"));
        }
        let (in_wire1, in_wire2) = (input_parts[0].to_string(), input_parts[2].to_string());
        let operator = match input_parts[1] {
            "AND" => Operator::AND,
            "OR" => Operator::OR,
            "XOR" => Operator::XOR,
            _ => return Err(anyhow!("Unknown gate: {}", input_parts[1])),
        };
        let out_wire = out_wire.to_string();
        Ok(Self {
            in_wire1,
            in_wire2,
            out_wire,
            operator,
        })
    }
}

#[derive(Debug, Eq, Hash, PartialEq)]
enum Operator {
    AND,
    OR,
    XOR,
}
