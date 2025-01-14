use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashSet;

#[aoc_generator(day24)]
fn parse(input: &str) -> Option<Vec<u64>> {
    input
        .lines()
        .map(|s| s.parse::<u64>())
        .collect::<Result<_, _>>()
        .ok()
}

fn get_combinations(packages: &[u64], volume: u64) -> Vec<HashSet<u64>> {
    if packages.is_empty() || packages[0] > volume {
        return Vec::new();
    }
    let package = packages[0];
    if package == volume {
        return vec![HashSet::from([package])];
    }
    let mut res = get_combinations(&packages[1..], volume);
    for mut combo in get_combinations(&packages[1..], volume - package) {
        combo.insert(package);
        res.push(combo);
    }
    res
}

#[aoc(day24, part1)]
fn part1(input: &[u64]) -> Option<u64> {
    let target = input.iter().sum::<u64>() / 3;
    let mut combos = get_combinations(input, target);
    combos.sort_by_key(|s| (s.len(), s.iter().product::<u64>()));
    for (i, combo) in combos.iter().enumerate() {
        for combo2 in &combos[i + 1..] {
            if combo.is_disjoint(combo2) {
                return Some(combo.iter().product());
            }
        }
    }
    None
}

#[aoc(day24, part2)]
fn part2(input: &[u64]) -> Option<u64> {
    let target = input.iter().sum::<u64>() / 4;
    let mut combos = get_combinations(input, target);
    combos.sort_by_key(|s| (s.len(), s.iter().product::<u64>()));
    for (i, combo) in combos.iter().enumerate() {
        let mut flag = false;
        for combo2 in &combos[i + 1..] {
            if combo.is_disjoint(combo2) {
                if flag {
                    return Some(combo.iter().product());
                } else {
                    flag = true;
                }
            }
        }
    }
    None
}
