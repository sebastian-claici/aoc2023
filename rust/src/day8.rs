use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};
use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, multispace0, multispace1},
    multi::many1,
    sequence::{preceded, terminated, tuple},
    IResult,
};

pub fn lcm(nums: &[usize]) -> usize {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lcm(&nums[1..]);
    a * b / gcd(a, b)
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }
    gcd(b, a % b)
}

#[derive(Debug)]
pub struct Data {
    inst: String,
    edges: HashMap<String, (String, String)>,
}

fn parse_input(input: &str) -> IResult<&str, Data> {
    let (input, inst) = alpha1(input)?;
    let (input, tups) = preceded(
        multispace1,
        many1(terminated(
            tuple((alpha1, tag(" = ("), alpha1, tag(", "), alpha1, tag(")"))),
            multispace0,
        )),
    )(input)?;

    let mut edges = HashMap::new();
    tups.into_iter().for_each(|(src, _, lft, _, rht, _)| {
        edges.insert(src.to_string(), (lft.to_string(), rht.to_string()));
    });

    Ok((
        input,
        Data {
            inst: inst.to_string(),
            edges,
        },
    ))
}

#[aoc_generator(day8)]
pub fn generate_input(input: &str) -> Data {
    parse_input(input).unwrap().1
}

pub fn search(data: &Data, mut node: String, end_fn: fn(&str) -> bool) -> usize {
    let mut result = 0;
    for i in data.inst.chars().cycle() {
        if end_fn(&node) {
            return result;
        }
        result += 1;
        if i == 'L' {
            node = data.edges.get(&node).unwrap().0.clone();
        } else {
            node = data.edges.get(&node).unwrap().1.clone();
        }
    }
    result
}

#[aoc(day8, part1)]
pub fn solve_a(data: &Data) -> usize {
    let node = "AAA".to_string();
    search(data, node, |s| s == "ZZZ")
}

#[aoc(day8, part2)]
pub fn solve_b(data: &Data) -> usize {
    let nodes: Vec<_> = data.edges.keys().filter(|s| s.ends_with("A")).collect();
    let lens: Vec<_> = nodes
        .iter()
        .map(|node| search(&data, node.to_string(), |s| s.ends_with("Z")))
        .collect();

    lcm(&lens)
}
