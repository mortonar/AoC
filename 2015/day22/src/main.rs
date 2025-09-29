use anyhow::Result;
use std::io;
use std::io::BufRead;

fn main() -> Result<()> {
    let mut boss = Character::default();

    for line in io::stdin().lock().lines() {
        let line = line?;
        let tokens: Vec<_> = line.trim().split_whitespace().collect();
        let value = tokens.last().unwrap().parse()?;
        match tokens[0] {
            "Hit" => boss.hp = value,
            "Damage:" => boss.damage = value,
            _ => panic!("Unexpected input"),
        };
    }
    let boss = boss;

    let player = Character::new(50, 0, 0, 500);
    let mut game_state = GameState {
        player: player,
        boss: boss,
        active_spells: SpellStack::default(),
        mana_spent: 0,
        hard_mode: false,
    };
    let mana_spent = combat_bfs(game_state.clone());
    println!("Part 1: {}", mana_spent.unwrap());

    game_state.hard_mode = true;
    let mana_spent = combat_bfs(game_state.clone());
    println!("Part 2: {}", mana_spent.unwrap());

    Ok(())
}

// Battle steps:
//   - Player:
//     * Apply effects and discard expired effects
//     * Choose and cast a spell (apply immediate effects, reduce mana)
//       - If you can't cast a spell, you lose
//       - Can't cast a spell that's already active but you can recast a spell that's expiring
//   - Boss:
//     * Apply effects and discard expired effects
//     * Attack
//
// -> Some(usize) if player wins, None if boss wins
fn combat_bfs(game_state: GameState) -> Option<usize> {
    let mut min_mana_win: Option<usize> = None;
    fn update_min_mana(min: &mut Option<usize>, spent: usize) {
        *min = Some(match *min {
            Some(current) => current.min(spent),
            None => spent,
        });
    }

    let mut queue = vec![game_state];

    while !queue.is_empty() {
        let mut game_state = queue.pop().unwrap();

        // Player:
        if game_state.hard_mode {
            game_state.player.hp = game_state.player.hp.saturating_sub(1);
            if game_state.is_player_dead() {
                continue;
            }
        }
        game_state.apply_effects();
        if game_state.is_boss_dead() {
            update_min_mana(&mut min_mana_win, game_state.mana_spent);
            continue;
        }

        for spell in game_state.possible_spells().into_iter() {
            let mut new_state = game_state.apply_spell(spell);

            if new_state.is_boss_dead() {
                update_min_mana(&mut min_mana_win, new_state.mana_spent);
                continue;
            }

            // Boss:
            new_state.apply_effects();
            if new_state.is_boss_dead() {
                update_min_mana(&mut min_mana_win, new_state.mana_spent);
                continue;
            }
            new_state.boss_attack();
            // Don't bother exploring paths that already exceed the best win
            if !new_state.is_player_dead()
                && new_state.mana_spent < min_mana_win.unwrap_or(usize::MAX)
            {
                queue.push(new_state);
            }
        }
    }

    min_mana_win
}

#[derive(Clone, Default, Debug)]
struct GameState {
    player: Character,
    boss: Character,
    active_spells: SpellStack,
    mana_spent: usize,
    hard_mode: bool,
}

impl GameState {
    fn apply_effects(&mut self) {
        self.active_spells
            .apply_effects(&mut self.player, &mut self.boss);
    }

    fn is_boss_dead(&self) -> bool {
        self.boss.hp == 0
    }

    fn is_player_dead(&self) -> bool {
        self.player.hp == 0
    }

    fn possible_spells(&self) -> Vec<Spell> {
        self.active_spells.possible_spells(self.player.mana)
    }

    fn apply_spell(&self, spell: Spell) -> Self {
        let mut new_state = self.clone();
        new_state.mana_spent += spell.cost;
        new_state
            .active_spells
            .cast(spell, &mut new_state.player, &mut new_state.boss);
        new_state
    }

    fn boss_attack(&mut self) {
        let player_armor = self
            .active_spells
            .spells
            .iter()
            .map(|s| s.armor)
            .sum::<usize>()
            + self.player.armor;
        let damage = self.boss.damage.saturating_sub(player_armor).max(1);
        self.player.hp = self.player.hp.saturating_sub(damage);
    }
}

#[derive(Clone, Default, Debug)]
struct Character {
    hp: usize,
    damage: usize,
    armor: usize,
    mana: usize,
}

impl Character {
    fn new(hp: usize, damage: usize, armor: usize, mana: usize) -> Self {
        Self {
            hp,
            damage,
            armor,
            mana,
        }
    }
}

#[derive(Clone, Debug, Default)]
struct SpellStack {
    spells: Vec<Spell>,
}

impl SpellStack {
    fn cast(&mut self, mut spell: Spell, player: &mut Character, boss: &mut Character) {
        player.mana -= spell.cost;

        if spell.instant {
            spell.apply_effects(player, boss);
        } else {
            self.spells.push(spell);
        }
    }

    fn apply_effects(&mut self, player: &mut Character, boss: &mut Character) {
        for spell in self.spells.iter_mut() {
            spell.apply_effects(player, boss);
        }
        self.spells.retain(|s| s.duration > 0);
    }

    fn possible_spells(&self, player_mana: usize) -> Vec<Spell> {
        let active_names: Vec<&str> = self.spells.iter().map(|s| s.name).collect();
        SPELLS
            .iter()
            .filter(|possible| {
                possible.cost <= player_mana && !active_names.contains(&possible.name)
            })
            .cloned()
            .collect()
    }
}

#[derive(Clone, Debug)]
struct Spell {
    name: &'static str,
    cost: usize,
    damage: usize,
    healing: usize,
    armor: usize,
    mana: usize,
    instant: bool,
    duration: usize,
}

impl Spell {
    fn apply_effects(&mut self, player: &mut Character, boss: &mut Character) {
        if self.damage > 0 {
            boss.hp = boss.hp.checked_sub(self.damage).unwrap_or(0);
        }
        if self.healing > 0 {
            player.hp += self.healing;
        }
        if self.mana > 0 {
            player.mana += self.mana;
        }
        if !self.instant && self.duration > 0 {
            self.duration -= 1;
        }
    }
}

const DEFAULT: Spell = Spell {
    name: "",
    cost: 0,
    damage: 0,
    healing: 0,
    armor: 0,
    mana: 0,
    instant: false,
    duration: 0,
};

const SPELLS: [Spell; 5] = [
    Spell {
        name: "Magic Missile",
        cost: 53,
        damage: 4,
        instant: true,
        ..DEFAULT
    },
    Spell {
        name: "Drain",
        cost: 73,
        damage: 2,
        healing: 2,
        instant: true,
        ..DEFAULT
    },
    Spell {
        name: "Shield",
        cost: 113,
        armor: 7,
        duration: 6,
        ..DEFAULT
    },
    Spell {
        name: "Poison",
        cost: 173,
        damage: 3,
        duration: 6,
        ..DEFAULT
    },
    Spell {
        name: "Recharge",
        cost: 229,
        mana: 101,
        duration: 5,
        ..DEFAULT
    },
];
