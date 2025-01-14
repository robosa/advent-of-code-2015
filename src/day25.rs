use aoc_runner_derive::{aoc, aoc_generator};
use nom::{
    bytes::complete::{tag, take_till},
    character::complete::u64,
    sequence::{preceded, separated_pair},
    IResult,
};
use std::iter::successors;

fn parse_input(input: &str) -> IResult<&str, (u64, u64)> {
    preceded(
        take_till(|c: char| c.is_ascii_digit()),
        separated_pair(u64, tag(", column "), u64),
    )(input)
}

#[aoc_generator(day25)]
fn parse(input: &str) -> Option<(u64, u64)> {
    parse_input(input).ok().map(|(_, p)| p)
}

#[aoc(day25, part1)]
fn part1(&(row, column): &(u64, u64)) -> Option<u64> {
    let n = row + column - 1;
    let count = n * (n - 1) / 2 + column - 1;
    successors(Some(20151125), |code| Some((code * 252533) % 33554393)).nth(count as usize)
}

