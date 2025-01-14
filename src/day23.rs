use aoc_runner_derive::aoc;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, i32, newline, space1},
    combinator::map,
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

struct Instr<'a> {
    code: &'a str,
    register: Option<char>,
    offset: Option<i32>,
}

fn parse_reg_instr(input: &str) -> IResult<&str, Instr> {
    map(
        separated_pair(
            alt((tag("hlf"), tag("tpl"), tag("inc"))),
            space1,
            alt((char('a'), char('b'))),
        ),
        |(code, reg)| Instr {
            code,
            register: Some(reg),
            offset: None,
        },
    )(input)
}

fn parse_jmp(input: &str) -> IResult<&str, Instr> {
    map(separated_pair(tag("jmp"), space1, i32), |(code, offset)| {
        Instr {
            code,
            register: None,
            offset: Some(offset),
        }
    })(input)
}

fn parse_jmp_if(input: &str) -> IResult<&str, Instr> {
    map(
        separated_pair(
            alt((tag("jie"), tag("jio"))),
            space1,
            separated_pair(alt((char('a'), char('b'))), tag(", "), i32),
        ),
        |(code, (reg, offset))| Instr {
            code,
            register: Some(reg),
            offset: Some(offset),
        },
    )(input)
}

fn parse_program(input: &str) -> IResult<&str, Vec<Instr>> {
    separated_list1(newline, alt((parse_reg_instr, parse_jmp, parse_jmp_if)))(input)
}

struct Computer<'a> {
    program: Vec<Instr<'a>>,
    reg_a: u32,
    reg_b: u32,
    current: Option<usize>,
}

impl<'a> Computer<'a> {
    fn new(input: &'a str, reg_a: u32) -> Self {
        Computer {
            program: parse_program(input).unwrap().1,
            reg_a,
            reg_b: 0,
            current: Some(0),
        }
    }

    fn step(&mut self) {
        let Some(idx) = self.current else { return };
        let instr = &self.program[idx];
        let mut offset = 1;
        match (instr.code, instr.register, instr.offset) {
            ("hlf", Some('a'), _) => self.reg_a /= 2,
            ("hlf", Some('b'), _) => self.reg_b /= 2,
            ("tpl", Some('a'), _) => self.reg_a *= 3,
            ("tpl", Some('b'), _) => self.reg_b *= 3,
            ("inc", Some('a'), _) => self.reg_a += 1,
            ("inc", Some('b'), _) => self.reg_b += 1,
            ("jmp", _, Some(o)) => offset = o,
            ("jie", Some('a'), Some(o)) if self.reg_a % 2 == 0 => offset = o,
            ("jie", Some('b'), Some(o)) if self.reg_b % 2 == 0 => offset = o,
            ("jio", Some('a'), Some(o)) if self.reg_a == 1 => offset = o,
            ("jio", Some('b'), Some(o)) if self.reg_b == 1 => offset = o,
            _ => {}
        }
        if offset < 0 {
            self.current = idx.checked_sub(offset.unsigned_abs() as usize)
        } else {
            self.current = Some(idx + offset as usize).take_if(|i| *i < self.program.len());
        }
    }

    fn run_program(&mut self) -> u32 {
        while self.current.is_some() {
            self.step()
        }
        self.reg_b
    }
}

#[aoc(day23, part1)]
fn part1(input: &str) -> u32 {
    Computer::new(input, 0).run_program()
}

#[aoc(day23, part2)]
fn part2(input: &str) -> u32 {
    Computer::new(input, 1).run_program()
}
