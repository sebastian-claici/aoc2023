use anyhow::Result;
use aoc_runner_derive::{aoc, aoc_generator};
use regex::Match;
use std::cmp::min;
use std::collections::HashMap;

struct Bounds {
    start_x: usize,
    start_y: usize,
    end_x: usize,
    end_y: usize,
}

fn find_symbols(board: &[String], bound: &Bounds) -> bool {
    for y in bound.start_y..=bound.end_y {
        for x in bound.start_x..=bound.end_x {
            let c = board[y].chars().nth(x).unwrap();
            if !c.is_numeric() && c != '.' {
                return true;
            }
        }
    }
    false
}

fn find_gears(
    board: &[String],
    bound: &Bounds,
    num: i32,
    gears: &mut HashMap<(usize, usize), Vec<i32>>,
) {
    for y in bound.start_y..=bound.end_y {
        for x in bound.start_x..=bound.end_x {
            let c = board[y].chars().nth(x).unwrap();
            if c == '*' {
                gears.entry((y, x)).or_insert(vec![]).push(num)
            }
        }
    }
}

fn get_bounds(board: &[String], line: &str, row_num: usize, mat: Match) -> Bounds {
    Bounds {
        start_x: mat.start().saturating_sub(1),
        start_y: row_num.saturating_sub(1),
        end_x: min(mat.end(), line.len() - 1),
        end_y: min(row_num + 1, board.len() - 1),
    }
}

#[aoc_generator(day3)]
pub fn generate_input(input: &str) -> Vec<String> {
    input.lines().map(|line| line.trim().to_string()).collect()
}

#[aoc(day3, part1)]
pub fn solve_a(board: &[String]) -> i32 {
    let mut total = 0;

    let num_pattern = regex::Regex::new(r"\d+").unwrap();
    for (row_num, line) in board.iter().enumerate() {
        for mat in num_pattern.find_iter(line) {
            let num = mat.as_str().parse::<i32>().unwrap();
            let bound = get_bounds(&board, line, row_num, mat);

            if find_symbols(&board, &bound) {
                total += num;
            }
        }
    }

    total
}

#[aoc(day3, part2)]
pub fn solve_b(board: &[String]) -> i32 {
    let mut gears = HashMap::new();
    let num_pattern = regex::Regex::new(r"\d+").unwrap();
    for (row_num, line) in board.iter().enumerate() {
        for mat in num_pattern.find_iter(line) {
            let num = mat.as_str().parse::<i32>().unwrap();
            let bound = get_bounds(&board, line, row_num, mat);

            find_gears(&board, &bound, num, &mut gears);
        }
    }

    gears
        .iter()
        .filter(|(_, v)| v.len() == 2)
        .map(|(_, v)| v[0] * v[1])
        .sum()
}
