use aoc_runner_derive::aoc;
use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, i32, space1},
    combinator::map,
    sequence::{delimited, separated_pair, tuple},
    IResult,
};
use std::collections::{HashMap, HashSet};

fn parse_happiness_change(input: &str) -> IResult<&str, i32> {
    map(
        separated_pair(alt((tag("gain"), tag("lose"))), space1, i32),
        |(verb, val)| if verb == "lose" { -val } else { val },
    )(input)
}

fn parse_edge(input: &str) -> IResult<&str, (&str, i32, &str)> {
    tuple((
        alpha1,
        delimited(
            tag(" would "),
            parse_happiness_change,
            tag(" happiness units by sitting next to "),
        ),
        alpha1,
    ))(input)
}

struct Graph<'a> {
    vertices: HashSet<&'a str>,
    edges: HashMap<(&'a str, &'a str), i32>,
}

impl<'a> Graph<'a> {
    fn new(input: &'a str) -> Self {
        let mut vertices = HashSet::new();
        let mut edges = HashMap::new();
        input
            .lines()
            .filter_map(|line| parse_edge(line).ok())
            .for_each(|(_, (v1, val, v2))| {
                vertices.insert(v1);
                vertices.insert(v2);
                *edges.entry((v1, v2)).or_default() += val;
                *edges.entry((v2, v1)).or_default() += val;
            });
        Self { vertices, edges }
    }

    fn scores(&self) -> impl Iterator<Item = i32> + '_ {
        let n = self.vertices.len();
        self.vertices.iter().permutations(n).map(move |perm| {
            perm.into_iter()
                .cycle()
                .take(n + 1)
                .tuple_windows()
                .fold(0, |acc, (&v1, &v2)| acc + self.edges[&(v1, v2)])
        })
    }
    fn scores_with_me(&self) -> impl Iterator<Item = i32> + '_ {
        let n = self.vertices.len();
        self.vertices.iter().permutations(n).map(move |perm| {
            perm.into_iter()
                .tuple_windows()
                .fold(0, |acc, (&v1, &v2)| acc + self.edges[&(v1, v2)])
        })
    }
}

#[aoc(day13, part1)]
fn part1(input: &str) -> Option<i32> {
    Graph::new(input).scores().max()
}

#[aoc(day13, part2)]
fn part2(input: &str) -> Option<i32> {
    Graph::new(input).scores_with_me().max()
}

