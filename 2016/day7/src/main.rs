use anyhow::Result;
use std::collections::HashSet;
use std::io;
use std::io::BufRead;

// This is kinda messy, but it made the most sense in my head on a first pass. Essentially the idea
// is to keep a sliding window of 4 character sequences and track of whether the window is inside or
// outside a hypernet by keeping a stack of '[' to open/close. The actual input is nice because I
// don't see any nested '[' or ']' - so maybe this was overly complicated. Of those sequences, two
// "subsequences" of 3 characters are also processed. These end up being processed twice but that
// doesn't matter for what part 2 is asking.
fn main() -> Result<()> {
    let mut support_tls_count = 0;
    let is_abba =
        |seq: &[char]| seq.len() == 4 && seq[0] == seq[3] && seq[1] == seq[2] && seq[0] != seq[1];

    let mut support_ssl_count = 0;
    let is_aba = |seq: &[char]| seq.len() == 3 && seq[0] == seq[2] && seq[1] != seq[0];
    let to_bab = |seq: &[char]| [seq[1], seq[0], seq[1]];

    for line in io::stdin().lock().lines() {
        let ip = line?;

        let mut hypernet_stack = Vec::new();

        let mut abba_in_hyper = false;
        let mut abba_outside_hyper = false;

        let mut abas_in_hyper: HashSet<[char; 3]> = HashSet::new();
        let mut abas_outside: HashSet<[char; 3]> = HashSet::new();
        let mut supports_ssl = false;

        for (i, c) in ip.trim().chars().enumerate() {
            let in_hypernet = !hypernet_stack.is_empty();

            if i > 2 {
                let abba_seq: [char; 4] = ip[(i - 3)..=i]
                    .chars()
                    .collect::<Vec<char>>()
                    .try_into()
                    .unwrap();
                let aba1 = abba_seq.first_chunk::<3>().unwrap().as_slice();
                let aba2 = abba_seq.last_chunk::<3>().unwrap().as_slice();
                let sequences = [abba_seq.as_slice(), aba1, aba2];
                for seq in sequences {
                    if seq.contains(&'[') && seq.contains(&']') {
                        continue;
                    }

                    if is_abba(seq) {
                        if !in_hypernet {
                            abba_in_hyper = true;
                        } else {
                            abba_outside_hyper = true;
                        }
                    }

                    if is_aba(seq) {
                        let bab = to_bab(seq);
                        if !in_hypernet {
                            abas_outside.insert(seq.try_into()?);
                            if abas_in_hyper.contains(bab.as_slice()) {
                                supports_ssl = true;
                            }
                        } else {
                            abas_in_hyper.insert(seq.try_into()?);
                            if abas_outside.contains(bab.as_slice()) {
                                supports_ssl = true;
                            }
                        }
                    }
                }
            }

            if c == '[' {
                hypernet_stack.push(i);
            } else if c == ']' {
                hypernet_stack.pop();
            }
        }

        if abba_in_hyper && !abba_outside_hyper {
            support_tls_count += 1;
        }
        if supports_ssl {
            support_ssl_count += 1;
        }
    }
    println!("Part 1: {}", support_tls_count);
    println!("Part 2: {}", support_ssl_count);
    Ok(())
}
