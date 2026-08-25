use anyhow::Result;
use std::io::{BufRead, stdin};

fn main() -> Result<()> {
    let raw = parse_input()?;

    let (_, packet) = parse_packet(&raw);
    println!("Part 1: {}", part1(&packet));
    println!("Part 2: {}", part2(&packet));

    Ok(())
}

fn parse_input() -> Result<BitString> {
    let mut line = String::new();
    stdin().lock().read_line(&mut line)?;

    let bit_string = line
        .trim()
        .chars()
        .flat_map(|hex| {
            match hex {
                '0' => "0000",
                '1' => "0001",
                '2' => "0010",
                '3' => "0011",
                '4' => "0100",
                '5' => "0101",
                '6' => "0110",
                '7' => "0111",
                '8' => "1000",
                '9' => "1001",
                'A' => "1010",
                'B' => "1011",
                'C' => "1100",
                'D' => "1101",
                'E' => "1110",
                _ => "1111",
            }
            .chars()
        })
        .collect();

    Ok(bit_string)
}

type BitString = String;

fn part1(packet: &Packet) -> u64 {
    packet.add_versions()
}

fn part2(packet: &Packet) -> u64 {
    packet.eval()
}

#[derive(Default, Debug)]
struct Packet {
    version: u64,
    type_id: u64,
    literal: u64,
    sub_packets: Vec<Packet>,
}

fn parse_packet(bits: &str) -> (&str, Packet) {
    let mut packet = Packet::default();

    let (bits, version) = bits.split_parse(3);
    packet.version = version;

    let (bits, type_id) = bits.split_parse(3);
    packet.type_id = type_id;

    match packet.type_id {
        4 => {
            let (bits, literal) = parse_literal(bits);
            packet.literal = literal;
            (bits, packet)
        }
        _ => {
            let (bits, sub_packets) = parse_sub_packets(bits);
            packet.sub_packets = sub_packets;
            (bits, packet)
        }
    }
}

fn parse_literal(bits: &str) -> (&str, u64) {
    let mut bits = bits;
    let mut accum = String::new();

    let mut lit: &str;
    loop {
        (lit, bits) = bits.split_at(5);
        accum.push_str(&lit[1..]);

        if &lit[0..1] == "0" {
            break;
        }
    }

    (bits, u64::from_str_radix(&accum, 2).unwrap())
}

fn parse_sub_packets(bits: &str) -> (&str, Vec<Packet>) {
    let mut sub_packets = Vec::new();
    let (bits, length_type) = bits.split_parse(1);

    if length_type == 0 {
        let (bits, length) = bits.split_parse(15);
        let (mut sub_bits, rest) = bits.split_at(length as usize);

        while !sub_bits.is_empty() {
            let (new_bits, packet) = parse_packet(sub_bits);
            sub_bits = new_bits;
            sub_packets.push(packet);
        }

        (rest, sub_packets)
    } else {
        let (mut bits, num_subs) = bits.split_parse(11);

        for _ in 0..num_subs {
            let (new_bits, packet) = parse_packet(bits);
            bits = new_bits;
            sub_packets.push(packet);
        }

        (bits, sub_packets)
    }
}

trait SplitParse {
    fn split_parse(&self, bits: usize) -> (&str, u64);
}

impl SplitParse for str {
    fn split_parse(&self, bits: usize) -> (&str, u64) {
        let (prefix, suffix) = self.split_at(bits);
        (suffix, u64::from_str_radix(prefix, 2).unwrap())
    }
}

impl Packet {
    fn add_versions(&self) -> u64 {
        self.version
            + self
                .sub_packets
                .iter()
                .map(Packet::add_versions)
                .sum::<u64>()
    }

    fn eval(&self) -> u64 {
        match self.type_id {
            0 => self.sub_packets.iter().map(Packet::eval).sum(),
            1 => self.sub_packets.iter().map(Packet::eval).product(),
            2 => self.sub_packets.iter().map(Packet::eval).min().unwrap(),
            3 => self.sub_packets.iter().map(Packet::eval).max().unwrap(),
            5 => (self.sub_packets[0].eval() > self.sub_packets[1].eval()) as u64,
            6 => (self.sub_packets[0].eval() < self.sub_packets[1].eval()) as u64,
            7 => (self.sub_packets[0].eval() == self.sub_packets[1].eval()) as u64,
            _ => self.literal,
        }
    }
}
