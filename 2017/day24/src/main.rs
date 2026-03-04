use anyhow::Result;
use std::io::stdin;

fn main() -> Result<()> {
    let components = parse_input()?;

    let (strength, _length) = components.as_slice().strongest_bridge_dfs(0, vec![], false);
    println!("Part 1: {strength}");

    let (strength, _length) = components.as_slice().strongest_bridge_dfs(0, vec![], true);
    println!("Part 2: {strength}");

    Ok(())
}

fn parse_input() -> Result<Vec<Component>> {
    let mut components = Vec::new();
    for line in stdin().lines() {
        let line = line?;
        let tokens: Vec<_> = line.trim().split("/").collect();
        components.push(Component::new((tokens[0].parse()?, tokens[1].parse()?)));
    }
    Ok(components)
}

#[derive(Debug)]
struct Component {
    port1: usize,
    port2: usize,
}

impl Component {
    fn new(ports: (usize, usize)) -> Self {
        Self {
            port1: ports.0,
            port2: ports.1,
        }
    }

    /// If the given pins matches a port, returns the # of pins on the other port - otherwise None.
    fn match_end(&self, pins: usize) -> Option<usize> {
        if self.port1 == pins {
            Some(self.port2)
        } else if self.port2 == pins {
            Some(self.port1)
        } else {
            None
        }
    }

    fn strength(&self) -> usize {
        self.port1 + self.port2
    }
}

trait StrongestBridge {
    fn strongest_bridge_dfs(
        &self,
        current_end: usize,
        chosen: Vec<usize>,
        pick_longest: bool,
    ) -> (usize, usize);
}

impl StrongestBridge for &[Component] {
    fn strongest_bridge_dfs(
        &self,
        current_end: usize,
        chosen: Vec<usize>,
        pick_longest: bool,
    ) -> (usize, usize) {
        let can_choose: Vec<_> = self
            .iter()
            .enumerate()
            .filter_map(|(i, c)| {
                if !chosen.contains(&i)
                    && let Some(new_end) = c.match_end(current_end)
                {
                    Some((i, new_end))
                } else {
                    None
                }
            })
            .collect();

        if can_choose.is_empty() {
            return (
                chosen.iter().map(|&i| self[i].strength()).sum(),
                chosen.len(),
            );
        }

        can_choose
            .iter()
            .map(|(i, new_end)| {
                let mut chosen = chosen.clone();
                chosen.push(*i);
                self.strongest_bridge_dfs(*new_end, chosen, pick_longest)
            })
            .max_by(|(s1, l1), (s2, l2)| {
                if pick_longest {
                    l1.cmp(l2).then_with(|| s1.cmp(s2))
                } else {
                    s1.cmp(s2)
                }
            })
            .unwrap()
    }
}
