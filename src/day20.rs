use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day20)]
fn parse(input: &str) -> Option<usize> {
    input.parse().ok()
}

#[aoc(day20, part1)]
fn part1(target: &usize) -> Option<usize> {
    let n = target / 10;
    let mut gifts = vec![0; n];
    for i in 1..n {
        for j in (i..n).step_by(i) {
            gifts[j] += i;
        }
    }
    gifts.iter().position(|&g| g * 10 >= *target)
}

#[aoc(day20, part2)]
fn part2(target: &usize) -> Option<usize> {
    let n = target / 10;
    let mut gifts = vec![0; n];
    for i in 1..n {
        for j in (i..n.min(50 * i + 1)).step_by(i) {
            gifts[j] += i;
        }
    }
    gifts.iter().position(|&g| g * 11 >= *target)
}
