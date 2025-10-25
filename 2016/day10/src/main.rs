use anyhow::Result;
use regex::Regex;
use std::collections::HashMap;
use std::env::args;
use std::io::BufRead;

fn main() -> Result<()> {
    let mut bots: HashMap<usize, Vec<usize>> = HashMap::new();
    let mut outputs: HashMap<usize, usize> = HashMap::new();
    let mut move_instructions: Vec<(usize, String, usize, String, usize)> = Vec::new();
    let val_regex = Regex::new(r"^value (\d+) goes to bot (\d+)$")?;
    let move_regex =
        Regex::new(r"^bot (\d+) gives low to (bot|output) (\d+) and high to (bot|output) (\d+)$")?;

    let search_chips = get_search_chips()?;

    for line in std::io::stdin().lock().lines() {
        let line = line?;
        if let Some((_full, [val, bot])) = val_regex.captures(&line).map(|c| c.extract()) {
            let (val, bot): (usize, usize) = (val.parse()?, bot.parse()?);
            bots.entry(bot)
                .and_modify(|vals| {
                    vals.push(val);
                    vals.sort();
                })
                .or_insert(vec![val]);
        } else if let Some((_full, [bot, low, low_val, high, high_val])) =
            move_regex.captures(&line).map(|c| c.extract())
        {
            move_instructions.push((
                bot.parse()?,
                low.to_string(),
                low_val.parse()?,
                high.to_string(),
                high_val.parse()?,
            ));
        } else {
            return Err(anyhow::anyhow!("Can't parse line: {}", line));
        }
    }

    while bots.values().any(|chips| chips.len() == 2) {
        for (bot_num, low_type, low_dest, high_type, high_dest) in &move_instructions {
            let bot = bots.entry(*bot_num).or_default();
            if bot.len() != 2 {
                continue;
            }

            if search_chips.iter().all(|chip| bot.contains(chip)) {
                println!("Part 1: {bot_num}");
            }

            let high_val = bot.pop().unwrap();
            let low_val = bot.pop().unwrap();
            if low_type == "bot" {
                let b = bots.entry(*low_dest).or_default();
                b.push(low_val);
                b.sort();
            } else {
                outputs.insert(*low_dest, low_val);
            };
            if high_type == "bot" {
                let b = bots.entry(*high_dest).or_default();
                b.push(high_val);
                b.sort();
            } else {
                outputs.insert(*high_dest, high_val);
            };
        }
    }

    println!(
        "Part 2: {}",
        (0..=2).map(|i| outputs[&i]).product::<usize>()
    );

    Ok(())
}

fn get_search_chips() -> Result<[usize; 2]> {
    let mut args = args()
        .skip(1)
        .take(2)
        .map(|a| a.parse::<usize>())
        .collect::<Result<Vec<_>, _>>()?;
    Ok([args.pop().unwrap_or(17), args.pop().unwrap_or(61)])
}
