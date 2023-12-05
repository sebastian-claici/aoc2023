use aoc_runner_derive::{aoc, aoc_generator};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, multispace1, space0, space1},
    combinator::map_res,
    multi::separated_list0,
    sequence::{preceded, terminated},
    IResult,
};
use rayon::prelude::*;
use std::str::FromStr;

#[derive(Debug)]
pub struct Data {
    seeds: Vec<u64>,
    maps: Vec<Vec<(u64, u64, u64)>>,
}

fn parse_number(input: &str) -> IResult<&str, u64> {
    map_res(digit1, |s: &str| u64::from_str(s))(input)
}

fn parse_vec(input: &str) -> IResult<&str, Vec<u64>> {
    preceded(space0, separated_list0(space1, parse_number))(input)
}

fn parse_map(input: &str) -> IResult<&str, Vec<Vec<u64>>> {
    separated_list0(multispace1, separated_list0(space1, parse_number))(input)
}

fn parse_map_tag(input: &str) -> IResult<&str, &str> {
    alt((
        tag("soil-to-fertilizer map:\n"),
        tag("fertilizer-to-water map:\n"),
        tag("water-to-light map:\n"),
        tag("light-to-temperature map:\n"),
        tag("temperature-to-humidity map:\n"),
        tag("humidity-to-location map:\n"),
    ))(input)
}

fn parse_input(input: &str) -> IResult<&str, Data> {
    let (input, seeds) = preceded(tag("seeds: "), parse_vec)(input)?;
    let (input, _) = terminated(multispace1, tag("seed-to-soil map:\n"))(input)?;
    let (input, tmp_maps) = separated_list0(parse_map_tag, parse_map)(input)?;

    let mut maps = vec![];
    for map in tmp_maps.iter() {
        maps.push(
            map.iter()
                .filter(|v| !v.is_empty())
                .map(|v| (v[0], v[1], v[2]))
                .collect(),
        );
    }
    Ok((input, Data { seeds, maps }))
}

#[aoc_generator(day5)]
pub fn generate_input(input: &str) -> Data {
    parse_input(input).unwrap().1
}

fn apply_r(maps: &Vec<Vec<(u64, u64, u64)>>, seed: u64) -> u64 {
    let mut result = seed;
    for map in maps {
        for (dst, src, len) in map.iter() {
            if src <= &result && result < src + len {
                result = dst + result - src;
                break;
            }
        }
    }
    result
}

#[aoc(day5, part1)]
pub fn solve_a(data: &Data) -> u64 {
    data.seeds
        .iter()
        .map(|s| apply_r(&data.maps, *s))
        .min()
        .unwrap()
}

#[aoc(day5, part2)]
pub fn solve_b(data: &Data) -> u64 {
    let seeds: Vec<_> = data
        .seeds
        .chunks(2)
        .map(|v| (v[0], v[0] + v[1] - 1))
        .collect();

    seeds
        .into_par_iter()
        .map(|(st, en)| (st..=en).map(|x| apply_r(&data.maps, x)).min().unwrap())
        .min()
        .unwrap()
}
