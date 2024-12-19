use aoc_runner_derive::{aoc, aoc_generator};
use nom::{
    bytes::complete::tag,
    character::complete::{newline, u32},
    combinator::map,
    multi::separated_list1,
    IResult,
};

struct GiftBox {
    l: u32,
    w: u32,
    h: u32,
}

impl GiftBox {
    fn get_paper(&self) -> u32 {
        let a = [self.l * self.w, self.w * self.h, self.h * self.l];
        2 * a.iter().sum::<u32>() + a.iter().min().unwrap()
    }

    fn get_ribbon(&self) -> u32 {
        let mut a = [self.l, self.w, self.h];
        a.sort();
        2 * (a[0] + a[1]) + self.l * self.w * self.h
    }
}

fn parse_box(input: &str) -> IResult<&str, GiftBox> {
    map(separated_list1(tag("x"), u32), |dims| GiftBox {
        l: dims[0],
        w: dims[1],
        h: dims[2],
    })(input)
}

#[aoc_generator(day2)]
fn parse(input: &str) -> Vec<GiftBox> {
    separated_list1(newline, parse_box)(input).unwrap().1
}

#[aoc(day2, part1)]
fn part1(input: &[GiftBox]) -> u32 {
    input.iter().map(GiftBox::get_paper).sum()
}

#[aoc(day2, part2)]
fn part2(input: &[GiftBox]) -> u32 {
    input.iter().map(GiftBox::get_ribbon).sum()
}
