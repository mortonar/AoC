use anyhow::Result;
use std::collections::HashMap;
use std::io::BufRead;

fn main() -> Result<()> {
    let mut frequencies: HashMap<usize, HashMap<char, usize>> = HashMap::new();
    for line in std::io::stdin().lock().lines() {
        line?.trim().chars().enumerate().for_each(|(i, c)| {
            frequencies
                .entry(i)
                .or_default()
                .entry(c)
                .and_modify(|e| *e += 1)
                .or_insert(1);
        })
    }

    let err_corrected = error_correct(&frequencies, true);
    println!("Part 1 {}", err_corrected);
    let err_corrected = error_correct(&frequencies, false);
    println!("Part 2 {}", err_corrected);

    Ok(())
}

fn error_correct(frequencies: &HashMap<usize, HashMap<char, usize>>, most_common: bool) -> String {
    (0..frequencies.keys().count())
        .map(|col| {
            *frequencies
                .get(&col)
                .unwrap()
                .iter()
                .max_by(|(_k1, v1), (_k2, v2)| if most_common { v1.cmp(v2) } else { v2.cmp(v1) })
                .unwrap()
                .0
        })
        .collect()
}
