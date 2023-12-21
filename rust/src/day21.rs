#![allow(unused)]

use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

use aoc_runner_derive::{aoc, aoc_generator};

type Data = Vec<Vec<u8>>;

#[aoc_generator(day21)]
pub fn generate_input(input: &str) -> Data {
    input
        .lines()
        .map(|line| line.trim().as_bytes().to_vec())
        .collect()
}

fn bfs(arr: &Data, node: (i64, i64), max_steps: usize) -> usize {
    let (R, C) = (arr.len() as i64, arr[0].len() as i64);

    let mut states = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back((node, 0));

    let mut ans = 0;
    let moves = [(1, 0), (-1, 0), (0, 1), (0, -1)];
    while let Some(((r, c), d)) = queue.pop_front() {
        if d == max_steps {
            ans += 1;
            continue;
        }
        for (dr, dc) in moves.iter() {
            let (nr, nc) = ((r + dr).rem_euclid(R), (c + dc).rem_euclid(C));
            if arr[nr as usize][nc as usize] == b'#' {
                continue;
            }
            let state = ((r + dr, c + dc), d + 1);
            if states.contains(&state) {
                continue;
            }
            states.insert(state.clone());
            queue.push_back(state.clone());
        }
    }

    ans
}

#[aoc(day21, part1)]
pub fn solve_a(data: &Data) -> usize {
    let mut start = (0, 0);
    for (r, line) in data.iter().enumerate() {
        for (c, &ch) in line.iter().enumerate() {
            if ch == b'S' {
                start = (r as i64, c as i64);
                break;
            }
        }
    }

    bfs(data, start, 64)
}

fn quadratic(x: usize) -> usize {
    // These coefficients are derived by solving the following linear system
    //   
    //   4 2 1   a       91379
    //   1 1 1   b   =   32957
    //   0 0 1   c       3703
    //
    // The area (number of cells) grows quadratically in distance with "period" 131.
    // So we need steps(0 * 131 + 65), steps(1 * 131 + 65), steps(2 * 131 + 65), where
    // 65 comes from 26501365 % 131
    14584 * x * x + 14670 * x + 3703
}

#[aoc(day21, part2)]
pub fn solve_b(data: &Data) -> usize {
    let mut start = (0, 0);
    for (r, line) in data.iter().enumerate() {
        for (c, &ch) in line.iter().enumerate() {
            if ch == b'S' {
                start = (r as i64, c as i64);
                break;
            }
        }
    }

    quadratic(26501365 / 131)
}
