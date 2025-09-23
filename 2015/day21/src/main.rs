use anyhow::Result;
use itertools::Itertools;
use std::io;
use std::io::BufRead;

fn main() -> Result<()> {
    let mut boss = Character::default();

    for line in io::stdin().lock().lines() {
        let line = line?;
        let tokens: Vec<_> = line.trim().split_whitespace().collect();
        let value = tokens.last().unwrap().parse()?;
        let setting: &mut isize = match tokens[0] {
            "Hit" => &mut boss.hp,
            "Damage:" => &mut boss.damage,
            "Armor:" => &mut boss.armor,
            _ => panic!("Unexpected input"),
        };
        *setting = value;
    }

    let weapons = vec![
        Item::new("Dagger", 8, 4, 0),
        Item::new("Shortsword", 10, 5, 0),
        Item::new("Warhammer", 25, 6, 0),
        Item::new("Longsword", 40, 7, 0),
        Item::new("Greataxe", 74, 8, 0),
    ];
    let armors = vec![
        Item::new("Leather", 13, 0, 1),
        Item::new("Chainmail", 31, 0, 2),
        Item::new("Splintmail", 53, 0, 3),
        Item::new("Bandedmail", 75, 0, 4),
        Item::new("Platemail", 102, 0, 5),
    ];
    let rings = vec![
        Item::new("Damage +1", 25, 1, 0),
        Item::new("Damage +2", 50, 2, 0),
        Item::new("Damage +3", 100, 3, 0),
        Item::new("Defense +1", 20, 0, 1),
        Item::new("Defense +2", 40, 0, 2),
        Item::new("Defense +3", 80, 0, 3),
    ];

    let mut min_gold = usize::MAX;
    let mut max_gold = 0;
    for weapon in weapons.iter() {
        for a in 0..=1 {
            for armor in armors.iter().combinations(a) {
                for r in 0..=2 {
                    for ring in rings.iter().combinations(r) {
                        let mut items = Vec::new();
                        items.push(weapon);
                        if a == 1 {
                            items.push(armor[0]);
                        }
                        if r >= 1 {
                            ring.iter().for_each(|ring| items.push(ring));
                        }
                        let player = Character::new(
                            100,
                            items.iter().map(|i| i.damage).sum(),
                            items.iter().map(|i| i.armor).sum(),
                        );
                        let player_victor = combat(player, boss.clone());
                        let gold: usize = items.iter().map(|i| i.cost).sum();
                        if player_victor {
                            min_gold = min_gold.min(gold);
                        } else {
                            max_gold = max_gold.max(gold);
                        }
                    }
                }
            }
        }
    }
    println!("Part 1: {}", min_gold);
    println!("Part 2: {}", max_gold);

    Ok(())
}

// True if player wins, false if boss wins
fn combat(mut player: Character, mut boss: Character) -> bool {
    while player.hp > 0 && boss.hp > 0 {
        boss.hp -= (player.damage - boss.armor).max(1);
        if boss.hp <= 0 {
            return true;
        }

        player.hp -= (boss.damage - player.armor).max(1);
        if player.hp <= 0 {
            return false;
        }
    }
    panic!("Should not reach here");
}

#[derive(Clone, Default, Debug)]
struct Character {
    hp: isize,
    damage: isize,
    armor: isize,
}

impl Character {
    fn new(hp: isize, damage: isize, armor: isize) -> Self {
        Self { hp, damage, armor }
    }
}

#[derive(Clone, Debug)]
struct Item {
    _name: String,
    cost: usize,
    damage: isize,
    armor: isize,
}

impl Item {
    fn new(name: &str, cost: usize, damage: isize, armor: isize) -> Self {
        Self {
            _name: name.to_string(),
            cost,
            damage,
            armor,
        }
    }
}
