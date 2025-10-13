use anyhow::Result;
use regex::Regex;
use std::cmp::Ordering;
use std::io;
use std::io::BufRead;

const BASE: usize = 'a' as usize;

#[derive(Copy, Clone, Debug, Default)]
struct Freq {
    c: char,
    count: usize,
}

fn main() -> Result<()> {
    let regex = Regex::new(r"([a-z-]+)([0-9].+)\[([a-z].+)\]")?;
    let mut sector_sum = 0;
    let mut sector_found = 0;
    for line in io::stdin().lock().lines() {
        let line = line?;
        let caps = regex
            .captures(&line)
            .ok_or_else(|| anyhow::anyhow!("Invalid input"))?;
        let [_match, name, sector_id, checksum] = [&caps[0], &caps[1], &caps[2], &caps[3]];
        let sector_id = sector_id.parse::<usize>()?;

        if is_real(name, checksum) {
            sector_sum += sector_id;
        }

        if decrypt(&name, sector_id) == "northpole object storage " {
            sector_found = sector_id;
        }
    }
    println!("Part 1: {sector_sum}");
    println!("Part 2: {sector_found}");

    Ok(())
}

fn is_real(name: &str, checksum: &str) -> bool {
    // common[i] = freq of i; i = offset from 'a'
    let mut common: [Freq; 26] = [Freq::default(); 26];
    for i in 0..26 {
        common[i].c = (i + BASE) as u8 as char;
    }

    for c in name.chars() {
        if c != '-' {
            common[c as usize - BASE].count += 1;
        }
    }

    common.sort_by(|a, b| match b.count.cmp(&a.count) {
        Ordering::Equal => a.c.cmp(&b.c),
        val => val,
    });

    let mut calc_checksum = String::new();
    for i in 0..5 {
        calc_checksum.push(common[i].c);
    }

    calc_checksum == checksum
}

fn decrypt(name: &str, sector_id: usize) -> String {
    let mut decrypted = String::with_capacity(name.len());
    for c in name.chars() {
        if c == '-' {
            decrypted.push(' ');
        } else {
            let rotated = ((c as usize - BASE) + sector_id) % 26 + BASE;
            decrypted.push(rotated as u8 as char);
        }
    }
    decrypted
}
