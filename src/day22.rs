use aoc_runner_derive::{aoc, aoc_generator};
use nom::{
    bytes::complete::tag,
    character::complete::u16,
    sequence::{preceded, tuple},
    IResult,
};
use std::collections::{BinaryHeap, HashSet};

fn parse_stats(input: &str) -> IResult<&str, (u16, u16)> {
    tuple((
        preceded(tag("Hit Points: "), u16),
        preceded(tag("\nDamage: "), u16),
    ))(input)
}

enum Spell {
    MagicMissile,
    Drain,
    Shield,
    Poison,
    Recharge,
}

const SPELLS: [(Spell, u16); 5] = [
    (Spell::MagicMissile, 53),
    (Spell::Drain, 73),
    (Spell::Shield, 113),
    (Spell::Poison, 173),
    (Spell::Recharge, 229),
];

enum Outcome {
    Victory,
    Defeat,
    Continue,
}

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
struct State {
    player_hp: u16,
    player_mana: u16,
    boss_hp: u16,
    boss_damage: u16,
    shield: u8,
    poison: u8,
    recharge: u8,
}

impl State {
    fn new(boss_hp: u16, boss_damage: u16) -> Self {
        State {
            player_hp: 50,
            player_mana: 500,
            boss_hp,
            boss_damage,
            shield: 0,
            poison: 0,
            recharge: 0,
        }
    }

    fn player_action(&self, spell: Spell, cost: u16) -> Option<Self> {
        if self.player_mana < cost {
            return None;
        }
        match spell {
            Spell::MagicMissile => Some(State {
                player_mana: self.player_mana - cost,
                boss_hp: self.boss_hp.saturating_sub(4),
                ..*self
            }),
            Spell::Drain => Some(State {
                player_mana: self.player_mana - cost,
                player_hp: self.player_hp + 2,
                boss_hp: self.boss_hp.saturating_sub(2),
                ..*self
            }),
            Spell::Shield if self.shield == 0 => Some(State {
                player_mana: self.player_mana - cost,
                shield: 3,
                ..*self
            }),
            Spell::Poison if self.poison == 0 => Some(State {
                player_mana: self.player_mana - cost,
                poison: 6,
                ..*self
            }),
            Spell::Recharge if self.recharge == 0 => Some(State {
                player_mana: self.player_mana - cost,
                recharge: 5,
                ..*self
            }),
            _ => None,
        }
    }

    fn apply_effects(&mut self) {
        if self.poison > 0 {
            self.boss_hp = self.boss_hp.saturating_sub(3);
            self.poison -= 1;
        }
        if self.recharge > 0 {
            self.player_mana += 101;
            self.recharge -= 1
        }
    }

    fn turn_outcome(&mut self, hard: bool) -> Outcome {
        self.apply_effects();
        if self.boss_hp == 0 {
            return Outcome::Victory;
        }
        let mut damage = self.boss_damage;
        if self.shield > 0 {
            damage = damage.saturating_sub(7).max(1);
            self.shield -= 1;
        }
        if hard {
            damage += 1
        }
        self.player_hp = self.player_hp.saturating_sub(damage);
        if self.player_hp == 0 {
            return Outcome::Defeat;
        }
        self.apply_effects();
        if self.boss_hp == 0 {
            return Outcome::Victory;
        }
        Outcome::Continue
    }
}

struct StateWithCost(u16, State);

impl PartialEq for StateWithCost {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Eq for StateWithCost {}

impl Ord for StateWithCost {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.0.cmp(&self.0)
    }
}

impl PartialOrd for StateWithCost {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn dijkstra(first_state: &State, hard: bool) -> Option<u16> {
    let mut queue = BinaryHeap::new();
    let mut visited = HashSet::new();
    for (spell, cost) in SPELLS {
        if let Some(new_state) = first_state.player_action(spell, cost) {
            queue.push(StateWithCost(cost, new_state));
        }
    }
    while let Some(StateWithCost(current_cost, mut state)) = queue.pop() {
        if !visited.insert(state) {
            continue;
        }
        match state.turn_outcome(hard) {
            Outcome::Victory => return Some(current_cost),
            Outcome::Defeat => continue,
            Outcome::Continue => {}
        }
        for (spell, cost) in SPELLS {
            if let Some(new_state) = state.player_action(spell, cost) {
                queue.push(StateWithCost(current_cost + cost, new_state));
            }
        }
    }
    None
}

#[aoc_generator(day22)]
fn parse(input: &str) -> Option<State> {
    parse_stats(input).ok().map(|(_, (h, d))| State::new(h, d))
}

#[aoc(day22, part1)]
fn part1(input: &State) -> Option<u16> {
    dijkstra(input, false)
}

#[aoc(day22, part2)]
fn part2(input: &State) -> Option<u16> {
    dijkstra(input, true)
}
