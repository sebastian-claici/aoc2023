#![allow(unused)]

use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

use aoc_runner_derive::{aoc, aoc_generator};

type Data = Vec<Vec<u8>>;

#[aoc_generator(day10)]
pub fn generate_input(input: &str) -> Data {
    input.lines().map(|line| line.trim().as_bytes().to_vec()).collect()
}

fn neighbors(node: (i32, i32), bounds: (i32, i32), moves: &[(i32, i32)]) -> Vec<(usize, usize)> {
    moves
        .iter()
        .map(|(dr, dc)| (node.0 + dr, node.1 + dc))
        .filter(|(r, c)| 0 <= *r && *r < bounds.0 && 0 <= *c && *c < bounds.1)
        .map(|(r, c)| (r as usize, c as usize))
        .collect()
}

fn moves(arr: &[Vec<u8>], node: (usize, usize)) -> Vec<(usize, usize)> {
    let (r, c) = (node.0 as i32, node.1 as i32);
    let (m, n) = (arr.len() as i32, arr[node.0].len() as i32);

    let (east, west, north, south) = ((0, 1), (0, -1), (-1, 0), (1, 0));
    match arr[node.0][node.1] {
        b'.' => vec![],
        b'-' => neighbors((r, c), (m, n), &[east, west]),
        b'|' => neighbors((r, c), (m, n), &[north, south]),
        b'L' => neighbors((r, c), (m, n), &[north, east]),
        b'J' => neighbors((r, c), (m, n), &[north, west]),
        b'F' => neighbors((r, c), (m, n), &[south, east]),
        b'7' => neighbors((r, c), (m, n), &[south, west]),
        // S behaves like a J in my input
        b'S' => neighbors((r, c), (m, n), &[north, west]),
        _ => panic!("Unrecognized character"),
    }
}

fn bfs(arr: &Data, node: (usize, usize)) -> HashMap<(usize, usize), usize> {
    let mut dists = HashMap::new();
    let mut queue = VecDeque::new();
    queue.push_back(node);

    while let Some(node) = queue.pop_front() {
        let d = *dists.entry(node).or_insert(0);
        for next in moves(arr, node) {
            if dists.contains_key(&next) {
                continue;
            }
            dists.insert(next, d + 1);
            queue.push_back(next);
        }
    }

    dists
}

#[aoc(day10, part1)]
pub fn solve_a(data: &Data) -> usize {
    let mut start = (0, 0);
    for (i, row) in data.iter().enumerate() {
        if let Some(j) = row.iter().position(|c| *c == b'S') {
            start = (i, j);
            break;
        }
    }
    let dists = bfs(data, start);
    *dists.values().max().unwrap()
}

#[aoc(day10, part2)]
pub fn solve_b(data: &Data) -> i64 {
    let mut start = (0, 0);
    for (i, row) in data.iter().enumerate() {
        if let Some(j) = row.iter().position(|c| *c == b'S') {
            start = (i, j);
            break;
        }
    }
    let on_loop: HashSet<_> = bfs(data, start).into_keys().collect();

    let mut ans = 0;
    for (i, row) in data.iter().enumerate() {
        let mut cross = 0;
        let mut corner: Option<u8> = None;
        for (j, c) in row.iter().enumerate() {
            let pipe = if *c == b'S' { b'J' } else { *c };
            if on_loop.contains(&(i, j)) {
                (cross, corner) = match (corner, pipe) {
                    (_, b'|') => (cross + 1, None),
                    (_, b'-') => (cross, corner),
                    (Some(b'L'), b'7') | (Some(b'F'), b'J') => (cross + 1, None),
                    (_, c) => (cross, Some(c)),
                }
            } else {
                ans += if cross % 2 == 1 { 1 } else { 0 };
            }
        }
    }
    ans
}
