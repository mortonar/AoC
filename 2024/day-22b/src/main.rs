use anyhow::Result;
use std::collections::HashMap;
use std::io::BufRead;

// This is a bit slow (~6s) but it gets the job done.
// * Process the price change sequence for each buyer and keep track of what the value is the first
//   time a sequence is encountered.
// * Accumulate the value in the map for all buyers for that given sequence.
// * The largest value in the shared map is the best sell.
fn main() -> Result<()> {
    let mut secrets: Vec<PriceChanges> = Vec::new();
    for line in std::io::stdin().lock().lines() {
        secrets.push(PriceChanges::new(line?.trim().parse()?));
    }

    // Map of price seq -> the highest sell
    let mut seq_map: HashMap<(i64, i64, i64, i64), u64> = HashMap::new();

    for secret in &mut secrets {
        // Keep a local max around.
        // After this buyer is fully processed, add its local max to the overall sequence map.
        let mut local_seq_map: HashMap<(i64, i64, i64, i64), u64> = HashMap::new();
        for _ in 0..(2000 - 4) {
            let sequence = secret.next().unwrap();
            let val = sequence[3].1;
            let sequence = (sequence[0].0, sequence[1].0, sequence[2].0, sequence[3].0);
            // Monkey sells the hiding spot the first time it sees the sequence
            if !local_seq_map.contains_key(&sequence) {
                local_seq_map.insert(sequence, val);
            }
        }
        local_seq_map.into_iter().for_each(|(seq, val)| {
            seq_map.entry(seq).and_modify(|v| *v += val).or_insert(val);
        });
    }

    let max = *seq_map.values().max().unwrap();
    println!("{max}");

    Ok(())
}

struct PriceChanges {
    prev: u64,
    value: u64,
    window: [(i64, u64); 4],
}

impl PriceChanges {
    fn new(init_value: u64) -> Self {
        let mut ret = Self {
            prev: init_value,
            value: Self::evolve(init_value),
            window: [(0, 0); 4],
        };
        (0..4).for_each(|_| ret.advance_seq());
        ret
    }

    fn evolve(value: u64) -> u64 {
        let mut value = value;
        value = Self::mix_prune(value, value * 64);
        value = Self::mix_prune(value, value / 32);
        Self::mix_prune(value, value * 2048)
    }

    fn advance_seq(&mut self) {
        for i in 0..self.window.len() - 1 {
            self.window[i] = self.window[i + 1];
        }
        self.window[3] = (
            (self.value % 10) as i64 - (self.prev % 10) as i64,
            self.value % 10,
        );

        self.prev = self.value;
        self.value = Self::evolve(self.value);
    }

    fn mix_prune(secret: u64, mix_with: u64) -> u64 {
        (secret ^ mix_with) % 16777216
    }
}

impl Iterator for PriceChanges {
    type Item = [(i64, u64); 4];

    fn next(&mut self) -> Option<Self::Item> {
        self.advance_seq();
        Some(self.window)
    }
}
