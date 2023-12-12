#![allow(unused)]

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use bit_set::BitSet;
use rayon::prelude::*;

type Universe = Vec<Vec<u8>>;
type Galaxies = Vec<(i16, i16)>;
type Explosions = Vec<(i16, i16)>;

fn get_events(universe: &Universe) -> (Galaxies, Explosions) {
    let mut galaxies = vec![];
    let mut explosions = vec![];

    let (r, c) = (universe.len(), universe[0].len());
    let mut ecols = BitSet::with_capacity(c);
    for (i, row) in universe.iter().enumerate() {
        let mut has_galaxy = false;
        for (j, &c) in row.iter().enumerate() {
            if c == b'#' {
                galaxies.push((i as i16, j as i16));
                has_galaxy = true;
                ecols.insert(j);
            }
        }
        if !has_galaxy {
            explosions.push((i as i16, 0));
        }
    }
    for j in 0..c {
        if !ecols.contains(j) {
            explosions.push((0, j as i16));
        }
    }

    (galaxies, explosions)
}

#[aoc_generator(day11)]
pub fn generate_input(input: &str) -> Universe {
    input
        .lines()
        .map(|line| line.trim().as_bytes().to_vec())
        .collect()
}

fn solve(universe: &Universe, num: i64) -> i64 {
    let (galaxies, explosions) = get_events(universe);

    let mut ans: i64 = 0;
    for (&(r1, c1), &(r2, c2)) in galaxies.iter().tuple_combinations() {
        ans += ((r1 - r2).abs() + (c1 - c2).abs()) as i64;
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
    solve(universe, 1)
}

#[aoc(day11, part2)]
pub fn solve_b(universe: &Universe) -> i64 {
    solve(universe, 1000000 - 1)
}
