use aoc_runner_derive::aoc;
use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, newline},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};
use std::collections::{HashMap, HashSet};

fn parse_repl(input: &str) -> IResult<&str, (&str, &str)> {
    separated_pair(alpha1, tag(" => "), alpha1)(input)
}

fn parse_repl_list(input: &str) -> IResult<&str, Vec<(&str, &str)>> {
    separated_list1(newline, parse_repl)(input)
}

fn parse_input(input: &str) -> (HashMap<&str, Vec<&str>>, &str) {
    let (repl_list, mollecule) = separated_pair(parse_repl_list, tag("\n\n"), alpha1)(input)
        .unwrap()
        .1;
    let mut repl_map: HashMap<&str, Vec<&str>> = HashMap::new();
    for (k, v) in repl_list.iter() {
        repl_map.entry(k).or_default().push(v);
    }
    (repl_map, mollecule)
}

#[aoc(day19, part1)]
fn part1(input: &str) -> usize {
    let mut molecule_set = HashSet::new();
    let (repl_map, molecule) = parse_input(input);
    let patterns: Vec<&str> = repl_map.keys().copied().collect();
    for i in 0..molecule.len() {
        let (s1, s2) = molecule.split_at(i);
        for pattern in patterns.iter() {
            if s2.starts_with(pattern) {
                for repl in &repl_map[pattern] {
                    molecule_set.insert(s1.to_owned() + repl + s2.strip_prefix(pattern).unwrap());
                }
            }
        }
    }
    molecule_set.len()
}

#[aoc(day19, part2)]
fn part2(input: &str) -> usize {
    let mut chars = input.lines().last().unwrap().chars().peekable();
    let mut count = 0;
    while let Some(c) = chars.next() {
        match (c, chars.peek()) {
            ('R', Some('n')) | ('A', Some('r')) => {
                chars.next();
            }
            ('Y', _) => {
                count -= 1;
            }
            (_, Some(c2)) if c2.is_lowercase() => {
                count += 1;
                chars.next();
            }
            _ => {
                count += 1;
            }
        }
    }
    count - 1
}
