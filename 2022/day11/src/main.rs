use std::{collections::VecDeque, env};

use anyhow::{Result, bail};
use num_integer::lcm;

fn main() -> Result<()> {
    let set = env::args().nth(1).unwrap_or("-i".to_string());
    let monkeys = match set.as_str() {
        "-s" => sample(),
        "-i" => input(),
        _ => bail!("unrecognized option {set}"),
    };

    println!("Part 1: {}", part1(&monkeys));
    println!("Part 2: {}", part2(&monkeys));

    Ok(())
}

fn part1(monkeys: &[Monkey]) -> usize {
    simulate(monkeys, 20, |i: usize| i / 3)
}

fn part2(monkeys: &[Monkey]) -> usize {
    let reduce_factor = monkeys.iter().map(|m| m.divisor).reduce(lcm).unwrap();
    simulate(monkeys, 10_000, |i: usize| i % reduce_factor)
}

fn simulate(monkeys: &[Monkey], rounds: usize, relief: impl Fn(usize) -> usize) -> usize {
    let mut monkeys = monkeys.to_vec();
    for _round in 0..rounds {
        for m in 0..monkeys.len() {
            while let Some(mut item) = monkeys[m].items.pop_front() {
                item = (monkeys[m].operation)(item);
                item = relief(item);
                let toss_to = (monkeys[m].throw_test)(item);
                monkeys[toss_to].items.push_back(item);
                monkeys[m].inspects += 1;
            }
        }
    }
    monkeys.sort_by_key(|m| m.inspects);
    monkeys.iter().rev().take(2).map(|m| m.inspects).product()
}

#[derive(Clone)]
struct Monkey {
    items: VecDeque<usize>,
    operation: fn(usize) -> usize,
    throw_test: fn(usize) -> usize,
    divisor: usize,
    inspects: usize,
}

fn sample() -> Vec<Monkey> {
    vec![
        Monkey {
            items: VecDeque::from([79, 98]),
            operation: |old: usize| old * 19,
            throw_test: |t: usize| if t.is_multiple_of(23) { 2 } else { 3 },
            divisor: 23,
            inspects: 0,
        },
        Monkey {
            items: VecDeque::from([54, 65, 75, 74]),
            operation: |old: usize| old + 6,
            throw_test: |t: usize| if t.is_multiple_of(19) { 2 } else { 0 },
            divisor: 19,
            inspects: 0,
        },
        Monkey {
            items: VecDeque::from([79, 60, 97]),
            operation: |old: usize| old * old,
            throw_test: |t: usize| if t.is_multiple_of(13) { 1 } else { 3 },
            divisor: 13,
            inspects: 0,
        },
        Monkey {
            items: VecDeque::from([74]),
            operation: |old: usize| old + 3,
            throw_test: |t: usize| if t.is_multiple_of(17) { 0 } else { 1 },
            divisor: 17,
            inspects: 0,
        },
    ]
}

fn input() -> Vec<Monkey> {
    vec![
        Monkey {
            items: VecDeque::from([66, 71, 94]),
            operation: |old: usize| old * 5,
            throw_test: |t: usize| if t.is_multiple_of(3) { 7 } else { 4 },
            divisor: 3,
            inspects: 0,
        },
        Monkey {
            items: VecDeque::from([70]),
            operation: |old: usize| old + 6,
            throw_test: |t: usize| if t.is_multiple_of(17) { 3 } else { 0 },
            divisor: 17,
            inspects: 0,
        },
        Monkey {
            items: VecDeque::from([62, 68, 56, 65, 94, 78]),
            operation: |old: usize| old + 5,
            throw_test: |t: usize| if t.is_multiple_of(2) { 3 } else { 1 },
            divisor: 2,
            inspects: 0,
        },
        Monkey {
            items: VecDeque::from([89, 94, 94, 67]),
            operation: |old: usize| old + 2,
            throw_test: |t: usize| if t.is_multiple_of(19) { 7 } else { 0 },
            divisor: 19,
            inspects: 0,
        },
        Monkey {
            items: VecDeque::from([71, 61, 73, 65, 98, 98, 63]),
            operation: |old: usize| old * 7,
            throw_test: |t: usize| if t.is_multiple_of(11) { 5 } else { 6 },
            divisor: 11,
            inspects: 0,
        },
        Monkey {
            items: VecDeque::from([55, 62, 68, 61, 60]),
            operation: |old: usize| old + 7,
            throw_test: |t: usize| if t.is_multiple_of(5) { 2 } else { 1 },
            divisor: 5,
            inspects: 0,
        },
        Monkey {
            items: VecDeque::from([93, 91, 69, 64, 72, 89, 50, 71]),
            operation: |old: usize| old + 1,
            throw_test: |t: usize| if t.is_multiple_of(13) { 5 } else { 2 },
            divisor: 13,
            inspects: 0,
        },
        Monkey {
            items: VecDeque::from([76, 50]),
            operation: |old: usize| old * old,
            throw_test: |t: usize| if t.is_multiple_of(7) { 4 } else { 6 },
            divisor: 7,
            inspects: 0,
        },
    ]
}
