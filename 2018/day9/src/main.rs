use anyhow::{Error, Result};
use std::collections::VecDeque;
use std::io::stdin;
use std::str::FromStr;

// Part 2 turned this into a humbling problem. After letting part 2 spin and looking at some hints,
// I realized that representing the marble circle as a Vec is a sloppy and horribly inefficient.
// Remove operations become more and more expensive as the circle grows, requiring larger copies to
// fill the hole in the vec. Additionally, the index management is very tricky: something I spent
// a decent amount of time on for part 1. I could blame the sample walkthrough as leading me astray
// down the "vec path" but I think that was the point/trick for the problem ;). Sometimes the
// graphics used to demonstrate the problem don't translate 1-1 with the data structures.
//
// Using a double-ended queue addresses both the inefficiencies of the Vec and removes the pesky
// index logic altogether. Removing an element and filling the hole is now constant time since all
// the rotations are just pointer math ops.
fn main() -> Result<()> {
    let mut game = parse_input()?;

    println!("Part 1 {}", game.play());

    game.target_points *= 100;
    println!("Part 2 {}", game.play());

    Ok(())
}

fn parse_input() -> Result<Game> {
    stdin().lines().next().unwrap()?.parse()
}

#[derive(Clone, Debug)]
struct Game {
    players: usize,
    target_points: usize,
}

impl FromStr for Game {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let tokens: Vec<_> = s.split_ascii_whitespace().collect();
        let players = tokens[0].parse()?;
        let target_points = tokens[6].parse()?;
        Ok(Game {
            players,
            target_points,
        })
    }
}

impl Game {
    fn play(&mut self) -> usize {
        let mut scores = vec![0; self.players];

        // The end of circle is always the current marble
        let mut circle = VecDeque::with_capacity(self.target_points);
        circle.push_back(0);

        for marble in 1..=self.target_points {
            if marble.is_multiple_of(23) {
                circle.rotate_right(7);
                let points = marble + circle.pop_back().unwrap();
                // 1-based player to 0-based index: scores[0] = last player's score
                scores[marble % self.players] += points;
                circle.rotate_left(1);
            } else {
                circle.rotate_left(1);
                circle.push_back(marble);
            }
        }

        *scores.iter().max().unwrap()
    }
}
