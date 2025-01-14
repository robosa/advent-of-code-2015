use aoc_runner_derive::aoc;
use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, newline, space1, u16, u8},
    multi::separated_list1,
    sequence::{delimited, pair, separated_pair},
    IResult,
};

type Compound<'a> = (&'a str, u8);

fn parse_compound(input: &str) -> IResult<&str, Compound> {
    separated_pair(alpha1, tag(": "), u8)(input)
}

fn parse_sue_number(input: &str) -> IResult<&str, u16> {
    delimited(pair(alpha1, space1), u16, tag(": "))(input)
}

fn parse_sue(input: &str) -> IResult<&str, (u16, Vec<Compound>)> {
    pair(parse_sue_number, separated_list1(tag(", "), parse_compound))(input)
}

fn parse_sue_list(input: &str) -> Vec<(u16, Vec<Compound>)> {
    separated_list1(newline, parse_sue)(input).unwrap().1
}

fn find_sue(input: &str, compound_checker: fn(&Compound) -> bool) -> Option<u16> {
    parse_sue_list(input)
        .iter()
        .find_map(|(i, sue)| sue.iter().all(compound_checker).then_some(*i))
}

#[aoc(day16, part1)]
fn part1(input: &str) -> Option<u16> {
    find_sue(input, |&compound| {
        matches!(
            compound,
            ("children", 3)
                | ("cats", 7)
                | ("samoyeds", 2)
                | ("pomeranians", 3)
                | ("akitas", 0)
                | ("vizslas", 0)
                | ("goldfish", 5)
                | ("trees", 3)
                | ("cars", 2)
                | ("perfumes", 1)
        )
    })
}

#[aoc(day16, part2)]
fn part2(input: &str) -> Option<u16> {
    find_sue(input, |&compound| match compound {
        ("children", 3)
        | ("samoyeds", 2)
        | ("akitas", 0)
        | ("vizslas", 0)
        | ("cars", 2)
        | ("perfumes", 1) => true,
        ("cats", v) if v > 7 => true,
        ("pomeranians", v) if v < 3 => true,
        ("goldfish", v) if v < 5 => true,
        ("trees", v) if v > 3 => true,
        _ => false,
    })
}
