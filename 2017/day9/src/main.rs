use anyhow::Result;
use std::io::stdin;

fn main() -> Result<()> {
    let stream = parse_stream()?;
    let (score, garbage_chars) = score(&stream);
    println!("Part 1: {score}");
    println!("Part 2: {garbage_chars}");
    Ok(())
}

fn parse_stream() -> Result<Vec<char>> {
    let mut line = String::new();
    stdin().read_line(&mut line)?;
    Ok(line.trim().chars().collect())
}

fn score(stream: &[char]) -> (usize, usize) {
    let mut score = 0;
    let mut garbage_chars = 0;
    let mut groups = 0;
    let mut idx = 0;
    let mut in_garbage = false;
    while idx < stream.len() {
        match stream[idx] {
            '{' if !in_garbage => groups += 1,
            '}' if !in_garbage => {
                score += groups;
                groups -= 1;
            }
            '!' => idx += 1,
            '<' if !in_garbage => in_garbage = true,
            '>' if in_garbage => in_garbage = false,
            _ if in_garbage => garbage_chars += 1,
            _ => {}
        }
        idx += 1;
    }
    (score, garbage_chars)
}
