use anyhow::Result;
use itertools::Itertools;
use std::io::{stdin, Read};

fn main() -> Result<()> {
    let mut disk = Vec::new();
    for (id, mut chunk) in stdin().bytes().chunks(2).into_iter().enumerate() {
        let file_size = chunk.next().unwrap()? - b'0';
        disk.push(Block {
            size: file_size,
            b_type: Type::Occ(id),
        });
        let free_size = chunk.next().unwrap_or(Ok(b'0'))? - b'0';
        disk.push(Block {
            size: free_size,
            b_type: Type::Free,
        });
    }
    defrag(&mut disk);

    let checksum: usize = disk
        .iter()
        .flat_map(Block::expand)
        .enumerate()
        .map(|(pos, id)| pos * id)
        .sum();
    println!("{}", checksum);

    Ok(())
}

fn defrag(disk: &mut Vec<Block>) {
    for from in (0..disk.len()).rev() {
        if let Type::Free = &disk[from].b_type {
            continue;
        }

        let mut move_b = false;
        for to in 0..from {
            {
                let f_block = &disk[from];
                let t_block = &disk[to];
                match t_block.b_type {
                    Type::Occ(_) => {}
                    Type::Free => {
                        if t_block.size >= f_block.size {
                            move_b = true;
                        }
                    }
                }
            }
            if move_b {
                move_block(disk, from, to);
                break;
            }
        }
    }
}

fn move_block(disk: &mut Vec<Block>, from: usize, to: usize) {
    let f_block = disk.remove(from);
    let mut t_block = disk.remove(to);

    t_block.size -= f_block.size;
    let free_size = f_block.size;
    disk.insert(to, f_block);
    disk.insert(
        from,
        Block {
            size: free_size,
            b_type: Type::Free,
        },
    );

    if t_block.size > 0 {
        disk.insert(to + 1, t_block);
    }
}

#[derive(Debug)]
struct Block {
    size: u8,
    b_type: Type,
}

impl Block {
    fn expand(&self) -> Expanded {
        Expanded {
            id: match self.b_type {
                Type::Occ(id) => id,
                Type::Free => 0,
            },
            size: self.size,
        }
    }
}

#[derive(Debug)]
enum Type {
    Occ(ID),
    Free,
}

type ID = usize;

#[derive(Debug)]
struct Expanded {
    id: ID,
    size: u8,
}

impl Iterator for Expanded {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.size != 0 {
            self.size -= 1;
            Some(self.id)
        } else {
            None
        }
    }
}
