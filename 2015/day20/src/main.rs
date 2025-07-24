use anyhow::Result;
use std::io;
use std::io::Read;

fn main() -> Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    let target = buffer.trim().parse::<u128>()?;

    println!("Part 1: {}", deliver(target, presents_part1));
    println!("Part 2: {}", deliver(target, presents_part2));

    Ok(())
}

fn deliver(target: u128, presents: fn(u128) -> u128) -> u128 {
    let mut i = 1;
    loop {
        if presents(i) >= target {
            return i;
        }
        i += 1;
    }
}

// Sample deliveries:
// 1 = 10  -> 10
// 2 = 30  -> 20 + 10
// 3 = 40  -> 30 + 10
// 4 = 70  -> 40 + 10 + 20
// 5 = 60  -> 50 + 10
// 6 = 120 -> 60 + 10 + 20 + 30
// 7 = 80  -> 70 + 10
// 8 = 150 -> 80 + 10 + 20 + 40
// 9 = 130 -> 90 + 10 + 30
// [...]
//
// The number of presents delivered (P) for house # (n):
//   P = n * 10 + 10 * sum { div(n) }
//   - div(n) - all divisors of n (including itself and 1)
fn presents_part1(house_num: u128) -> u128 {
    match house_num {
        1 => 10,
        2 => 10 + 20,
        _ => (divisors::get_divisors(house_num).iter().sum::<u128>() + house_num + 1) * 10,
    }
}

// This is the same as part 1 except now we're delivering 11 presents each and each elf stops after
// 50. Basically any divisor D is still good as long as 50D >= the house number.
fn presents_part2(house_num: u128) -> u128 {
    match house_num {
        1 => 11,
        2 => 22 + 11,
        _ => {
            let mut divisors = divisors::get_divisors(house_num);
            divisors.push(1);
            divisors.push(house_num);
            divisors
                .iter()
                .filter(|&&d| d * 50 >= house_num)
                .sum::<u128>()
                * 11
        }
    }
}
