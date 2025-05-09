use anyhow::Result;
use serde_json::Value;

fn main() -> Result<()> {
    let mut document = String::new();
    std::io::stdin().read_line(&mut document)?;

    let value: Value = serde_json::from_str(&document)?;
    println!("Part 1: {}", count(&value, false));
    println!("Part 2: {}", count(&value, true));

    Ok(())
}

fn count(value: &Value, filter_red: bool) -> i64 {
    match value {
        Value::Number(n) => n.as_i64().unwrap_or(0),
        Value::Array(a) => a.iter().fold(0, |acc, v| acc + count(v, filter_red)),
        Value::Object(o) => {
            if !filter_red || !o.values().any(|v| v.as_str() == Some("red")) {
                o.iter().fold(0, |acc, (_k, v)| acc + count(v, filter_red))
            } else {
                0
            }
        }
        _ => 0,
    }
}
