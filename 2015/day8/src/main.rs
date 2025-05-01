use anyhow::Result;
use regex::Regex;
use std::io::BufRead;

fn main() -> Result<()> {
    let mut literal_chars: usize = 0;
    let mut encode_chars: usize = 0;
    let mut mem_chars: usize = 0;

    let decode_re = Regex::new(r#"\\("|\\|x[0-9a-fA-F]{2})"#)?;
    let encode_re = Regex::new(r#"(["\\])"#)?;

    for line in std::io::stdin().lock().lines() {
        let line = line?;
        let trimmed = line.trim();

        literal_chars += trimmed.len();

        let mem_result = decode_re.replace_all(&trimmed, "x");
        // -2 for enclosing ""
        mem_chars += mem_result.len() - 2;

        let encoded_result = encode_re.replace_all(&trimmed, r#"\x"#);
        // +2 for enclosing ""
        encode_chars += encoded_result.len() + 2;
    }

    println!("Part 1: {}", literal_chars - mem_chars);
    println!("Part 2: {}", encode_chars - literal_chars);

    Ok(())
}
