use anyhow::Result;

fn main() -> Result<()> {
    let (_immune_won, winning_units) = combat(get_input_armies(0));
    println!("Part 1: {winning_units}");

    for boost in 1.. {
        let (immune_won, winning_units) = combat(get_input_armies(boost));
        if immune_won {
            println!("Part 2: {winning_units}");
            break;
        }
    }

    Ok(())
}

fn combat(mut armies: Vec<Group>) -> (bool, usize) {
    while !armies.combat_over() {
        let mut combat_pairs = armies.combat_pairs();

        combat_pairs.sort_by(|(a, _), (b, _)| armies[*b].initiative.cmp(&armies[*a].initiative));

        let mut total_kills = 0usize;
        for (attacker, defender) in &combat_pairs {
            if armies[*attacker].dead {
                continue;
            }
            let damage = armies[*attacker].damage_dealt_to(&armies[*defender]);
            let units_lost = damage / armies[*defender].hp;
            if units_lost >= armies[*defender].units {
                total_kills += armies[*defender].units;
                armies[*defender].dead = true;
            } else {
                total_kills += units_lost;
                armies[*defender].units -= units_lost;
            }
        }

        // Stalemate: no units killed this round, combat will never end
        if total_kills == 0 {
            return (false, armies.total_units());
        }

        armies.retain(|g| !g.dead);
    }

    (armies[0].immune_team, armies.total_units())
}

#[derive(Debug, Clone)]
struct Group {
    immune_team: bool,
    dead: bool,
    units: usize,
    hp: usize,
    immunities: Vec<String>,
    weaknesses: Vec<String>,
    attack_power: usize,
    damage_type: String,
    initiative: isize,
}

impl Group {
    fn effective_power(&self) -> usize {
        self.units * self.attack_power
    }

    fn damage_dealt_to(&self, other: &Group) -> usize {
        if other.immunities.contains(&self.damage_type) {
            0
        } else {
            let scale = if other.weaknesses.contains(&self.damage_type) {
                2
            } else {
                1
            };
            self.units * self.attack_power * scale
        }
    }
}

trait Armies {
    fn total_units(&self) -> usize;
    fn combat_over(&self) -> bool;
    fn combat_pairs(&self) -> Vec<(usize, usize)>;
}

impl Armies for Vec<Group> {
    fn total_units(&self) -> usize {
        self.iter().map(|g| g.units).sum()
    }

    fn combat_over(&self) -> bool {
        self.iter().all(|g| g.immune_team == self[0].immune_team)
    }

    // Return indices of attacking / defending pairs
    fn combat_pairs(&self) -> Vec<(usize, usize)> {
        let mut selection_order: Vec<usize> = (0..self.len()).collect();
        selection_order.sort_by(|&a, &b| {
            self[b]
                .effective_power()
                .cmp(&self[a].effective_power())
                .then_with(|| self[b].initiative.cmp(&self[a].initiative))
        });

        let mut pairs = Vec::new();
        let mut targeted: Vec<bool> = vec![false; self.len()];

        for i in selection_order {
            let attacking_group = &self[i];
            let mut best_target: Option<usize> = None;
            let mut best_damage = 0usize;
            let mut best_ep = 0usize;
            let mut best_init = isize::MIN;

            for j in 0..self.len() {
                if self[j].immune_team == attacking_group.immune_team || targeted[j] {
                    continue;
                }
                let damage = attacking_group.damage_dealt_to(&self[j]);
                if damage == 0 {
                    continue;
                }
                let ep = self[j].effective_power();
                let init = self[j].initiative;
                if damage > best_damage
                    || (damage == best_damage && ep > best_ep)
                    || (damage == best_damage && ep == best_ep && init > best_init)
                {
                    best_target = Some(j);
                    best_damage = damage;
                    best_ep = ep;
                    best_init = init;
                }
            }

            if let Some(j) = best_target {
                targeted[j] = true;
                pairs.push((i, j));
            }
        }
        pairs
    }
}

