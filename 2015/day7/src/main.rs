use anyhow::{Error, Result};
use std::collections::HashMap;
use std::io::BufRead;
use std::ops::{BitAnd, BitOr};
use std::str::FromStr;

fn main() -> Result<()> {
    let mut to_solve: Vec<Gate> = Vec::new();

    for line in std::io::stdin().lock().lines() {
        let line = line?;
        let tokens: Vec<&str> = line.trim().split_whitespace().collect();
        if tokens.len() == 3 {
            to_solve.push(Gate::new(
                Operation::Assign(tokens[0].parse()?),
                tokens[2].to_string(),
            ));
        } else if tokens.len() == 4 {
            to_solve.push(Gate::new(
                Operation::Not(tokens[1].to_string()),
                tokens[3].to_string(),
            ));
        } else {
            let operation = match &tokens[1] {
                &"OR" => Operation::Or(tokens[0].parse()?, tokens[2].parse()?),
                &"AND" => Operation::And(tokens[0].parse()?, tokens[2].parse()?),
                &"LSHIFT" => Operation::LShift(tokens[0].to_string(), tokens[2].parse()?),
                &"RSHIFT" => Operation::RShift(tokens[0].to_string(), tokens[2].parse()?),
                _ => return Err(anyhow::anyhow!("Unknown operation {}", &tokens[1])),
            };
            to_solve.push(Gate::new(operation, tokens[4].to_string()));
        }
    }

    let mut circuit = Circuit::new(to_solve);
    circuit.solve();

    let a: Label = "a".to_string();
    if let Some(a_val) = circuit.get(&a) {
        println!("Part 1: {}", a_val);

        circuit.reset();
        let b = "b".to_string();
        circuit.set(&b, a_val);
        circuit.solve();
        if let Some(a_val) = circuit.get(&a) {
            println!("Part 2: {}", a_val);
        } else {
            println!("Part 2: 'a' not found");
        }
    } else {
        println!("Part 1: 'a' not found");
    }

    Ok(())
}

#[derive(Debug)]
struct Circuit {
    gates: Vec<Gate>,
    known: HashMap<Label, u16>,
}

impl Circuit {
    fn new(gates: Vec<Gate>) -> Self {
        Self {
            gates,
            known: HashMap::new(),
        }
    }

    fn solve(&mut self) {
        while !self.gates.iter().all(|g| g.solved) {
            // Retain only unsolved gates
            self.gates
                .iter_mut()
                .filter(|g| !g.solved)
                .for_each(|gate| {
                    if let Some(output) = gate.operation.solve(&self.known) {
                        self.known.insert(gate.output.clone(), output);
                        gate.solved = true;
                    }
                });
        }
    }

    fn get(&self, label: &Label) -> Option<u16> {
        self.known.get(label).cloned()
    }

    fn set(&mut self, label: &Label, value: u16) {
        for gate in &mut self.gates {
            if gate.output == *label {
                gate.solved = true;
                self.known.insert(label.clone(), value);
                break;
            }
        }
    }

    fn reset(&mut self) {
        self.gates.iter_mut().for_each(|gate| gate.solved = false);
        self.known = HashMap::new();
    }
}

#[derive(Debug)]
struct Gate {
    operation: Operation,
    output: Label,
    solved: bool,
}

type Label = String;

impl Gate {
    fn new(operation: Operation, output: Label) -> Self {
        Self {
            operation,
            output,
            solved: false,
        }
    }
}

#[derive(Debug)]
enum Operation {
    Assign(Operand),
    And(Operand, Operand),
    Or(Operand, Operand),
    LShift(Label, u16),
    RShift(Label, u16),
    Not(Label),
}

impl Operation {
    fn solve(&self, known: &HashMap<Label, u16>) -> Option<u16> {
        let needed = self.operands();
        if needed.is_empty() || needed.iter().all(|n| known.contains_key(*n)) {
            let output = match self {
                Operation::Assign(op) => op.value(&known).unwrap(),
                Operation::And(op1, op2) => op1.value(&known)?.bitand(op2.value(&known)?),
                Operation::Or(op1, op2) => op1.value(&known)?.bitor(op2.value(&known)?),
                Operation::LShift(l, ls) => known[l] << ls,
                Operation::RShift(l, rs) => known[l] >> rs,
                Operation::Not(l) => !known[l],
            };
            return Some(output);
        }
        None
    }

    fn operands(&self) -> Vec<&Label> {
        match self {
            Operation::Assign(op) => op.input_labels(),
            Operation::And(op1, op2) | Operation::Or(op1, op2) => {
                let mut v = op1.input_labels();
                v.extend(op2.input_labels());
                v
            }
            Operation::LShift(l, _) | Operation::RShift(l, _) => vec![l],
            Operation::Not(l) => vec![l],
        }
    }
}

#[derive(Debug)]
enum Operand {
    Label(Label),
    Value(u16),
}

impl Operand {
    fn input_labels(&self) -> Vec<&Label> {
        match self {
            Operand::Label(l) => vec![&l],
            Operand::Value(_) => vec![],
        }
    }

    fn value(&self, known: &HashMap<Label, u16>) -> Option<u16> {
        match self {
            Operand::Label(l) => known.get(l).map(|v| *v),
            Operand::Value(v) => Some(*v),
        }
    }
}

impl FromStr for Operand {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s.parse::<u16>() {
            Ok(val) => Ok(Self::Value(val)),
            Err(_) => Ok(Self::Label(s.to_string())),
        }
    }
}
