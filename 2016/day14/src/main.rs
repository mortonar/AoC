use anyhow::Result;
use std::collections::HashMap;
use std::fmt::Write;
use std::io;
use std::io::BufRead;

fn main() -> Result<()> {
    let salt = io::stdin()
        .lock()
        .lines()
        .next()
        .ok_or(anyhow::anyhow!("Empty input"))??;

    let mut hasher = HexHasher::new(&salt);
    println!("Part 1: {}", find_target_otp(&mut hasher, false)?);
    println!("Part 2: {}", find_target_otp(&mut hasher, true)?);

    Ok(())
}

fn find_target_otp(hasher: &mut HexHasher, key_stretching: bool) -> Result<usize> {
    let mut ot_pads = 0;
    for index in 0.. {
        let hex_str = hasher.hash(index, key_stretching)?;
        if let Some(triple) = dups(&hex_str, 3).next() {
            let is_key = ((index + 1)..(index + 1000)).any(|next_index| {
                let next_hex = hasher.hash(next_index, key_stretching).unwrap();
                dups(&next_hex, 5).any(|quintet| quintet == triple)
            });
            if is_key {
                ot_pads += 1;
                if ot_pads == 64 {
                    return Ok(index);
                }
            }
        }
    }
    Err(anyhow::anyhow!("How did we even get here ANYHOW?"))
}

struct HexHasher {
    salt: String,
    buffer: String,
    memo: HashMap<(usize, bool), String>,
}

impl HexHasher {
    fn new(salt: &str) -> HexHasher {
        Self {
            salt: salt.to_string(),
            buffer: String::with_capacity(salt.chars().count() + 20),
            memo: HashMap::new(),
        }
    }

    fn hash(&mut self, index: usize, key_stretching: bool) -> Result<String> {
        if let Some(cached) = self.memo.get(&(index, key_stretching)) {
            return Ok(cached.clone());
        }

        self.buffer.clear();
        write!(&mut self.buffer, "{}{}", self.salt, index)?;
        let mut hash: [u8; 16] = md5::compute(self.buffer.as_bytes()).into();
        let mut hex = hex::encode(hash);

        if key_stretching {
            for _ in 0..2016 {
                hash = md5::compute(hex.as_bytes()).into();
                hex = hex::encode(hash);
            }
        }

        self.memo.insert((index, key_stretching), hex.clone());

        Ok(hex)
    }
}

fn dups(hex_str: &str, size: usize) -> impl Iterator<Item = char> {
    hex_str.as_bytes().windows(size).filter_map(|w| {
        if w.iter().all(|&c| c == w[0]) {
            Some(w[0] as char)
        } else {
            None
        }
    })
}
