use aoc_runner_derive::aoc;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, newline, u16, u8},
    combinator::map,
    multi::separated_list1,
    sequence::{preceded, separated_pair},
    IResult,
};
use std::collections::HashMap;

#[derive(Clone, Copy)]
enum Op<'a> {
    Value(u16),
    Wire(&'a str),
    And(&'a str, &'a str),
    OneAnd(&'a str),
    Or(&'a str, &'a str),
    LShift(&'a str, u8),
    RShift(&'a str, u8),
    Not(&'a str),
}

fn parse_op(input: &str) -> IResult<&str, Op> {
    alt((
        map(separated_pair(alpha1, tag(" AND "), alpha1), |(a, b)| {
            Op::And(a, b)
        }),
        map(preceded(tag("1 AND "), alpha1), Op::OneAnd),
        map(separated_pair(alpha1, tag(" OR "), alpha1), |(a, b)| {
            Op::Or(a, b)
        }),
        map(separated_pair(alpha1, tag(" LSHIFT "), u8), |(a, b)| {
            Op::LShift(a, b)
        }),
        map(separated_pair(alpha1, tag(" RSHIFT "), u8), |(a, b)| {
            Op::RShift(a, b)
        }),
        map(preceded(tag("NOT "), alpha1), Op::Not),
        map(u16, Op::Value),
        map(alpha1, Op::Wire),
    ))(input)
}

fn parse_connection(input: &str) -> IResult<&str, (Op, &str)> {
    separated_pair(parse_op, tag(" -> "), alpha1)(input)
}

fn parse_circuit(input: &str) -> IResult<&str, Vec<(&str, Op)>> {
    separated_list1(newline, map(parse_connection, |(o, w)| (w, o)))(input)
}

fn compute_circuit<'a>(circuit: &mut HashMap<&'a str, Op<'a>>, wire: &'a str) -> u16 {
    let conn = circuit[wire];
    if let Op::Value(res) = conn {
        return res;
    }
    let value = match conn {
        Op::And(a, b) => compute_circuit(circuit, a) & compute_circuit(circuit, b),
        Op::OneAnd(a) => 1 & compute_circuit(circuit, a),
        Op::Or(a, b) => compute_circuit(circuit, a) | compute_circuit(circuit, b),
        Op::LShift(a, n) => compute_circuit(circuit, a) << n,
        Op::RShift(a, n) => compute_circuit(circuit, a) >> n,
        Op::Not(a) => !compute_circuit(circuit, a),
        Op::Wire(a) => compute_circuit(circuit, a),
        _ => panic!(),
    };
    circuit.insert(wire, Op::Value(value));
    value
}

#[aoc(day7, part1)]
fn part1(input: &str) -> u16 {
    let mut circuit = parse_circuit(input).unwrap().1.into_iter().collect();
    compute_circuit(&mut circuit, "a")
}

#[aoc(day7, part2)]
fn part2(input: &str) -> u16 {
    let mut circuit: HashMap<_, _> = parse_circuit(input).unwrap().1.into_iter().collect();
    let value = compute_circuit(&mut circuit.clone(), "a");
    circuit.insert("b", Op::Value(value));
    compute_circuit(&mut circuit, "a")
}
