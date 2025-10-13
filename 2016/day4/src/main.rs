use anyhow::Result;
use regex::Regex;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::io;
use std::io::BufRead;

fn main() -> Result<()> {
    let regex = Regex::new(r"([a-z-]+)([0-9].+)\[([a-z].+)\]")?;
    let mut sector_sum = 0;
    for line in io::stdin().lock().lines() {
        let line = line?;
        let caps = regex
            .captures(&line)
            .ok_or_else(|| anyhow::anyhow!("Invalid input"))?;
        let [_match, name, sector_id, checksum] = [&caps[0], &caps[1], &caps[2], &caps[3]];
        let sector_id: usize = sector_id.parse()?;

        if is_real(name, checksum) {
            sector_sum += sector_id;
        }

        if decrypt(&name, sector_id) == "northpole object storage " {
            println!("Part 2: {sector_id}");
        }
    }
    println!("Part 1: {sector_sum}");

    Ok(())
}

fn is_real(name: &str, checksum: &str) -> bool {
    let mut common: [usize; 26] = [0; 26];
    let base = 'a' as usize;
    for c in name.chars() {
        if c != '-' {
            common[c as usize - base] += 1;
        }
    }

    let mut sorted = BinaryHeap::with_capacity(common.len());
    let base = 'a' as u8;
    for i in 0..common.len() {
        sorted.push(Freq {
            c: (base + i as u8) as char,
            count: common[i],
        });
    }

    let mut calc_checksum = String::new();
    for _i in 0..5 {
        calc_checksum.push(sorted.pop().unwrap().c);
    }

    calc_checksum == checksum
}

struct Freq {
    c: char,
    count: usize,
}

impl Eq for Freq {}

impl PartialEq<Self> for Freq {
    fn eq(&self, other: &Self) -> bool {
        self.c == other.c && self.count == other.count
    }
}

impl PartialOrd<Self> for Freq {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Freq {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.count > other.count {
            Ordering::Greater
        } else if self.count < other.count {
            Ordering::Less
        } else {
            other.c.cmp(&self.c)
        }
    }
}

fn decrypt(name: &str, sector_id: usize) -> String {
    let mut decrypted = String::with_capacity(name.len());
    let base = 'a' as usize;
    for c in name.chars() {
        if c == '-' {
            decrypted.push(' ');
        } else {
            let rotated = ((c as usize - base) + sector_id) % 26 + base;
            decrypted.push(rotated as u8 as char);
        }
    }
    decrypted
}
