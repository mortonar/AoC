use anyhow::{Result, anyhow};
use std::collections::HashMap;
use std::io::{BufRead, stdin};
use std::mem;

fn main() -> Result<()> {
    let mut graph = parse_input()?;
    let bottom_prog = graph
        .find_root()
        .ok_or(anyhow!("Root node not found"))?
        .clone();
    println!("Part 1: {bottom_prog}");
    rebalance_dfs(&mut graph, &bottom_prog);
    Ok(())
}

#[derive(Debug)]
struct Graph {
    programs: HashMap<String, usize>,
    holding: HashMap<String, Vec<String>>,
}

impl Graph {
    fn find_root(&self) -> Option<&String> {
        self.programs
            .keys()
            .find(|p| self.holding.values().all(|held| !held.contains(p)))
    }
}

fn rebalance_dfs(graph: &mut Graph, program: &str) -> usize {
    if !graph.holding.contains_key(program) {
        return graph.programs[program];
    }

    // Calculate map of weight -> held child programs at that weight. One map entry implies balance.
    // If unbalanced/off, adjust the weight of the off balanced program (map entry with one child).
    let mut weights_to_programs = HashMap::new();
    let children = graph.holding[program].clone();
    for child in &children {
        weights_to_programs
            .entry(rebalance_dfs(graph, child))
            .and_modify(|programs: &mut Vec<&String>| programs.push(child))
            .or_insert(vec![child]);
    }
    if weights_to_programs.keys().count() == 1 {
        let (&weight, programs) = weights_to_programs.iter().next().unwrap();
        weight * programs.len() + graph.programs[program]
    } else {
        let mut weights = weights_to_programs.iter().take(2);
        let mut common = weights.next().unwrap();
        let mut off = weights.next().unwrap();
        if off.1.len() > common.1.len() {
            mem::swap(&mut common, &mut off);
        }
        let diff = common.0.abs_diff(*off.0);
        let program_weight = graph.programs.get_mut(off.1[0]).unwrap();
        if off.0 < common.0 {
            *program_weight += diff
        } else {
            *program_weight -= diff
        }
        println!("Part 2: {program_weight}");
        *common.0 * (common.1.len() + 1) + graph.programs[program]
    }
}

fn parse_input() -> Result<Graph> {
    let mut programs = HashMap::new();
    let mut holding = HashMap::new();
    for line in stdin().lock().lines() {
        let line = line?;

        let tokens: Vec<_> = line.split("->").collect();
        let program_tokens: Vec<_> = tokens[0].split_ascii_whitespace().collect();
        let holding_program = program_tokens[0];
        let weight = program_tokens[1];
        programs.insert(
            holding_program.to_string(),
            weight[1..weight.len() - 1].parse()?,
        );

        if tokens.len() > 1 {
            for held in tokens[1].split_ascii_whitespace() {
                let held = held.strip_suffix(",").unwrap_or(held);
                holding
                    .entry(holding_program.to_string())
                    .and_modify(|held_programs: &mut Vec<String>| {
                        held_programs.push(held.to_string())
                    })
                    .or_insert(vec![held.to_string()]);
            }
        }
    }
    Ok(Graph { programs, holding })
}