fn get_input_armies(boost: usize) -> Vec<Group> {
    let mut armies = vec![
        // Immune System:
        // 2334 units each with 8900 hit points with an attack that does 31 fire damage at initiative 4
        Group {
            immune_team: true,
            dead: false,
            units: 2334,
            hp: 8900,
            immunities: vec![],
            weaknesses: vec![],
            attack_power: 31,
            damage_type: "fire".to_owned(),
            initiative: 4,
        },
        // 411 units each with 8067 hit points with an attack that does 195 radiation damage at initiative 1
        Group {
            immune_team: true,
            dead: false,
            units: 411,
            hp: 8067,
            immunities: vec![],
            weaknesses: vec![],
            attack_power: 195,
            damage_type: "radiation".to_owned(),
            initiative: 1,
        },
        // 449 units each with 9820 hit points (weak to fire) with an attack that does 193 bludgeoning damage at initiative 3
        Group {
            immune_team: true,
            dead: false,
            units: 449,
            hp: 9820,
            immunities: vec![],
            weaknesses: vec!["fire".to_owned()],
            attack_power: 193,
            damage_type: "bludgeoning".to_owned(),
            initiative: 3,
        },
        // 452 units each with 4418 hit points (weak to fire; immune to bludgeoning) with an attack that does 89 bludgeoning damage at initiative 10
        Group {
            immune_team: true,
            dead: false,
            units: 452,
            hp: 4418,
            immunities: vec!["bludgeoning".to_owned()],
            weaknesses: vec!["fire".to_owned()],
            attack_power: 89,
            damage_type: "bludgeoning".to_owned(),
            initiative: 10,
        },
        // 858 units each with 5016 hit points (weak to bludgeoning, fire; immune to slashing) with an attack that does 58 bludgeoning damage at initiative 18
        Group {
            immune_team: true,
            dead: false,
            units: 858,
            hp: 5016,
            immunities: vec!["slashing".to_owned()],
            weaknesses: vec!["bludgeoning".to_owned(), "fire".to_owned()],
            attack_power: 58,
            damage_type: "bludgeoning".to_owned(),
            initiative: 18,
        },
        // 3049 units each with 9940 hit points with an attack that does 29 cold damage at initiative 12
        Group {
            immune_team: true,
            dead: false,
            units: 3049,
            hp: 9940,
            immunities: vec![],
            weaknesses: vec![],
            attack_power: 29,
            damage_type: "cold".to_owned(),
            initiative: 12,
        },
        // 610 units each with 7021 hit points (weak to bludgeoning, radiation) with an attack that does 114 fire damage at initiative 7
        Group {
            immune_team: true,
            dead: false,
            units: 610,
            hp: 7021,
            immunities: vec![],
            weaknesses: vec!["bludgeoning".to_owned(), "radiation".to_owned()],
            attack_power: 114,
            damage_type: "fire".to_owned(),
            initiative: 7,
        },
        // 4033 units each with 8807 hit points (weak to radiation) with an attack that does 21 cold damage at initiative 5
        Group {
            immune_team: true,
            dead: false,
            units: 4033,
            hp: 8807,
            immunities: vec![],
            weaknesses: vec!["radiation".to_owned()],
            attack_power: 21,
            damage_type: "cold".to_owned(),
            initiative: 5,
        },
        // 1209 units each with 7468 hit points (weak to fire; immune to cold) with an attack that does 50 radiation damage at initiative 20
        Group {
            immune_team: true,
            dead: false,
            units: 1209,
            hp: 7468,
            immunities: vec!["cold".to_owned()],
            weaknesses: vec!["fire".to_owned()],
            attack_power: 50,
            damage_type: "radiation".to_owned(),
            initiative: 20,
        },
        // 3228 units each with 7550 hit points (weak to cold; immune to bludgeoning, radiation) with an attack that does 21 slashing damage at initiative 14
        Group {
            immune_team: true,
            dead: false,
            units: 3228,
            hp: 7550,
            immunities: vec!["bludgeoning".to_owned(), "radiation".to_owned()],
            weaknesses: vec!["cold".to_owned()],
            attack_power: 21,
            damage_type: "slashing".to_owned(),
            initiative: 14,
        },
        // Infection:
        // 1230 units each with 36915 hit points (weak to cold, slashing) with an attack that does 58 bludgeoning damage at initiative 16
        Group {
            immune_team: false,
            dead: false,
            units: 1230,
            hp: 36915,
            immunities: vec![],
            weaknesses: vec!["cold".to_owned(), "slashing".to_owned()],
            attack_power: 58,
            damage_type: "bludgeoning".to_owned(),
            initiative: 16,
        },
        // 629 units each with 23164 hit points with an attack that does 72 slashing damage at initiative 11
        Group {
            immune_team: false,
            dead: false,
            units: 629,
            hp: 23164,
            immunities: vec![],
            weaknesses: vec![],
            attack_power: 72,
            damage_type: "slashing".to_owned(),
            initiative: 11,
        },
        // 266 units each with 16518 hit points with an attack that does 113 fire damage at initiative 2
        Group {
            immune_team: false,
            dead: false,
            units: 266,
            hp: 16518,
            immunities: vec![],
            weaknesses: vec![],
            attack_power: 113,
            damage_type: "fire".to_owned(),
            initiative: 2,
        },
        // 45 units each with 17769 hit points (immune to radiation, slashing, bludgeoning, fire) with an attack that does 774 fire damage at initiative 19
        Group {
            immune_team: false,
            dead: false,
            units: 45,
            hp: 17769,
            immunities: vec![
                "radiation".to_owned(),
                "slashing".to_owned(),
                "bludgeoning".to_owned(),
                "fire".to_owned(),
            ],
            weaknesses: vec![],
            attack_power: 774,
            damage_type: "fire".to_owned(),
            initiative: 19,
        },
        // 93 units each with 32105 hit points (weak to fire) with an attack that does 535 fire damage at initiative 8
        Group {
            immune_team: false,
            dead: false,
            units: 93,
            hp: 32105,
            immunities: vec![],
            weaknesses: vec!["fire".to_owned()],
            attack_power: 535,
            damage_type: "fire".to_owned(),
            initiative: 8,
        },
        // 957 units each with 19599 hit points (immune to cold) with an attack that does 32 cold damage at initiative 15
        Group {
            immune_team: false,
            dead: false,
            units: 957,
            hp: 19599,
            immunities: vec!["cold".to_owned()],
            weaknesses: vec![],
            attack_power: 32,
            damage_type: "cold".to_owned(),
            initiative: 15,
        },
        // 347 units each with 29661 hit points with an attack that does 170 bludgeoning damage at initiative 17
        Group {
            immune_team: false,
            dead: false,
            units: 347,
            hp: 29661,
            immunities: vec![],
            weaknesses: vec![],
            attack_power: 170,
            damage_type: "bludgeoning".to_owned(),
            initiative: 17,
        },
        // 418 units each with 17587 hit points (immune to fire, slashing; weak to radiation, cold) with an attack that does 73 bludgeoning damage at initiative 6
        Group {
            immune_team: false,
            dead: false,
            units: 418,
            hp: 17587,
            immunities: vec!["fire".to_owned(), "slashing".to_owned()],
            weaknesses: vec!["radiation".to_owned(), "cold".to_owned()],
            attack_power: 73,
            damage_type: "bludgeoning".to_owned(),
            initiative: 6,
        },
        // 2656 units each with 49851 hit points with an attack that does 32 radiation damage at initiative 9
        Group {
            immune_team: false,
            dead: false,
            units: 2656,
            hp: 49851,
            immunities: vec![],
            weaknesses: vec![],
            attack_power: 32,
            damage_type: "radiation".to_owned(),
            initiative: 9,
        },
        // 5365 units each with 35984 hit points with an attack that does 13 radiation damage at initiative 13
        Group {
            immune_team: false,
            dead: false,
            units: 5365,
            hp: 35984,
            immunities: vec![],
            weaknesses: vec![],
            attack_power: 13,
            damage_type: "radiation".to_owned(),
            initiative: 13,
        },
    ];
    armies
        .iter_mut()
        .filter(|g| g.immune_team)
        .for_each(|g| g.attack_power += boost);
    armies
}

