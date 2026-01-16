use anyhow::{Result, anyhow};
use std::collections::HashMap;
use std::io::{BufRead, stdin};

fn main() -> Result<()> {
    let mut registers = Registers::default();
    let mut max = isize::MIN;
    for line in stdin().lock().lines() {
        let line = line?;
        let tokens: Vec<_> = line.split_ascii_whitespace().collect();
        let cond_reg = registers.get(tokens[4]);
        let target_register = registers.get_mut(tokens[0]);
        let op = tokens[1];
        let adj_value: isize = tokens[2].parse()?;
        let cond = tokens[5];
        let cond_val: isize = tokens[6].parse()?;

        let matches = match cond {
            ">" => cond_reg > cond_val,
            ">=" => cond_reg >= cond_val,
            "<" => cond_reg < cond_val,
            "<=" => cond_reg <= cond_val,
            "==" => cond_reg == cond_val,
            "!=" => cond_reg != cond_val,
            _ => return Err(anyhow!("Unrecognized condition: {cond}")),
        };
        if matches {
            match op {
                "inc" => *target_register += adj_value,
                "dec" => *target_register -= adj_value,
                _ => return Err(anyhow!("Unrecognized op: {op}")),
            }
        }
        max = max.max(registers.largest_val());
    }
    println!("Part 1: {}", registers.largest_val());
    println!("Part 2: {max}");
    Ok(())
}

#[derive(Debug, Default)]
struct Registers {
    registers: HashMap<String, isize>,
}

impl Registers {
    fn get_mut(&mut self, key: &str) -> &mut isize {
        if !self.registers.contains_key(key) {
            self.registers.insert(key.to_string(), 0);
        }
        self.registers.get_mut(key).unwrap()
    }

    fn get(&mut self, key: &str) -> isize {
        if !self.registers.contains_key(key) {
            self.registers.insert(key.to_string(), 0);
        }
        self.registers[key]
    }

    fn largest_val(&self) -> isize {
        *self.registers.values().max().unwrap()
    }
}
