use anyhow::Result;
use std::io;

fn main() -> Result<()> {
    let mut text = String::new();
    io::stdin().read_line(&mut text)?;

    println!("Part 1: {}", decompress_len(&text, false));
    println!("Part 2: {}", decompress_len(&text, true));

    Ok(())
}

fn decompress_len(text: &str, recurse: bool) -> usize {
    let mut decompressed_len = 0;
    let mut iter = text.chars().filter(|c| !c.is_whitespace());

    while let Some(c) = iter.next() {
        // Try to parse this as a (AxB) marker sequence
        if c == '(' {
            let mut try_marker = String::new();
            let mut seen_close = false;
            for c in iter.by_ref() {
                if c == ')' {
                    seen_close = true;
                    break;
                }
                try_marker.push(c);
            }
            // We never encountered a ')': consider these normal characters
            if !seen_close {
                decompressed_len += try_marker.chars().count();
                continue;
            }

            let seq_tokens = try_marker.split('x').collect::<Vec<_>>();
            if let [left, right] = seq_tokens.as_slice() {
                match (left.parse::<usize>(), right.parse::<usize>()) {
                    (Ok(num_char), Ok(repeat)) => {
                        let mut to_repeat = String::new();
                        // We always have enough characters to repeat
                        for _ in 0..num_char {
                            to_repeat.push(iter.next().unwrap());
                        }
                        // Decompress length will be the same if repeated - so just compute once
                        let len = if recurse {
                            decompress_len(&to_repeat, true)
                        } else {
                            to_repeat.chars().count()
                        };
                        decompressed_len += len * repeat;
                    }
                    _ => {
                        decompressed_len += try_marker.chars().count();
                    }
                }
            }
        } else {
            decompressed_len += 1;
        }
    }

    decompressed_len
}
