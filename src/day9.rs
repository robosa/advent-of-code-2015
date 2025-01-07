use aoc_runner_derive::aoc;
use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, u32},
    sequence::tuple,
    IResult,
};
use std::collections::{HashMap, HashSet};

fn parse_edge(input: &str) -> IResult<&str, (&str, &str, &str, &str, u32)> {
    tuple((alpha1, tag(" to "), alpha1, tag(" = "), u32))(input)
}

struct Graph<'a> {
    vertices: HashSet<&'a str>,
    edges: HashMap<(&'a str, &'a str), u32>,
}

impl<'a> Graph<'a> {
    fn new(input: &'a str) -> Self {
        let mut vertices = HashSet::new();
        let mut edges = HashMap::new();
        input
            .lines()
            .filter_map(|line| parse_edge(line).ok())
            .for_each(|(_, (v1, _, v2, _, d))| {
                vertices.insert(v1);
                vertices.insert(v2);
                edges.insert((v1, v2), d);
                edges.insert((v2, v1), d);
            });
        Self { vertices, edges }
    }

    fn dists(&self) -> impl Iterator<Item = u32> + '_ {
        let n = self.vertices.len();
        self.vertices.iter().permutations(n).map(move |perm| {
            perm.into_iter()
                .tuple_windows()
                .fold(0, |acc, (&v1, &v2)| acc + self.edges[&(v1, v2)])
        })
    }
}

#[aoc(day9, part1)]
fn part1(input: &str) -> Option<u32> {
    Graph::new(input).dists().min()
}

#[aoc(day9, part2, new)]
fn part2(input: &str) -> Option<u32> {
    Graph::new(input).dists().max()
}
