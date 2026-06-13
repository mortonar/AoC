use anyhow::{Result, bail};
use std::collections::HashMap;
use std::io::stdin;

fn main() -> Result<()> {
    let expense_report = parse_input()?;

    let pair = expense_report.sum_k(2, 2020)?;
    println!("Part 1: {}", pair.iter().product::<isize>());

    let triplet = expense_report.sum_k(3, 2020)?;
    println!("Part 2: {}", triplet.iter().product::<isize>());

    Ok(())
}

fn parse_input() -> Result<Vec<isize>> {
    stdin().lines().map(|l| Ok(l?.parse()?)).collect()
}

trait ExpenseReport {
    fn sum_k(&self, k: usize, target: isize) -> Result<Vec<isize>>;
}

impl ExpenseReport for Vec<isize> {
    fn sum_k(&self, k: usize, target: isize) -> Result<Vec<isize>> {
        if k == 0 {
            if target == 0 {
                return Ok(vec![]);
            }
            bail!("No combination of 0 numbers sums to {target}");
        }

        if self.len() < k {
            bail!("Need at least {k} numbers to check")
        }

        // maps j -> a concrete combination of j entries that produces particular sums
        // ----
        // ex.
        //   Given [10, 20, 30, 40, 50],
        //   after processing 20:
        // sums[0] = { 0: [] }
        // sums[1] = { 10: [10], 20: [20] }
        // sums[2] = { 30: [10, 20] }     ← 10+20=30, first 2-number sum!
        // sums[3] = {}
        //   after processing 30:
        // sums[0] = { 0: [] }
        // sums[1] = { 10: [10], 20: [20], 30: [30] }
        // sums[2] = { 30: [10, 20], 40: [10, 30], 50: [20, 30] }
        // sums[3] = { 60: [10, 20, 30] }
        let mut sums: Vec<HashMap<isize, Vec<isize>>> = vec![HashMap::new(); k + 1];
        sums[0].insert(0, vec![]);

        for &entry in self.iter() {
            // Process layers in reverse so we don't use the same entry twice
            for j in (1..=k).rev() {
                let prev: Vec<(isize, Vec<isize>)> = sums[j - 1]
                    .iter()
                    .map(|(&sum, entries)| (sum, entries.clone()))
                    .collect();

                for (sum, entries) in prev {
                    let new_sum = sum + entry;
                    sums[j].entry(new_sum).or_insert_with(|| {
                        let mut new_comb = entries;
                        new_comb.push(entry);
                        new_comb
                    });
                }
            }

            if let Some(k_set) = sums[k].get(&target) {
                return Ok(k_set.clone());
            }
        }

        bail!("No combination of {k} numbers sums to {target}")
    }
}
