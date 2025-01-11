use aoc_runner_derive::{aoc, aoc_generator};
use nom::{
    character::complete::{alpha1, char, i32, newline, space1},
    multi::separated_list1,
    sequence::{delimited, preceded, terminated},
    IResult,
};

fn parse_ingredient(input: &str) -> IResult<&str, Vec<i32>> {
    preceded(
        terminated(alpha1, char(':')),
        separated_list1(char(','), preceded(delimited(space1, alpha1, space1), i32)),
    )(input)
}

fn get_score(ingredients: &[Vec<i32>], spoons: &[i32]) -> u32 {
    let mut mix = [0; 4];
    for (i, ingredient) in ingredients.iter().enumerate() {
        for (j, quality) in ingredient[0..4].iter().enumerate() {
            mix[j] += quality * spoons[i];
        }
    }
    mix.iter().fold(1, |acc, &i| acc * i.max(0) as u32)
}

fn get_calories(ingredients: &[Vec<i32>], spoons: &[i32]) -> u32 {
    spoons
        .iter()
        .enumerate()
        .fold(0, |acc, (i, spoon)| acc + ingredients[i][4] * spoon) as u32
}

#[aoc_generator(day15)]
fn parse(input: &str) -> Vec<Vec<i32>> {
    separated_list1(newline, parse_ingredient)(input).unwrap().1
}

#[aoc(day15, part1)]
fn part1(input: &[Vec<i32>]) -> u32 {
    let mut best = 0;
    for i in 0..=100 {
        for j in 0..=(100 - i) {
            for k in 0..=(100 - i - j) {
                best = best.max(get_score(input, &[i, j, k, 100 - i - j - k]));
            }
        }
    }
    best
}

#[aoc(day15, part2)]
fn part2(input: &[Vec<i32>]) -> u32 {
    let mut best = 0;
    for i in 0..=100 {
        for j in 0..=(100 - i) {
            for k in 0..=(100 - i - j) {
                let spoons = &[i, j, k, 100 - i - j - k];
                if get_calories(input, spoons) == 500 {
                    best = best.max(get_score(input, spoons));
                }
            }
        }
    }
    best
}

