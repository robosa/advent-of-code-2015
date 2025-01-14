use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day17)]
fn parse(input: &str) -> Option<Vec<u8>> {
    input
        .lines()
        .map(|s| s.parse::<u8>())
        .collect::<Result<_, _>>()
        .ok()
}

fn count_combos(containers: &[u8], volume: u8, count: usize, counts: &mut Vec<usize>) -> usize {
    if volume == 0 {
        counts[count] += 1;
        return 1;
    }
    let Some(container) = containers.first() else {
        return 0;
    };
    let skip = count_combos(&containers[1..], volume, count, counts);
    if container > &volume {
        return skip;
    }
    skip + count_combos(&containers[1..], volume - container, count + 1, counts)
}

#[aoc(day17, part1)]
fn part1(input: &[u8]) -> usize {
    count_combos(input, 150, 0, &mut vec![0; input.len() + 1])
}

#[aoc(day17, part2)]
fn part2(input: &[u8]) -> Option<usize> {
    let mut counts = vec![0; input.len() + 1];
    count_combos(input, 150, 0, &mut counts);
    counts.into_iter().find(|&n| n > 0)
}
