use aoc_runner_derive::{aoc, aoc_generator};
use nom::{
    bytes::complete::tag,
    character::complete::u16,
    sequence::{preceded, tuple},
    IResult,
};
use std::iter::once;

const WEAPONS: [(u16, u16); 5] = [(8, 4), (10, 5), (25, 6), (40, 7), (74, 8)];
const ARMORS: [Option<(u16, u16)>; 6] = [
    None,
    Some((13, 1)),
    Some((31, 2)),
    Some((53, 3)),
    Some((75, 4)),
    Some((102, 5)),
];
const RINGS: [Option<(u16, u16, u16)>; 7] = [
    None,
    Some((25, 1, 0)),
    Some((50, 2, 0)),
    Some((100, 3, 0)),
    Some((20, 0, 1)),
    Some((40, 0, 2)),
    Some((80, 0, 3)),
];

struct Character {
    hp: u16,
    damage: u16,
    armor: u16,
}

fn parse_stats(input: &str) -> IResult<&str, (u16, u16, u16)> {
    tuple((
        preceded(tag("Hit Points: "), u16),
        preceded(tag("\nDamage: "), u16),
        preceded(tag("\nArmor: "), u16),
    ))(input)
}

#[aoc_generator(day21)]
fn parse(input: &str) -> Option<Character> {
    parse_stats(input)
        .ok()
        .map(|(_, (hp, damage, armor))| Character { hp, damage, armor })
}

impl Character {
    fn new() -> Self {
        Character {
            hp: 100,
            damage: 0,
            armor: 0,
        }
    }

    fn equip(
        &mut self,
        weapon: &(u16, u16),
        armor: &Option<(u16, u16)>,
        ring1: &Option<(u16, u16, u16)>,
        ring2: &Option<(u16, u16, u16)>,
    ) -> u16 {
        let (weapon_cost, base_damage) = weapon;
        let (armor_cost, base_armor) = armor.unwrap_or_default();
        let (ring1_cost, ring1_damage, ring1_armor) = ring1.unwrap_or_default();
        let (ring2_cost, ring2_damage, ring2_armor) = ring2.unwrap_or_default();
        self.damage = base_damage + ring1_damage + ring2_damage;
        self.armor = base_armor + ring1_armor + ring2_armor;
        weapon_cost + armor_cost + ring1_cost + ring2_cost
    }

    fn fight(&self, boss: &Character) -> bool {
        let self_real_damage = self.damage.saturating_sub(boss.armor).max(1);
        let boss_real_damage = boss.damage.saturating_sub(self.armor).max(1);
        (boss.hp.div_ceil(self_real_damage) - 1) * boss_real_damage < self.hp
    }

    fn tryouts(&mut self, boss: &Character) -> (u16, u16) {
        let mut best = u16::MAX;
        let mut worst = 0;
        for weapon in &WEAPONS {
            for armor in &ARMORS {
                for (i, ring1) in RINGS.iter().enumerate() {
                    for ring2 in once(&None).chain(RINGS[i + 1..].iter()) {
                        let cost = self.equip(weapon, armor, ring1, ring2);
                        if self.fight(boss) {
                            best = best.min(cost);
                        } else {
                            worst = worst.max(cost);
                        }
                    }
                }
            }
        }
        (best, worst)
    }
}

#[aoc(day21, part1)]
fn part1(boss: &Character) -> u16 {
    Character::new().tryouts(boss).0
}

#[aoc(day21, part2)]
fn part2(boss: &Character) -> u16 {
    Character::new().tryouts(boss).1
}
