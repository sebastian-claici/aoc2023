#![allow(unused)]

use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

use aoc_runner_derive::{aoc, aoc_generator};

type Data = Vec<String>;

#[aoc_generator(day10)]
pub fn generate_input(input: &str) -> Data {
    input.lines().map(|line| line.trim().to_string()).collect()
}

fn neighbors(node: (i32, i32), bounds: (i32, i32), moves: &[(i32, i32)]) -> Vec<(usize, usize)> {
    moves
        .iter()
        .map(|(dr, dc)| (node.0 + dr, node.1 + dc))
        .filter(|(r, c)| 0 <= *r && *r < bounds.0 && 0 <= *c && *c < bounds.1)
        .map(|(r, c)| (r as usize, c as usize))
        .collect()
}

fn moves(arr: &[String], node: (usize, usize)) -> Vec<(usize, usize)> {
    let (r, c) = (node.0 as i32, node.1 as i32);
    let (m, n) = (arr.len() as i32, arr[node.0].len() as i32);

    let (east, west, north, south) = ((0, 1), (0, -1), (-1, 0), (1, 0));
    match arr[node.0].chars().nth(node.1) {
        Some('.') => vec![],
        Some('-') => neighbors((r, c), (m, n), &[east, west]),
        Some('|') => neighbors((r, c), (m, n), &[north, south]),
        Some('L') => neighbors((r, c), (m, n), &[north, east]),
        Some('J') => neighbors((r, c), (m, n), &[north, west]),
        Some('F') => neighbors((r, c), (m, n), &[south, east]),
        Some('7') => neighbors((r, c), (m, n), &[south, west]),
        Some('S') => neighbors((r, c), (m, n), &[north, west]),
        None => panic!("Neighbor out of bounds"),
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
            if arr[next.0].chars().nth(next.1) == Some('.') || dists.contains_key(&next) {
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
        if let Some(j) = row.chars().position(|c| c == 'S') {
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
        if let Some(j) = row.chars().position(|c| c == 'S') {
            start = (i, j);
            break;
        }
    }
    let on_loop: HashSet<(usize, usize)> = bfs(data, start).into_keys().collect();

    let mut ans = 0;
    for (i, row) in data.iter().enumerate() {
        let mut cross = 0;
        let mut corner: Option<char> = None;
        for (j, pipe) in row.chars().enumerate() {
            if on_loop.contains(&(i, j)) {
                let pipe = data[i].chars().nth(j).unwrap();
                (cross, corner) = match (corner, pipe) {
                    (_, '|') => (cross + 1, None),
                    (_, '-') => (cross, corner),
                    (Some('L'), '7') | (Some('F'), 'J') => (cross + 1, None),
                    (_, c) => (cross, Some(c)),
                }
            } else {
                ans += if cross % 2 == 1 { 1 } else { 0 };
            }
        }
    }
    ans
}
