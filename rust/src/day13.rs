#![allow(unused)]

use aoc_runner_derive::{aoc, aoc_generator};
use bit_set::BitSet;
use itertools::Itertools;
use rayon::prelude::*;
use std::collections::HashMap;
use tap::Tap;

type Pattern = Vec<Vec<u8>>;
type Data = Vec<Pattern>;

#[aoc_generator(day13)]
pub fn generate_input(input: &str) -> Data {
    let mut data = vec![];
    let mut pattern = Pattern::new();
    for line in input.lines() {
        let trimmed = line.trim().as_bytes().to_vec();
        if trimmed.is_empty() {
            data.push(pattern.clone());
            pattern = Pattern::new();
        } else {
            pattern.push(trimmed);
        }
    }
    if !pattern.is_empty() {
        data.push(pattern.clone());
    }
    data
}

fn transpose(pattern: &Pattern) -> Pattern {
    let mut transposed = Pattern::new();
    let (r, c) = (pattern.len(), pattern[0].len());
    for j in 0..c {
        let mut row = vec![];
        for i in 0..r {
            row.push(pattern[i][j]);
        }
        transposed.push(row.clone());
    }
    transposed
}

fn show(row: &[u8]) -> String {
    String::from_utf8(row.to_owned()).unwrap()
}

fn cnt_smudges(lft: &[u8], rgt: &[u8]) -> usize {
    let mut mis = 0;
    for i in 0..lft.len() {
        if lft[i] != rgt[i] {
            mis += 1;
        }
    }
    mis
}

fn reflections(pattern: &Pattern, cnt: usize) -> Vec<usize> {
    let r = pattern.len();
    let mut ans = vec![];
    for k in 0..(r - 1) {
        let mut smudges = 0;
        for (i, j) in (0..=k).rev().zip(k + 1..r) {
            smudges += cnt_smudges(&pattern[i], &pattern[j]);
        }
        if smudges == cnt {
            ans.push(k + 1);
        }
    }
    ans
}

#[aoc(day13, part1)]
pub fn solve_a(data: &Data) -> usize {
    data.iter()
        .map(|pattern| -> usize {
            let trans = transpose(pattern);
            let horz: usize = reflections(pattern, 0).iter().map(|x| x * 100).sum();
            let vert: usize = reflections(&trans, 0).iter().sum();
            horz + vert
        })
        .sum()
}

#[aoc(day13, part2)]
pub fn solve_b(data: &Data) -> usize {
    data.iter()
        .map(|pattern| -> usize {
            let trans = transpose(pattern);
            let horz = reflections(pattern, 1);
            let vert = reflections(&trans, 1);
            if horz.len() + vert.len() != 1 {
                println!("{:?}, {:?}", horz, vert);
            }
            horz.iter().map(|x| x * 100).sum::<usize>() + vert.iter().sum::<usize>()
        })
        .sum()
}
