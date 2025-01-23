use std::cmp::max;
use std::fs::File;
use std::io::{BufRead, BufReader, self};

type Object = (i16, i16, i16);
type Shop = Vec<Object>;

struct Character {
    health_points: i16,
    damage: i16,
    armor_points: i16,
}

pub fn part1() -> i16 {
    let (weapons, armors, rings) = get_shop();
    let mut player = build_character(100, 0, 0);
    let boss = get_boss_characteristics().expect("Failed to create boss!");
    let mut objects: Vec<Object> = get_all_possibilities(&weapons, &armors, &rings);
    objects.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    for object in objects {
        player.damage = object.1;
        player.armor_points = object.2;
        if fight(&player, &boss) {
            return object.0;
        }
        player.damage = 0;
        player.armor_points = 0;
    }

    0
}

pub fn part2() -> i16 {
    let (weapons, armors, rings) = get_shop();
    let mut player = build_character(100, 0, 0);
    let boss = get_boss_characteristics().expect("Failed to create boss!");
    let mut objects: Vec<Object> = get_all_possibilities(&weapons, &armors, &rings);
    objects.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());
    for object in objects {
        player.damage = object.1;
        player.armor_points = object.2;
        if !fight(&player, &boss) {
            return object.0;
        }
        player.damage = 0;
        player.armor_points = 0;
    }

    0
}

fn get_all_possibilities(weapons: &Shop, armors: &Shop, rings: &Shop) -> Vec<Object> {
    let mut possibilities: Vec<Object> = Vec::new();
    for weapon in weapons {
        let mut initial_object = (0, 0, 0);
        initial_object.0 += weapon.0;
        initial_object.1 += weapon.1;
        initial_object.2 += weapon.2;
        possibilities.push(initial_object);
        for armor in armors {
            let mut second_object = initial_object;
            second_object.0 += armor.0;
            second_object.1 += armor.1;
            second_object.2 += armor.2;
            possibilities.push(second_object);
            for (i, ring) in rings.iter().enumerate() {
                let mut third_object = second_object;
                third_object.0 += ring.0;
                third_object.1 += ring.1;
                third_object.2 += ring.2;
                possibilities.push(third_object);
                for j in i..rings.len() {
                    let mut fourth_object = third_object;
                    fourth_object.0 += rings[j].0;
                    fourth_object.1 += rings[j].1;
                    fourth_object.2 += rings[j].2;
                    possibilities.push(fourth_object);
                }
            }
        }
        for (i, ring) in rings.iter().enumerate() {
            let mut fifth_object = initial_object;
            fifth_object.0 += ring.0;
            fifth_object.1 += ring.1;
            fifth_object.2 += ring.2;
            possibilities.push(fifth_object);
            for j in i..rings.len() {
                let mut sixth_object = fifth_object;
                sixth_object.0 += rings[j].0;
                sixth_object.1 += rings[j].1;
                sixth_object.2 += rings[j].2;
                possibilities.push(sixth_object);
            }
        }
    }
    possibilities
}

fn fight(player: &Character, boss: &Character) -> bool {
    let mut p_health = player.health_points;
    let mut b_health = boss.health_points;
    while p_health > 0 {
        b_health -= max(1, player.damage - boss.armor_points);
        if b_health <= 0 {
            return true;
        }
        p_health -= max(1, boss.damage - player.armor_points);
    }
    false
}

fn get_shop() -> (Shop, Shop, Shop) {
    let weapons: Shop = vec![(8, 4, 0), (10, 5, 0), (25, 6, 0), (40, 7, 0), (74, 8, 0)];
    let armors: Shop = vec![(13, 0, 1), (31, 0, 2), (53, 0, 3), (75, 0, 4), (102, 0, 5)];
    let rings: Shop = vec![(25, 1, 0), (50, 2, 0), (100, 3, 0), (20, 0, 1), (40, 0, 2), (80, 0, 3)];

    (weapons, armors, rings)
}

fn build_character(health_points: i16, damage: i16, armor_points: i16) -> Character {
    Character {
        health_points,
        damage,
        armor_points
    }
}

fn get_boss_characteristics() -> io::Result<Character> {
    let file = File::open("ressources/year2015/day21.txt")?;
    let reader = BufReader::new(file);
    let mut health_points = 0;
    let mut damage = 0;
    let mut armor_points = 0;

    for line in reader.lines() {
        let line = line?;
        if let Some(health) = line.strip_prefix("Hit Points:") {
            health_points = health.trim().parse().unwrap();
        } else if let Some(dmg) = line.strip_prefix("Damage:") {
            damage = dmg.trim().parse().unwrap();
        } else if let Some(armor) = line.strip_prefix("Armor:") {
            armor_points = armor.trim().parse().unwrap();
        }
    }

    Ok(Character {
        health_points,
        damage,
        armor_points,
    })
}