use aoc_runner_derive::{aoc, aoc_generator};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{line_ending, multispace1, space0, space1, u64},
    multi::{many1, separated_list0},
    sequence::{preceded, terminated, tuple},
    IResult,
};
use rayon::prelude::*;

#[derive(Debug)]
pub struct Data {
    seeds: Vec<u64>,
    maps: Vec<Vec<(u64, u64, u64)>>,
}

fn parse_vec(input: &str) -> IResult<&str, Vec<u64>> {
    preceded(space0, separated_list0(space1, u64))(input)
}

fn parse_tup(input: &str) -> IResult<&str, (u64, u64, u64)> {
    let (input, (x, _, y, _, z)) = tuple((u64, space1, u64, space1, u64))(input)?;
    Ok((input, (x, y, z)))
}

fn parse_map(input: &str) -> IResult<&str, Vec<(u64, u64, u64)>> {
    let (input, _) = parse_map_tag(input)?;
    many1(terminated(parse_tup, line_ending))(input)
}

fn parse_map_tag(input: &str) -> IResult<&str, &str> {
    alt((
        tag("seed-to-soil map:\n"),
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
    let (input, maps) = many1(preceded(multispace1, parse_map))(input)?;

    Ok((input, Data { seeds, maps }))
}

#[aoc_generator(day5)]
pub fn generate_input(input: &str) -> Data {
    parse_input(input).unwrap().1
}

fn apply_map(map: &[(u64, u64, u64)], seed: u64) -> u64 {
    let mut result = seed;
    for (dst, src, len) in map.iter() {
        if src <= &result && result < src + len {
            result = dst + result - src;
            break;
        }
    }
    result
}

fn apply_maps(maps: &[Vec<(u64, u64, u64)>], seed: u64) -> u64 {
    maps.iter().fold(seed, |acc, map| apply_map(map, acc))
}

#[aoc(day5, part1)]
pub fn solve_a(data: &Data) -> u64 {
    data.seeds
        .iter()
        .map(|s| apply_maps(&data.maps, *s))
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
        .map(|(st, en)| (st..=en).fold(u64::MAX, |acc, val| acc.min(apply_maps(&data.maps, val))))
        .min()
        .unwrap()
}
