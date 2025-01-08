use aoc_runner_derive::aoc;
use std::iter::once;

fn add_char(c: char, carry: u8) -> (Option<char>, u8) {
    match (c, carry) {
        (' ', 1) => (Some('a'), 0),
        (' ', _) => (None, 0),
        ('z', 1) => (Some('a'), 1),
        (c, 1) if c == 'h' || c == 'k' || c == 'n' => (char::from_u32(c as u32 + 2), 0),
        (c, 1) => (char::from_u32(c as u32 + 1), 0),
        (c, _) => (Some(c), 0),
    }
}

fn update_password(input: &str) -> String {
    input
        .chars()
        .chain(once(' '))
        .scan(1, |carry, c| {
            let (nc, ncarry) = add_char(c, *carry);
            *carry = ncarry;
            nc
        })
        .collect()
}

fn check_password(input: &str) -> bool {
    let mut current = None;
    let mut pairs = 0;
    let mut seq = 0;
    let mut last_pair = false;
    let mut seq_ok = false;
    for c in input.chars() {
        let prev = current;
        current = Some(c as u32);
        if current == prev && !last_pair {
            pairs += 1;
            last_pair = true;
            seq = 0;
            continue;
        }
        last_pair = false;
        if current == prev.map(|s| s - 1) {
            seq += 1;
            seq_ok |= seq == 2;
            continue;
        }
        seq = 0;
    }
    seq_ok && pairs >= 2
}

fn next_password(input: &str) -> String {
    let mut pwd = update_password(&input.chars().rev().collect::<String>());
    while !check_password(&pwd) {
        pwd = update_password(&pwd);
    }
    pwd.chars().rev().collect()
}

#[aoc(day11, part1)]
fn part1(input: &str) -> String {
    next_password(input)
}

#[aoc(day11, part2)]
fn part2(input: &str) -> String {
    next_password(&next_password(input))
}
