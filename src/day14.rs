use aoc_runner_derive::{aoc, aoc_generator};
use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, newline, u32},
    combinator::map,
    multi::separated_list1,
    sequence::tuple,
    IResult,
};

struct Reindeer {
    speed: u32,
    movement: u32,
    cycle: u32,
}

impl Reindeer {
    fn get_distance(&self, duration: u32) -> u32 {
        let (q, r) = (duration / self.cycle, duration % self.cycle);
        self.speed * (q * self.movement + r.min(self.movement))
    }
}

fn parse_reindeer(input: &str) -> IResult<&str, Reindeer> {
    map(
        tuple((
            alpha1,
            tag(" can fly "),
            u32,
            tag(" km/s for "),
            u32,
            tag(" seconds, but then must rest for "),
            u32,
            tag(" seconds."),
        )),
        |(_, _, speed, _, movement, _, rest, _)| Reindeer {
            speed,
            movement,
            cycle: movement + rest,
        },
    )(input)
}

#[aoc_generator(day14)]
fn parse(input: &str) -> Vec<Reindeer> {
    separated_list1(newline, parse_reindeer)(input).unwrap().1
}

#[aoc(day14, part1)]
fn part1(input: &[Reindeer]) -> Option<u32> {
    input.iter().map(|r| r.get_distance(2503)).max()
}

#[aoc(day14, part2)]
fn part2(input: &[Reindeer]) -> Option<u32> {
    let mut points = vec![0; input.len()];
    let mut dists = vec![0; input.len()];
    for t in 0..2503 {
        let mut max_dist = 0;
        for (i, r) in input.iter().enumerate() {
            if t % r.cycle < r.movement {
                dists[i] += r.speed;
            }
            max_dist = max_dist.max(dists[i]);
        }
        for (i, &dist) in dists.iter().enumerate() {
            if dist == max_dist {
                points[i] += 1
            }
        }
    }
    points.into_iter().max()
}

