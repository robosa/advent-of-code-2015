use aoc_runner_derive::aoc;

fn step(level: i32, c: char) -> i32 {
    match c {
        '(' => level + 1,
        _ => level - 1,
    }
}

#[aoc(day1, part1)]
fn part1(input: &str) -> i32 {
    input.chars().fold(0, step)
}

#[aoc(day1, part2)]
fn part2(input: &str) -> usize {
    1 + input
        .chars()
        .scan(0, |level, c| {
            *level = step(*level, c);
            Some(*level)
        })
        .take_while(|&level| level >= 0)
        .count()
}
