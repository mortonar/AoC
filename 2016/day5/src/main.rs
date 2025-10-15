use anyhow::Result;
use std::fmt::Write;
use std::io;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let door_id = input.trim();

    let (mut pass_1, mut pass_2) = (['_'; 8], ['_'; 8]);
    let mut p1_i = 0;
    let mut index = 0;
    let mut buf = String::with_capacity(door_id.len() + 10);
    let hex_char = |n: u8| char::from_digit(n as u32, 16).unwrap();
    while pass_1.contains(&'_') || pass_2.contains(&'_') {
        buf.clear();
        write!(&mut buf, "{}{}", door_id, index)?;
        index += 1;

        let hash: [u8; 16] = md5::compute(buf.as_bytes()).into();
        if hash[0] != 0x00 || hash[1] != 0x00 || hash[2] > 0x0F {
            continue;
        }
        let sixth = hex_char(hash[2] & 0x0F);

        if p1_i < 8 {
            pass_1[p1_i] = sixth;
            p1_i += 1;
        }

        let position = sixth;
        if let '0'..='7' = position {
            let position = position.to_digit(10).unwrap() as usize;
            if pass_2[position] == '_' {
                let seventh = hex_char((hash[3] & 0xF0) >> 4);
                pass_2[position] = seventh;
            }
        }
    }

    println!("Part 1: {}", pass_1.iter().collect::<String>());
    println!("Part 2: {}", pass_2.iter().collect::<String>());

    Ok(())
}
