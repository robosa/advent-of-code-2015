use aoc_runner_derive::aoc;
use std::iter::{once, successors};

fn look_and_say(input: &[char]) -> Vec<char> {
    input
        .chunk_by(|a, b| a == b)
        .flat_map(|ch| once(char::from_digit(ch.len() as u32, 10).unwrap()).chain(once(ch[0])))
        .collect()
}

fn get_length(input: Vec<char>, n: usize) -> Option<usize> {
    successors(Some(input), |s| Some(look_and_say(s)))
        .nth(n)
        .map(|s| s.len())
}

#[aoc(day10, part1)]
fn part1(input: &str) -> Option<usize> {
    get_length(input.chars().collect(), 40)
}

#[aoc(day10, part2)]
fn part2(input: &str) -> Option<usize> {
    get_length(input.chars().collect(), 50)
}

