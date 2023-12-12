#![allow(unused)]

use aoc_runner_derive::{aoc, aoc_generator};
use bit_set::BitSet;
use itertools::Itertools;
use rayon::prelude::*;
use tap::Tap;
use std::collections::HashMap;

type Data = Vec<(Vec<u8>, Vec<usize>)>;

#[aoc_generator(day12)]
pub fn generate_input(input: &str) -> Data {
    let mut data = vec![];
    for line in input.lines() {
        let parts = line.trim().split_once(' ').unwrap();
        let nums = parts
            .1
            .split(',')
            .map(|s| s.parse::<usize>().unwrap())
            .collect();

        data.push((parts.0.as_bytes().to_vec(), nums));
    }
    data
}

fn solve(
    row: &[u8],
    pos: usize,
    groups: &[usize],
    indx: usize,
    sz: usize,
    cache: &mut HashMap<(usize, usize, usize), usize>,
) -> usize {
    if pos == row.len() {
        if (indx == groups.len() && sz == 0) || (indx == groups.len() - 1 && groups[indx] == sz) {
            return 1;
        } else {
            return 0;
        };
    }

    let key = (pos, indx, sz);
    if let Some(&cached) = cache.get(&key) {
        return cached;
    }

    let mut count = 0;
    for ins in [b'.', b'#'] {
        if row[pos] == ins || row[pos] == b'?' {
            if ins == b'.' && sz == 0 {
                count += solve(row, pos + 1, groups, indx, 0, cache);
            } else if ins == b'.' && sz > 0 && indx < groups.len() && groups[indx] == sz {
                count += solve(row, pos + 1, groups, indx + 1, 0, cache);
            } else if ins == b'#' && indx < groups.len() && sz < groups[indx] {
                count += solve(row, pos + 1, groups, indx, sz + 1, cache);
            }
        }
    }

    cache.insert(key, count);
    count
}

#[aoc(day12, part1)]
pub fn solve_a(data: &Data) -> usize {
    let mut ans = 0;
    for (s, groups) in data.iter() {
        let mut cache = HashMap::new();
        ans += solve(s, 0, groups, 0, 0, &mut cache);
    }
    ans
}

#[aoc(day12, part2)]
pub fn solve_b(data: &Data) -> usize {
    let mut ans = 0;
    for (s, groups) in data.iter() {
        let mut copied_s = [s.as_slice(), &[b'?']].concat().repeat(5);
        copied_s.pop();
        let copied_groups = groups.repeat(5);

        let mut cache = HashMap::new();
        ans += solve(&copied_s, 0, &copied_groups, 0, 0, &mut cache);
    }
    ans
}
