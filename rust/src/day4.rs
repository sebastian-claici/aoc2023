use aoc_runner_derive::{aoc, aoc_generator};
use nom::{
    bytes::complete::{is_not, tag},
    character::complete::{digit1, space0, space1},
    combinator::map_res,
    multi::separated_list0,
    sequence::separated_pair,
    IResult,
};
use std::{collections::HashMap, str::FromStr};

pub struct Card {
    winning: Vec<u32>,
    held: Vec<u32>,
}

fn parse_number(input: &str) -> IResult<&str, u32> {
    map_res(digit1, |s: &str| u32::from_str(s))(input)
}

fn parse_vec(input: &str) -> IResult<&str, Vec<u32>> {
    let (input, _) = space0(input)?;
    let (input, parts) = separated_list0(space1, parse_number)(input)?;

    Ok((input, parts))
}

fn parse_card(input: &str) -> IResult<&str, Card> {
    let (input, _) = is_not(":")(input)?;
    let (input, _) = tag(":")(input)?;
    let (input, _) = space0(input)?;
    let (input, (winning, held)) = separated_pair(parse_vec, tag(" | "), parse_vec)(input)?;

    Ok((input, Card { winning, held }))
}

fn overlap(card: &Card) -> usize {
    card.held
        .iter()
        .filter(|x| card.winning.contains(x))
        .count()
}

#[aoc_generator(day4)]
pub fn generate_input(input: &str) -> Vec<Card> {
    input
        .lines()
        .map(|line| parse_card(line).unwrap().1)
        .collect()
}

#[aoc(day4, part1)]
pub fn solve_a(cards: &[Card]) -> u32 {
    cards
        .iter()
        .map(|card| overlap(&card) as u32)
        .filter(|x| x > &0)
        .map(|x| 2_u32.pow(x - 1))
        .sum()
}

#[aoc(day4, part2)]
pub fn solve_b(cards: &[Card]) -> u32 {
    let mut counts: HashMap<usize, u32> = HashMap::new();
    for (i, card) in cards.iter().enumerate() {
        let cnt = *counts.entry(i).or_insert(1);
        let num = overlap(&card);
        (i + 1..=i + num).for_each(|k| *counts.entry(k).or_insert(1) += cnt);
    }

    counts.values().sum()
}
