use anyhow::{Error, Result, bail};
use std::cmp::Ordering;
use std::collections::HashMap;
use std::io::stdin;
use std::str::FromStr;

fn main() -> Result<()> {
    let mut records = parse_input()?;
    records.sort_by(|r1, r2| r1.timestamp.cmp(&r2.timestamp));
    dbg!(&records);

    // TODO Get guard asleep the most
    Ok(())
}

fn parse_input() -> Result<Vec<Record>> {
    stdin().lines().map(|l| l?.parse()).collect()
}

#[derive(Debug)]
struct Record {
    timestamp: Timestamp,
    event: Event,
}

#[derive(Debug)]
struct Timestamp {
    year: u64,
    month: u64,
    day: u64,
    hour: u64,
    minute: u64,
}

#[derive(Debug)]
enum Event {
    BeginShift(u64),
    Sleep,
    Wake,
}

fn part1(records: &[Record]) {
    let mut guard_totals: HashMap<u64, Vec<u64>> = HashMap::new();
    let mut guard_id = 0;
    let mut sleep_start = None;
    for record in records.iter() {
        match record.event {
            Event::BeginShift(id) => guard_id = id,
            Event::Sleep => sleep_start = Some(record.timestamp),
            Event::Wake =>
                _ => {}
        }
    }
}

impl FromStr for Record {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let tokens: Vec<_> = s.split("]").collect();
        Ok(Self {
            timestamp: tokens[0][1..].parse()?,
            event: tokens[1][1..].parse()?,
        })
    }
}

impl FromStr for Timestamp {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let tokens: Vec<_> = s.split(" ").collect();
        let ymd: Vec<_> = tokens[0].split("-").collect();
        let hm: Vec<_> = tokens[1].split(":").collect();
        Ok(Self {
            year: ymd[0].parse()?,
            month: ymd[1].parse()?,
            day: ymd[2].parse()?,
            hour: hm[0].parse()?,
            minute: hm[1].parse()?,
        })
    }
}

impl FromStr for Event {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        if s.contains("begins shift") {
            let tokens: Vec<_> = s.split_ascii_whitespace().collect();
            Ok(Event::BeginShift(tokens[1][1..].parse()?))
        } else if s.contains("falls asleep") {
            Ok(Event::Sleep)
        } else if s.contains("wakes up") {
            Ok(Event::Wake)
        } else {
            bail!("Unrecognized event: {}", s);
        }
    }
}

impl Eq for Timestamp {}

impl PartialEq<Self> for Timestamp {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other).is_eq()
    }
}

impl PartialOrd<Self> for Timestamp {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Timestamp {
    fn cmp(&self, other: &Self) -> Ordering {
        self.year
            .cmp(&other.year)
            .then_with(|| self.month.cmp(&other.month))
            .then_with(|| self.day.cmp(&other.day))
            .then_with(|| self.hour.cmp(&other.hour))
            .then_with(|| self.minute.cmp(&other.minute))
    }
}
