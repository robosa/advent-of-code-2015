use aoc_runner_derive::aoc;
use std::collections::HashSet;

fn step((x, y): (i32, i32), c: char) -> (i32, i32) {
    match c {
        '^' => (x, y - 1),
        '<' => (x - 1, y),
        'v' => (x, y + 1),
        _ => (x + 1, y),
    }
}

#[aoc(day3, part1)]
fn part1(input: &str) -> usize {
    input
        .chars()
        .scan((0, 0), |pos, c| {
            *pos = step(*pos, c);
            Some(*pos)
        })
        .collect::<HashSet<_>>()
        .len()
}

#[aoc(day3, part2)]
fn part2(input: &str) -> usize {
    input
        .chars()
        .enumerate()
        .scan([(0, 0), (0, 0)], |pos, (i, c)| {
            if i % 2 == 0 {
                pos[0] = step(pos[0], c);
            } else {
                pos[1] = step(pos[1], c);
            };
            Some(*pos)
        })
        .flatten()
        .collect::<HashSet<_>>()
        .len()
}
