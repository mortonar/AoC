use anyhow::Result;
use std::io::BufRead;

static VOWELS: &'static [char] = &['a', 'e', 'i', 'o', 'u'];

fn main() -> Result<()> {
    let mut filters: Vec<Box<dyn NiceFilter>> =
        vec![Box::new(Part1::default()), Box::new(Part2::default())];
    for line in std::io::stdin().lock().lines() {
        let line = line?;
        filters.iter_mut().for_each(|f| f.filter(&line));
    }
    println!("Part 1 {}", filters[0].nice_count());
    println!("Part 2 {}", filters[1].nice_count());

    Ok(())
}

trait NiceFilter {
    fn filter(&mut self, string: &str);
    fn nice_count(&self) -> usize;
}

#[derive(Default)]
struct Part1 {
    nice_count: usize,
}

impl NiceFilter for Part1 {
    fn filter(&mut self, string: &str) {
        if string.chars().count() < 2 {
            return;
        }

        let mut vowel_count = 0;
        let mut double_letter = false;

        let mut prev = string.chars().next().unwrap();
        if VOWELS.contains(&prev) {
            vowel_count += 1;
        }
        for c in string.chars().skip(1) {
            match (prev, c) {
                ('a', 'b') | ('c', 'd') | ('p', 'q') | ('x', 'y') => return,
                (p, n) if p == n => double_letter = true,
                _ => {}
            }

            if VOWELS.contains(&c) {
                vowel_count += 1;
            }

            prev = c;
        }

        if vowel_count >= 3 && double_letter {
            self.nice_count += 1;
        }
    }

    fn nice_count(&self) -> usize {
        self.nice_count
    }
}

#[derive(Default)]
struct Part2 {
    nice_count: usize,
}

impl NiceFilter for Part2 {
    fn filter(&mut self, string: &str) {
        let chars: Vec<_> = string.chars().collect();
        if chars.len() < 4 {
            return;
        }

        let mut has_pair = false;
        for i in 0..chars.len() - 3 {
            for j in i + 2..chars.len() - 1 {
                if chars[i..=i + 1] == chars[j..=j + 1] {
                    has_pair = true;
                    break;
                }
            }
        }

        let is_nice = has_pair
            && chars
                .windows(3)
                .find(|chunk| chunk[0] == chunk[2])
                .is_some();
        if is_nice {
            self.nice_count += 1;
        }
    }

    fn nice_count(&self) -> usize {
        self.nice_count
    }
}
