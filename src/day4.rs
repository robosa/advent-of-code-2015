use aoc_runner_derive::aoc;

fn find_md5(key: &str, pattern: &str) -> Option<u32> {
    (0..).find(|i| format!("{:x}", md5::compute(format!("{}{}", key, i))).starts_with(pattern))
}

#[aoc(day4, part1)]
fn part1(input: &str) -> Option<u32> {
    find_md5(input, "00000")
}

#[aoc(day4, part2)]
fn part2(input: &str) -> Option<u32> {
    find_md5(input, "000000")
}
