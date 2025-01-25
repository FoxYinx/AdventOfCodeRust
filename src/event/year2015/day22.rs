use std::cmp::{max, Reverse};
use std::collections::BinaryHeap;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

#[derive(Clone, Eq, PartialEq)]
struct State {
    player: Character,
    boss: Character,
    effect: [u8; 3],
    mana_spent: i32,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        Reverse(self.mana_spent).cmp(&Reverse(other.mana_spent))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Clone, Eq, PartialEq)]
struct Character {
    health_points: i16,
    damage: i16,
    armor_points: i16,
    mana: i32,
}

pub fn part1() -> i32 {
    let player = Character {
        health_points: 50,
        damage: 0,
        armor_points: 0,
        mana: 500
    };
    let boss = get_boss_characteristics().expect("Unable to make boss!");
    dijkstra(player, boss, false)
}

pub fn part2() -> i32 {
    let player = Character {
        health_points: 50,
        damage: 0,
        armor_points: 0,
        mana: 500
    };
    let boss = get_boss_characteristics().expect("Unable to make boss!");
    dijkstra(player, boss, true)
}

fn dijkstra(player: Character, boss: Character, hard: bool) -> i32 {
    let mut heap = BinaryHeap::new();
    heap.push(State {
        player,
        boss,
        effect: [0; 3],
        mana_spent: 0,
    });

    while let Some(state) = heap.pop() {
        if state.boss.health_points <= 0 {
            return state.mana_spent;
        }

        for spell in &[Spell::Missile, Spell::Drain, Spell::Shield, Spell::Poison, Spell::Recharge] {
            if state.player.mana < spell.mana_cost() {
                continue;
            }
            let mut new_state = state.clone();
            
            if hard {
                new_state.player.health_points -= 1;
            }
            
            new_state.player.mana -= spell.mana_cost();
            new_state.mana_spent += spell.mana_cost();
            

            apply_effects(&mut new_state.player, &mut new_state.boss, &mut new_state.effect);
            if new_state.boss.health_points <= 0 {
                return new_state.mana_spent;
            }

            cast_spell(&mut new_state.player, &mut new_state.effect, spell, &mut new_state.boss);
            if new_state.boss.health_points <= 0 {
                return new_state.mana_spent;
            }

            apply_effects(&mut new_state.player, &mut new_state.boss, &mut new_state.effect);
            if new_state.boss.health_points <= 0 {
                return new_state.mana_spent;
            }

            new_state.player.health_points -= max(1, new_state.boss.damage - new_state.player.armor_points);
            if new_state.player.health_points > 0 {
                heap.push(new_state);
            }
        }
    }

    -1
}

fn apply_effects(player: &mut Character, boss: &mut Character, effect: &mut [u8; 3]) {
    if effect[0] > 0 {
        effect[0] -= 1;
        if effect[0] == 0 {
            player.armor_points = 0;
        }
    }
    if effect[1] > 0 {
        effect[1] -= 1;
        boss.health_points -= 3;
    }
    if effect[2] > 0 {
        effect[2] -= 1;
        player.mana += 101;
    }
}

fn cast_spell(player: &mut Character, effect: &mut [u8; 3], spell: &Spell, boss: &mut Character) {
    match spell {
        Spell::Missile => {
            boss.health_points -= 4;
        },
        Spell::Drain => {
            boss.health_points -= 2;
            player.health_points += 2;
        }
        Spell::Shield => {
            effect[0] = 6;
            player.armor_points = 7;
        }
        Spell::Poison => {
            effect[1] = 6;
        }
        Spell::Recharge => {
            effect[2] = 5;
        }
    }
}

fn get_boss_characteristics() -> io::Result<Character> {
    let file = File::open("ressources/year2015/day22.txt")?;
    let reader = BufReader::new(file);
    let mut health_points = 0;
    let mut damage = 0;

    for line in reader.lines() {
        let line = line?;
        if let Some(health) = line.strip_prefix("Hit Points:") {
            health_points = health.trim().parse().unwrap();
        } else if let Some(dmg) = line.strip_prefix("Damage:") {
            damage = dmg.trim().parse().unwrap();
        } 
    }

    Ok(Character {
        health_points,
        damage,
        armor_points: 0,
        mana: 0
    })
}

enum Spell {
    Missile,
    Drain,
    Shield,
    Poison,
    Recharge
}

impl Spell {
    fn mana_cost(&self) -> i32 {
        match *self {
            Spell::Missile => 53,
            Spell::Drain => 73,
            Spell::Shield => 113,
            Spell::Poison => 173,
            Spell::Recharge => 229
        }
    }
}