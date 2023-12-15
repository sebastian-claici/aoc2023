#![allow(unused)]

use aoc_runner_derive::{aoc, aoc_generator};
use bit_set::BitSet;
use itertools::Itertools;
use rayon::prelude::*;
use std::collections::{HashMap, HashSet};
use tap::Tap;

type Matrix = Vec<Vec<u8>>;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Dir {
    North,
    South,
    East,
    West,
}

#[aoc_generator(day14)]
pub fn generate_input(input: &str) -> Matrix {
    input
        .lines()
        .map(|s| s.trim().as_bytes().to_vec())
        .collect_vec()
}

fn load(matrix: &Matrix) -> usize {
    let (r, c) = (matrix.len(), matrix[0].len());

    let mut ans = 0;
    for col in 0..c {
        for row in 0..r {
            if matrix[row][col] == b'O' {
                ans += r - row;
            }
        }
    }
    ans
}

fn shift(matrix: &Matrix, dir: Dir) -> Matrix {
    let mut result = match dir {
        Dir::North => matrix.clone(),
        Dir::South => flip(&matrix),
        Dir::West => transpose(&matrix),
        Dir::East => flip(&transpose(&matrix)),
    };

    let (r, c) = (matrix.len(), matrix[0].len());
    for col in 0..c {
        let mut last = -1;
        for row in 0..r {
            let curr = result[row][col];
            let idx = (last + 1) as usize;
            if curr == b'O' {
                if idx != row {
                    result[idx][col] = b'O';
                    result[row][col] = b'.';
                }
                last += 1;
            } else if curr == b'#' {
                last = row as i64;
            }
        }
    }

    match dir {
        Dir::North => result,
        Dir::South => flip(&result),
        Dir::West => transpose(&result),
        Dir::East => transpose(&flip(&result)),
    }
}

fn flip<T>(matrix: &Vec<Vec<T>>) -> Vec<Vec<T>>
where
    T: Clone + Copy,
{
    let mut flipped = vec![];
    let (r, c) = (matrix.len(), matrix[0].len());
    for i in (0..r).rev() {
        let mut row = vec![];
        for j in 0..c {
            row.push(matrix[i][j]);
        }
        flipped.push(row.clone());
    }
    flipped
}

fn transpose<T>(matrix: &Vec<Vec<T>>) -> Vec<Vec<T>>
where
    T: Clone + Copy,
{
    let mut transposed = vec![];
    let (r, c) = (matrix.len(), matrix[0].len());
    for j in 0..c {
        let mut row = vec![];
        for i in 0..r {
            row.push(matrix[i][j]);
        }
        transposed.push(row.clone());
    }
    transposed
}

fn show(matrix: &Matrix) {
    for row in matrix.iter() {
        println!("{}", row.iter().map(|c| *c as char).collect::<String>());
    }
    println!();
}

#[aoc(day14, part1)]
pub fn solve_a(data: &Matrix) -> usize {
    let shifted = shift(&data, Dir::North);
    load(&shifted)
}

#[aoc(day14, part2)]
pub fn solve_b(data: &Matrix) -> usize {
    let mut matrix = data.clone();
    let mut cycles = HashMap::new();
    let max_cnt = 1000000000;
    let mut cnt = 0;
    let mut cycle_found = true;

    while cnt < max_cnt {
        let mut state = matrix.clone();
        if !cycle_found {
            if let Some(idx) = cycles.get(&state) {
                cycle_found = true;
                let cycle_len = cnt - idx;
                let rem = max_cnt - cnt;
                let num_cycles = rem / cycle_len;
                cnt += cycle_len * num_cycles;
            } else {
                cycles.insert(state, cnt);
            }
        }

        matrix = shift(&matrix, Dir::North);
        matrix = shift(&matrix, Dir::West);
        matrix = shift(&matrix, Dir::South);
        matrix = shift(&matrix, Dir::East);

        cnt += 1;
    }
    show(&matrix);
    load(&matrix)
}
