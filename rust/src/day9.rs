#![allow(unused)]

use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};
use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, i64, multispace0, multispace1, space1},
    multi::{many1, separated_list1},
    sequence::{preceded, terminated, tuple},
    IResult,
};

#[derive(Debug)]
pub struct Data {
    vals: Vec<Vec<i64>>,
}

fn parse_input(input: &str) -> IResult<&str, Data> {
    let (input, vals) = many1(terminated(separated_list1(space1, i64), multispace0))(input)?;
    Ok((input, Data { vals }))
}

#[aoc_generator(day9)]
pub fn generate_input(input: &str) -> Data {
    parse_input(input).unwrap().1
}

fn solve(list: Vec<i64>) -> i64 {
    if list.iter().all(|x| *x == 0) {
        return 0;
    }

    let mut diffs = vec![];
    for i in 0..list.len() - 1 {
        diffs.push(list[i + 1] - list[i]);
    }

    list[list.len() - 1] + solve(diffs)
}

#[aoc(day9, part1)]
pub fn solve_a(data: &Data) -> i64 {
    data.vals.iter().map(|list| solve(list.to_vec())).sum()
}

#[aoc(day9, part2)]
pub fn solve_b(data: &Data) -> i64 {
    data.vals.iter().map(|list| {
        let mut rev_list = list.clone();
        rev_list.reverse();
        solve(rev_list)
    }).sum()
}