// fn get_sample_armies(boost: usize) -> Vec<Group> {
//     let mut armies = vec![
//         // 17 units each with 5390 hit points (weak to radiation, bludgeoning) with
//         //  an attack that does 4507 fire damage at initiative 2
//         Group {
//             immune_team: true,
//             dead: false,
//             units: 17,
//             hp: 5390,
//             immunities: vec![],
//             weaknesses: vec!["radiation".to_owned(), "bludgeoning".to_owned()],
//             attack_power: 4507,
//             damage_type: "fire".to_owned(),
//             initiative: 2,
//         },
//         // 989 units each with 1274 hit points (immune to fire; weak to bludgeoning,
//         //  slashing) with an attack that does 25 slashing damage at initiative 3
//         Group {
//             immune_team: true,
//             dead: false,
//             units: 989,
//             hp: 1274,
//             immunities: vec!["fire".to_owned()],
//             weaknesses: vec!["bludgeoning".to_owned(), "slashing".to_owned()],
//             attack_power: 25,
//             damage_type: "slashing".to_owned(),
//             initiative: 3,
//         },
//         // 801 units each with 4706 hit points (weak to radiation) with an attack
//         //  that does 116 bludgeoning damage at initiative 1
//         Group {
//             immune_team: false,
//             dead: false,
//             units: 801,
//             hp: 4706,
//             immunities: vec![],
//             weaknesses: vec!["radiation".to_owned()],
//             attack_power: 116,
//             damage_type: "bludgeoning".to_owned(),
//             initiative: 1,
//         },
//         // 4485 units each with 2961 hit points (immune to radiation; weak to fire,
//         //  cold) with an attack that does 12 slashing damage at initiative 4
//         Group {
//             immune_team: false,
//             dead: false,
//             units: 4485,
//             hp: 2961,
//             immunities: vec!["radiation".to_owned()],
//             weaknesses: vec!["fire".to_owned(), "cold".to_owned()],
//             attack_power: 12,
//             damage_type: "slashing".to_owned(),
//             initiative: 4,
//         },
//     ];
//     armies
//         .iter_mut()
//         .filter(|g| g.immune_team)
//         .for_each(|g| g.attack_power += boost);
//     armies
// }
