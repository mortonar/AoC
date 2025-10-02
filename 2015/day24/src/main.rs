use anyhow::Result;
use std::cmp::Ordering;
use std::io;
use std::io::BufRead;

fn main() -> Result<()> {
    let mut packages: Group = Group::default();
    for line in io::stdin().lock().lines() {
        let line = line?;
        packages.push(line.trim().parse()?);
    }

    let context = SearchContext {
        configuration: Configuration::default(),
        to_place: packages.clone(),
        placing_num: 0,
        target_weight: packages.iter().sum::<usize>() / 3,
    };
    let search_context = ideal_config_dfs(context).unwrap();
    dbg!(&search_context);
    println!("Part 1: {}", search_context.first_quantam_entanglement());

    Ok(())
}

// Three groups with exactly the same weight (first is the passenger compartment)
//   Breaking ties:
//     * As few packages as possible in the passenger compartment
//     * Smallest quantum entanglement number for the passenger compartment
fn ideal_config_dfs(search_context: SearchContext) -> Option<SearchContext> {
    if search_context.all_placed() {
        return if search_context.equal_weight() {
            Some(search_context)
        } else {
            None
        };
    }

    let mut best_context = None;
    for context in search_context.neighbor_contexts(&best_context) {
        if let Some(context) = ideal_config_dfs(context) {
            if best_context.is_none() {
                best_context = Some(context);
            } else {
                let best = best_context.take().unwrap();
                if context < best {
                    best_context = Some(context);
                } else {
                    best_context = Some(best);
                }
            }
        }
    }
    best_context
}

type Package = usize;

type Group = Vec<Package>;

#[derive(Clone, Debug)]
struct SearchContext {
    configuration: Configuration,
    to_place: Vec<Package>,
    placing_num: usize,
    target_weight: usize,
}

impl SearchContext {
    fn all_placed(&self) -> bool {
        self.configuration.num_placed() == self.to_place.len()
    }

    fn equal_weight(&self) -> bool {
        self.configuration.equal_weight(self.target_weight)
    }

    fn neighbor_contexts(&self, current_best: &Option<SearchContext>) -> Vec<SearchContext> {
        let mut contexts = Vec::new();

        let placing = self.placing_num;
        let package = self.to_place[placing];
        for (i, group) in self.configuration.groups.iter().enumerate() {
            // Ensure we're not placing more packages into the first group than our current best has
            match current_best {
                Some(cb) if i == 0 => {
                    if cb.configuration.groups[0].len() < group.len() + 1 {
                        continue;
                    }
                }
                _ => {}
            }
            // Ensure placing this package won't put us over target weight for a group
            if (group.iter().sum::<usize>() + package) <= self.target_weight {
                let mut context = self.clone();
                context.configuration.groups[i].push(package);
                context.placing_num = placing + 1;
                contexts.push(context);
            }
        }

        contexts
    }

    fn first_group_size(&self) -> usize {
        self.configuration.groups[0].len()
    }

    fn first_quantam_entanglement(&self) -> u128 {
        self.configuration.groups[0]
            .iter()
            .map(|p| *p as u128)
            .product()
    }
}

impl Eq for SearchContext {}

impl PartialEq<Self> for SearchContext {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl PartialOrd<Self> for SearchContext {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for SearchContext {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.first_group_size() < other.first_group_size() {
            Ordering::Less
        } else if self.first_group_size() > other.first_group_size() {
            Ordering::Greater
        } else if self.first_quantam_entanglement() < other.first_quantam_entanglement() {
            Ordering::Less
        } else if self.first_quantam_entanglement() > other.first_quantam_entanglement() {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }
}

#[derive(Clone, Default, Debug)]
struct Configuration {
    groups: [Group; 3],
}

impl Configuration {
    fn num_placed(&self) -> usize {
        self.groups.iter().map(|g| g.len()).sum()
    }

    fn equal_weight(&self, target: usize) -> bool {
        self.groups
            .iter()
            .all(|g| g.iter().sum::<usize>() == target)
    }
}
