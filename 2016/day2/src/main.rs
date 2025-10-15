use anyhow::Result;
use std::io;
use std::io::BufRead;

fn main() -> Result<()> {
    let mut keypads = [
        Keypad::default(),
        Keypad {
            layout: Layout::Diamond,
            ..Keypad::default()
        },
    ];
    for line in io::stdin().lock().lines() {
        let line = line?;
        for c in line.trim().chars() {
            keypads.iter_mut().for_each(|k| k.advance(&c));
        }
        for k in keypads.iter_mut() {
            k.add_to_code()?;
        }
    }
    println!("Part 1: {}", &keypads[0].code);
    println!("Part 2: {}", &keypads[1].code);
    Ok(())
}

enum Layout {
    // 1 2 3
    // 4 5 6
    // 7 8 9
    Square,
    //     1
    //   2 3 4
    // 5 6 7 8 9
    //   A B C
    //     D
    Diamond,
}

struct Keypad {
    value: u8,
    code: String,
    layout: Layout,
}

impl Default for Keypad {
    fn default() -> Self {
        Self {
            value: 5,
            code: String::new(),
            layout: Layout::Square,
        }
    }
}

impl Keypad {
    fn advance(&mut self, c: &char) {
        match self.layout {
            Layout::Square => match c {
                'U' => {
                    if self.value > 3 {
                        self.value -= 3;
                    }
                }
                'D' => {
                    if self.value < 7 {
                        self.value += 3;
                    }
                }
                'L' => {
                    if ![1, 4, 7].contains(&self.value) {
                        self.value -= 1;
                    }
                }
                'R' => {
                    if ![3, 6, 9].contains(&self.value) {
                        self.value += 1;
                    }
                }
                _ => {}
            },
            Layout::Diamond => match c {
                'U' => {
                    if self.value == 3 {
                        self.value = 1;
                    } else if [6, 7, 8].contains(&self.value) {
                        self.value -= 4;
                    } else if [65, 66, 67].contains(&self.value) {
                        self.value = self.value - 65 + 6;
                    } else if self.value == 68 {
                        self.value = 66;
                    }
                }
                'D' => {
                    if self.value == 1 {
                        self.value = 3;
                    } else if [2, 3, 4].contains(&self.value) {
                        self.value += 4;
                    } else if [6, 7, 8].contains(&self.value) {
                        self.value = self.value - 6 + 65;
                    } else if self.value == 66 {
                        self.value = 68;
                    }
                }
                'L' => {
                    if ![1, 2, 5, 65, 68].contains(&self.value) {
                        self.value -= 1;
                    }
                }
                'R' => {
                    if ![1, 4, 9, 67, 68].contains(&self.value) {
                        self.value += 1;
                    }
                }
                _ => {}
            },
        }
    }

    fn add_to_code(&mut self) -> Result<()> {
        if self.value >= 65 {
            self.code.push(self.value as char);
        } else {
            self.code.push(
                char::from_digit(self.value as u32, 10).ok_or(anyhow::anyhow!("invalid input"))?,
            );
        }
        Ok(())
    }
}
