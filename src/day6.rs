use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use nom::{
    bytes::complete::{tag, take_till},
    character::complete::{newline, u32},
    combinator::map,
    multi::separated_list1,
    sequence::{pair, separated_pair},
    IResult,
};

#[derive(Clone, Copy)]
enum Verb {
    Toggle,
    TurnOn,
    TurnOff,
}

type Area = ((u32, u32), (u32, u32));
type Instruction = (Verb, Area);

fn parse_verb(input: &str) -> IResult<&str, Verb> {
    map(take_till(|c: char| c.is_ascii_digit()), |s| match s {
        "toggle " => Verb::Toggle,
        "turn on " => Verb::TurnOn,
        "turn off " => Verb::TurnOff,
        _ => panic!(),
    })(input)
}

fn parse_pos(input: &str) -> IResult<&str, (u32, u32)> {
    separated_pair(u32, tag(","), u32)(input)
}

fn parse_area(input: &str) -> IResult<&str, Area> {
    separated_pair(parse_pos, tag(" through "), parse_pos)(input)
}

fn parse_instructions(input: &str) -> IResult<&str, Vec<Instruction>> {
    separated_list1(newline, pair(parse_verb, parse_area))(input)
}

trait LightGrid {
    fn toggle(&mut self, pos: (usize, usize));
    fn turn_on(&mut self, pos: (usize, usize));
    fn turn_off(&mut self, pos: (usize, usize));
    fn count_lights(&self) -> usize;

    fn apply(&mut self, instructions: &[Instruction]) -> usize {
        for &(verb, ((fx, fy), (tx, ty))) in instructions {
            let func = match verb {
                Verb::Toggle => Self::toggle,
                Verb::TurnOn => Self::turn_on,
                Verb::TurnOff => Self::turn_off,
            };
            for (x, y) in (fx..=tx).cartesian_product(fy..=ty) {
                func(self, (x as usize, y as usize));
            }
        }
        self.count_lights()
    }
}

struct Grid {
    data: Vec<Vec<bool>>,
}

impl Grid {
    fn new() -> Self {
        Self {
            data: vec![vec![false; 1000]; 1000],
        }
    }
}

impl LightGrid for Grid {
    fn toggle(&mut self, (x, y): (usize, usize)) {
        self.data[x][y] = !self.data[x][y]
    }
    fn turn_on(&mut self, (x, y): (usize, usize)) {
        self.data[x][y] = true
    }
    fn turn_off(&mut self, (x, y): (usize, usize)) {
        self.data[x][y] = false
    }
    fn count_lights(&self) -> usize {
        self.data
            .iter()
            .flat_map(|row| row.iter().filter(|&&l| l))
            .count()
    }
}

struct Grid2 {
    data: Vec<Vec<u32>>,
}

impl Grid2 {
    fn new() -> Self {
        Self {
            data: vec![vec![0; 1000]; 1000],
        }
    }
}

impl LightGrid for Grid2 {
    fn toggle(&mut self, (x, y): (usize, usize)) {
        self.data[x][y] += 2
    }
    fn turn_on(&mut self, (x, y): (usize, usize)) {
        self.data[x][y] += 1
    }
    fn turn_off(&mut self, (x, y): (usize, usize)) {
        self.data[x][y] = self.data[x][y].checked_sub(1).unwrap_or_default()
    }
    fn count_lights(&self) -> usize {
        self.data.iter().flat_map(|row| row.iter()).sum::<u32>() as usize
    }
}

#[aoc_generator(day6)]
fn parse(input: &str) -> Vec<Instruction> {
    parse_instructions(input).unwrap().1
}

#[aoc(day6, part1)]
fn part1(input: &[Instruction]) -> usize {
    Grid::new().apply(input)
}

#[aoc(day6, part2)]
fn part2(input: &[Instruction]) -> usize {
    Grid2::new().apply(input)
}
