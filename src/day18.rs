use aoc_runner_derive::aoc;
use std::iter::once;

fn parse(input: &str) -> Vec<Vec<bool>> {
    let width = input.lines().next().unwrap().len();
    once(vec![false; width + 2])
        .chain(input.lines().map(|line| {
            once(false)
                .chain(line.chars().map(|c| c == '#'))
                .chain(once(false))
                .collect()
        }))
        .chain(once(vec![false; width + 2]))
        .collect()
}

fn count_neighbors(grid: &[Vec<bool>], i: usize, j: usize) -> usize {
    let mut count = 0;
    for row in grid.iter().skip(i - 1).take(3) {
        for cell in row.iter().skip(j - 1).take(3) {
            if *cell {
                count += 1
            }
        }
    }
    count
}

fn switch(grid: &[Vec<bool>], fixed_corners: bool) -> Vec<Vec<bool>> {
    let mut new_grid = grid.to_vec();
    for i in 1..(grid.len() - 1) {
        for j in 1..(grid[i].len() - 1) {
            if fixed_corners
                && (i == 1 || i == grid.len() - 2)
                && (j == 1 || j == grid[i].len() - 2)
            {
                continue;
            }
            let c = count_neighbors(grid, i, j);
            new_grid[i][j] = c == 3 || grid[i][j] && c == 4;
        }
    }
    new_grid
}

fn count_lights(grid: &[Vec<bool>]) -> usize {
    grid.iter()
        .flat_map(|row| row.iter().filter(|&b| *b))
        .count()
}

#[aoc(day18, part1)]
fn part1(input: &str) -> usize {
    let mut grid = parse(input);
    for _ in 0..100 {
        grid = switch(&grid, false);
    }
    count_lights(&grid)
}

#[aoc(day18, part2)]
fn part2(input: &str) -> usize {
    let mut grid = parse(input);
    for i in [1, grid.len() - 2] {
        for j in [1, grid[0].len() - 2] {
            grid[i][j] = true;
        }
    }
    for _ in 0..100 {
        grid = switch(&grid, true);
    }
    count_lights(&grid)
}
