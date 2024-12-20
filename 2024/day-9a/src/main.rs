use anyhow::Result;
use itertools::Itertools;
use std::io::{stdin, Read};
use std::iter;
use std::iter::RepeatN;

fn main() -> Result<()> {
    let mut disk_map = Vec::new();
    let mut total_size = 0;
    for (id, mut chunk) in stdin().bytes().chunks(2).into_iter().enumerate() {
        let file_size = chunk.next().unwrap()? - b'0';
        total_size += file_size as u64;
        let free_size = chunk.next().unwrap_or(Ok(b'0'))? - b'0';
        disk_map.push(Block {
            id,
            file_size,
            free_size,
        });
    }

    let mut checksum = 0;
    let mut start = disk_map.iter().flat_map(Block::expand);
    let mut end = disk_map.iter().rev().flat_map(Block::data_only);
    let mut pos = 0;
    loop {
        if pos == total_size {
            break;
        }

        let s = start.next().unwrap();
        match s {
            Item::Data(id) => {
                checksum += pos * id as u64;
            }
            Item::Free => {
                let e = end.next().unwrap();
                checksum += pos * e as u64;
            }
        }
        pos += 1;
    }

    println!("{}", checksum);
    Ok(())
}

#[derive(Debug)]
struct Block {
    id: ID,
    file_size: u8,
    free_size: u8,
}

type ID = usize;

impl Block {
    fn expand(&self) -> Expanded {
        Expanded {
            id: self.id,
            data_blocks: self.file_size,
            free_blocks: self.free_size,
        }
    }

    fn data_only(&self) -> RepeatN<ID> {
        iter::repeat_n(self.id, self.file_size as usize)
    }
}

#[derive(Debug)]
struct Expanded {
    id: ID,
    data_blocks: u8,
    free_blocks: u8,
}

#[derive(Debug)]
enum Item {
    Data(ID),
    Free,
}

impl Iterator for Expanded {
    type Item = Item;

    fn next(&mut self) -> Option<Self::Item> {
        if self.data_blocks != 0 {
            self.data_blocks -= 1;
            Some(Item::Data(self.id))
        } else if self.free_blocks != 0 {
            self.free_blocks -= 1;
            Some(Item::Free)
        } else {
            None
        }
    }
}
