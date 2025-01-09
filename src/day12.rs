use aoc_runner_derive::aoc;
use nom::{
    branch::alt,
    character::complete::{alpha0, char, i32},
    combinator::map,
    multi::separated_list1,
    sequence::{delimited, pair, preceded},
    IResult,
};

#[derive(PartialEq)]
enum Value<'a> {
    Str(&'a str),
    Int(i32),
    Arr(Vec<Value<'a>>),
    Obj(Vec<Value<'a>>),
}

fn parse_string(input: &str) -> IResult<&str, &str> {
    delimited(char('"'), alpha0, char('"'))(input)
}

fn parse_array(input: &str) -> IResult<&str, Vec<Value>> {
    delimited(
        char('['),
        separated_list1(char(','), parse_value),
        char(']'),
    )(input)
}

fn parse_obj(input: &str) -> IResult<&str, Vec<Value>> {
    delimited(
        char('{'),
        separated_list1(
            char(','),
            preceded(pair(parse_string, char(':')), parse_value),
        ),
        char('}'),
    )(input)
}

fn parse_value(input: &str) -> IResult<&str, Value> {
    alt((
        map(parse_string, Value::Str),
        map(i32, Value::Int),
        map(parse_array, Value::Arr),
        map(parse_obj, Value::Obj),
    ))(input)
}

fn calc_arr(vals: &[Value], with_red: bool) -> i32 {
    vals.iter().map(|val| calc_value(val, with_red)).sum()
}

fn calc_obj(vals: &[Value], with_red: bool) -> i32 {
    if with_red && vals.contains(&Value::Str("red")) {
        return 0;
    }
    calc_arr(vals, with_red)
}

fn calc_value(val: &Value, with_red: bool) -> i32 {
    match val {
        Value::Str(_) => 0,
        Value::Int(i) => *i,
        Value::Arr(vals) => calc_arr(vals, with_red),
        Value::Obj(vals) => calc_obj(vals, with_red),
    }
}

#[aoc(day12, part1)]
fn part1(input: &str) -> i32 {
    calc_value(&parse_value(input).unwrap().1, false)
}

#[aoc(day12, part2)]
fn part2(input: &str) -> i32 {
    calc_value(&parse_value(input).unwrap().1, true)
}

