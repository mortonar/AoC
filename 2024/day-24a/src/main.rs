use anyhow::{anyhow, Result};
use std::collections::HashSet;
use std::io::BufRead;

fn main() -> Result<()> {
    let mut lines = std::io::stdin().lock().lines();

    let mut known_wires = HashSet::new();
    loop {
        let init_line = lines.next().transpose()?.expect("Expected a line");
        if init_line.trim().is_empty() {
            break;
        }

        let parts: Vec<_> = init_line.split(": ").collect();
        known_wires.insert(Wire::new(parts[0].to_string(), parts[1].parse()?));
    }
    let mut known_wires = KnownWires::new(known_wires);

    let mut connections = HashSet::new();
    while let Some(gate_line) = lines.next() {
        let gate_line = gate_line?;
        let gate_parts: Vec<_> = gate_line.split(" -> ").collect();

        let inputs: Vec<_> = gate_parts[0].split_ascii_whitespace().collect();
        let out_wire = gate_parts[1];
        connections.insert(Gate::from(&inputs, out_wire)?);
    }

    while !connections.is_empty() {
        let mut to_solve: HashSet<_> = connections
            .extract_if(|c| known_wires.contains(&c.in_wire1) && known_wires.contains(&c.in_wire2))
            .collect();
        if to_solve.is_empty() {
            panic!("No solution found");
        }
        for gate in to_solve.into_iter() {
            let in1 = known_wires.get(&gate.in_wire1).unwrap().value;
            let in2 = known_wires.get(&gate.in_wire2).unwrap().value;
            let output = gate.run(in1, in2);
            known_wires.insert(output);
        }
    }

    let mut zvals: Vec<_> = known_wires
        .iter()
        .filter(|w| w.label.starts_with('z'))
        .map(|w| (w.label.clone(), w.value))
        .collect();
    zvals.sort_by(|a, b| b.0.cmp(&a.0));
    let bit_string: String = zvals.iter().map(|(l, v)| v.to_string()).collect();
    let num = usize::from_str_radix(&bit_string, 2)?;
    println!("{num}");

    Ok(())
}

#[derive(Debug, Eq, Hash, PartialEq)]
struct Wire {
    label: String,
    value: u8,
}

impl Wire {
    fn new(label: String, value: u8) -> Self {
        Self { label, value }
    }
}

#[derive(Debug)]
struct KnownWires {
    wires: HashSet<Wire>,
}

impl KnownWires {
    fn new(known_wires: HashSet<Wire>) -> Self {
        Self { wires: known_wires }
    }

    fn contains(&self, wire_label: &str) -> bool {
        self.wires.iter().any(|w| w.label == wire_label)
    }

    fn get(&self, wire: &str) -> Option<&Wire> {
        self.wires.iter().find(|w| w.label == wire)
    }

    fn insert(&mut self, wire: Wire) {
        self.wires.insert(wire);
    }

    fn iter(&self) -> impl Iterator<Item = &Wire> {
        self.wires.iter()
    }
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

    fn run(&self, in1: u8, in2: u8) -> Wire {
        let val = match self.operator {
            Operator::AND => in1 & in2,
            Operator::OR => in1 | in2,
            Operator::XOR => in1 ^ in2,
        };
        Wire::new(self.out_wire.clone(), val)
    }
}

#[derive(Debug, Eq, Hash, PartialEq)]
enum Operator {
    AND,
    OR,
    XOR,
}
