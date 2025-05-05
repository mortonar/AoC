use anyhow::Result;

fn main() -> Result<()> {
    let mut sequence = String::new();
    std::io::stdin().read_line(&mut sequence)?;
    let mut sequence = sequence.trim().to_string();
    if sequence.chars().any(|c| !c.is_digit(10)) {
        return Err(anyhow::anyhow!("Invalid input {sequence}"));
    }

    for _ in 0..40 {
        sequence = look_say(&sequence);
    }
    println!("Part 1: {}", sequence.len());
    for _ in 0..10 {
        sequence = look_say(&sequence);
    }
    println!("Part 2: {}", sequence.len());

    Ok(())
}

fn look_say(sequence: &str) -> String {
    let mut translated = String::new();
    let (mut begin, mut bc) = sequence.char_indices().nth(0).unwrap();
    for (end, ec) in sequence.char_indices() {
        if bc != ec {
            let prefix = (end - begin).to_string();
            translated.push_str(&prefix);
            translated.push(bc);
            (begin, bc) = (end, ec);
        }
        if end == sequence.len() - 1 {
            let prefix = (end - begin + 1).to_string();
            translated.push_str(&prefix);
            translated.push(bc);
        }
    }

    translated
}
