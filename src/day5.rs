use aoc_runner_derive::aoc;
use std::collections::HashMap;

fn is_nice1(input: &str) -> bool {
    let mut count_vowels = 0;
    let mut prev = None;
    let mut double = false;
    for c in input.chars() {
        if Some(c) == prev {
            double = true;
        }
        match c {
            'a' | 'e' | 'i' | 'o' | 'u' => {
                count_vowels += 1;
            }
            'b' if prev == Some('a') => {
                return false;
            }
            'd' if prev == Some('c') => {
                return false;
            }
            'q' if prev == Some('p') => {
                return false;
            }
            'y' if prev == Some('x') => {
                return false;
            }
            _ => {}
        };
        prev = Some(c);
    }
    double && count_vowels >= 3
}

fn is_nice2(input: &str) -> bool {
    let mut prev = None;
    let mut prev2 = None;
    let mut pairs = HashMap::new();
    let mut two_pairs = false;
    let mut surround = false;
    for (i, c) in input.chars().enumerate() {
        if Some(c) == prev2 {
            surround = true;
        }
        if let Some(p) = prev {
            if *pairs.entry((p, c)).or_insert(i) < i - 1 {
                two_pairs = true;
            }
        }
        (prev, prev2) = (Some(c), prev);
    }
    surround && two_pairs
}

#[aoc(day5, part1)]
fn part1(input: &str) -> usize {
    input.lines().filter(|s| is_nice1(s)).count()
}

#[aoc(day5, part2)]
fn part2(input: &str) -> usize {
    input.lines().filter(|s| is_nice2(s)).count()
}
