use aoc_runner_derive::aoc;

#[aoc(day8, part1)]
fn part1(input: &str) -> usize {
    let mut res = 0;
    let mut char_iter = input.lines().flat_map(|line| line.chars());
    while let Some(c) = char_iter.next() {
        match c {
            '"' => res += 1,
            '\\' => match char_iter.next() {
                Some('x') => {
                    res += 3;
                    char_iter.next();
                    char_iter.next();
                }
                _ => res += 1,
            },
            _ => {}
        }
    }
    res
}

#[aoc(day8, part2)]
fn part2(input: &str) -> usize {
    input
        .lines()
        .map(|line| line.chars().filter(|&c| c == '"' || c == '\\').count() + 2)
        .sum()
}
