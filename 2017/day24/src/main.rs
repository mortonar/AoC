use anyhow::Result;
use std::collections::HashMap;
use std::io::stdin;

fn main() -> Result<()> {
    let components = parse_input()?;

    let strength = components.as_slice().strongest_bridge(0);
    println!("Part 1: {strength}");

    let strength = components.as_slice().longest_bridge(0);
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
    fn strongest_bridge(&self, starting_end: usize) -> usize;

    fn longest_bridge(&self, starting_end: usize) -> usize;

    fn strongest_bridge_dfs(
        &self,
        current_end: usize,
        chosen: Vec<usize>,
        pick_longest: bool,
        // (current_end, unchosen components) -> (best bridge from this state)
        memo: &mut HashMap<(usize, Vec<usize>), (usize, usize)>,
    ) -> (usize, usize);
}

impl StrongestBridge for &[Component] {
    fn strongest_bridge(&self, starting_end: usize) -> usize {
        let (strength, _length) =
            self.strongest_bridge_dfs(starting_end, vec![], false, &mut HashMap::new());
        strength
    }

    fn longest_bridge(&self, starting_end: usize) -> usize {
        let (strength, _length) =
            self.strongest_bridge_dfs(starting_end, vec![], true, &mut HashMap::new());
        strength
    }

    fn strongest_bridge_dfs(
        &self,
        current_end: usize,
        chosen: Vec<usize>,
        pick_longest: bool,
        memo: &mut HashMap<(usize, Vec<usize>), (usize, usize)>,
    ) -> (usize, usize) {
        let unchosen: Vec<_> = (0..self.len()).filter(|i| !chosen.contains(i)).collect();
        if let Some(cached) = memo.get(&(current_end, unchosen.clone())) {
            return *cached;
        }

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

        let best = can_choose
            .iter()
            .map(|(i, new_end)| {
                let mut chosen = chosen.clone();
                chosen.push(*i);
                self.strongest_bridge_dfs(*new_end, chosen, pick_longest, memo)
            })
            .max_by(|(s1, l1), (s2, l2)| {
                if pick_longest {
                    l1.cmp(l2).then_with(|| s1.cmp(s2))
                } else {
                    s1.cmp(s2)
                }
            })
            .unwrap();

        memo.insert((current_end, unchosen), best);

        best
    }
}
