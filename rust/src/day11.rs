#![allow(unused)]

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

type Universe = Vec<Vec<u8>>;

fn get_explosions(universe: &Universe) -> Vec<(i16, i16)> {
    let mut result = vec![];
    for (i, row) in universe.iter().enumerate() {
        if row.iter().all(|&c| c == b'.') {
            result.push((i as i16, 0));
        }
    }
    let (r, c) = (universe.len(), universe[0].len());
    for j in 0..c {
        let mut good = true;
        for i in 0..r {
            if universe[i][j] != b'.' {
                good = false;
                break;
            }
        }
        if good {
            result.push((0, j as i16));
        }
    }

    result
}

#[aoc_generator(day11)]
pub fn generate_input(input: &str) -> Universe {
    input
        .lines()
        .map(|line| line.trim().as_bytes().to_vec())
        .collect()
}

fn solve(universe: &Universe, num: i64) -> i64 {
    let mut galaxies = vec![];
    let explosions = get_explosions(universe);
    for (i, row) in universe.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            if *c == b'#' {
                galaxies.push((i as i16, j as i16));
            }
        }
    }

    let mut ans = 0;
    for (&(r1, c1), &(r2, c2)) in galaxies.iter().tuple_combinations() {
        ans += (r1.abs_diff(r2) + c1.abs_diff(c2)) as i64;
        for &(ei, ej) in explosions.iter() {
            if (ei - r1) * (ei - r2) < 0 {
                ans += num;
            }
            if (ej - c1) * (ej - c2) < 0 {
                ans += num;
            }
        }
    }
    ans
}

#[aoc(day11, part1)]
pub fn solve_a(universe: &Universe) -> i64 {
    solve(&universe, 1)
}

#[aoc(day11, part2)]
pub fn solve_b(universe: &Universe) -> i64 {
    solve(&universe, 1000000 - 1)
}
