use anyhow::Result;
use regex::Match;
use std::cmp::min;
use std::collections::HashMap;
use std::fs::read_to_string;

struct Bounds {
    start_x: usize,
    start_y: usize,
    end_x: usize,
    end_y: usize,
}

fn find_symbols(board: &Vec<String>, bound: &Bounds) -> Result<bool> {
    for y in bound.start_y..=bound.end_y {
        for x in bound.start_x..=bound.end_x {
            let c = board[y].chars().nth(x).unwrap();
            if !c.is_numeric() && c != '.' {
                return Ok(true);
            }
        }
    }
    Ok(false)
}

fn find_gears(
    board: &Vec<String>,
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

fn read_board(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .expect("Failed to read file")
        .lines()
        .map(|line| line.trim().to_string())
        .collect()
}

fn get_bounds(board: &Vec<String>, line: &str, row_num: usize, mat: Match) -> Bounds {
    Bounds {
        start_x: mat.start().saturating_sub(1),
        start_y: row_num.saturating_sub(1),
        end_x: min(mat.end(), line.len() - 1),
        end_y: min(row_num + 1, board.len() - 1),
    }
}

pub fn solve_a(filename: &str) -> Result<i32> {
    let mut total = 0;
    let board = read_board(filename);

    let num_pattern = regex::Regex::new(r"\d+").unwrap();
    for (row_num, line) in board.iter().enumerate() {
        for mat in num_pattern.find_iter(line) {
            let num = mat.as_str().parse::<i32>().unwrap();
            let bound = get_bounds(&board, line, row_num, mat);

            if find_symbols(&board, &bound)? {
                total += num;
            }
        }
    }

    Ok(total)
}

pub fn solve_b(filename: &str) -> Result<i32> {
    let board = read_board(filename);

    let mut gears = HashMap::new();
    let num_pattern = regex::Regex::new(r"\d+").unwrap();
    for (row_num, line) in board.iter().enumerate() {
        for mat in num_pattern.find_iter(line) {
            let num = mat.as_str().parse::<i32>().unwrap();
            let bound = get_bounds(&board, line, row_num, mat);

            find_gears(&board, &bound, num, &mut gears);
        }
    }

    Ok(gears
        .iter()
        .filter(|(_, v)| v.len() == 2)
        .map(|(_, v)| v[0] * v[1])
        .sum())
}
