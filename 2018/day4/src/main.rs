use anyhow::{Error, Result, bail};
use std::cmp::Ordering;
use std::collections::HashMap;
use std::io::stdin;
use std::str::FromStr;

fn main() -> Result<()> {
    let mut records = parse_input()?;
    records.sort_by(|r1, r2| r1.timestamp.cmp(&r2.timestamp));
    // Guard ID -> Vec of midnight minutes asleep
    let guard_totals = midnight_minutes_asleep(&records);

    let sleepiest = guard_totals.iter().max_by_key(|(_id, v)| v.len()).unwrap();
    println!("Part 1: {}", sleepiest.0 * sleepiest.1.mode());

    let most_regular = guard_totals
        .iter()
        .max_by_key(|(_id, v)| {
            let mode = v.mode();
            v.iter().filter(|m| **m == mode).count()
        })
        .unwrap();
    println!("Part 2: {}", most_regular.0 * most_regular.1.mode());

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

#[derive(Clone, Debug)]
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

// Calculate: Guard ID -> Vec of midnight minutes asleep
fn midnight_minutes_asleep(records: &[Record]) -> HashMap<u64, Vec<u64>> {
    let mut guard_totals: HashMap<u64, Vec<u64>> = HashMap::new();
    let mut guard_id = 0;
    let mut sleep_start: usize = 0;
    for (i, record) in records.iter().enumerate() {
        match record.event {
            Event::BeginShift(id) => guard_id = id,
            Event::Sleep => sleep_start = i,
            Event::Wake => {
                let mut midnight_mins_asleep = records[sleep_start]
                    .timestamp
                    .midnight_mins(&records[i].timestamp);
                guard_totals
                    .entry(guard_id)
                    .and_modify(|mins| mins.append(&mut midnight_mins_asleep))
                    .or_insert(midnight_mins_asleep);
            }
        }
    }
    guard_totals
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

impl Timestamp {
    /// Minutes elapsed in the midnight hour between two timestamps (other >= self)
    fn midnight_mins(&self, other: &Timestamp) -> Vec<u64> {
        // "Walk" self to other and return the midnight minutes
        let mut mins = Vec::new();
        let mut current = self.clone();
        while current.cmp(other) != Ordering::Equal {
            if current.hour == 0 {
                mins.push(current.minute);
            }
            current.tick();
        }
        mins
    }

    fn tick(&mut self) {
        self.minute += 1;
        if self.minute > 59 {
            self.minute = 0;
            self.hour += 1;
        }
        // Ignore days and beyond
    }
}

trait Mode {
    fn mode(&self) -> u64;
}

impl Mode for Vec<u64> {
    fn mode(&self) -> u64 {
        let mut frequencies = HashMap::new();
        self.iter()
            .for_each(|&f| *frequencies.entry(f).or_insert(1) += 1);
        *frequencies.iter().max_by_key(|(_min, occ)| *occ).unwrap().0
    }
}
