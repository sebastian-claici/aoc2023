use aoc_runner_derive::{aoc, aoc_generator};
use nom::{
    bytes::complete::{is_not, tag},
    character::complete::{digit1, space0, space1},
    combinator::map_res,
    multi::separated_list0,
    sequence::{preceded, separated_pair, tuple},
    IResult,
};
use std::{collections::HashMap, str::FromStr};

pub struct Card {
    winning: Vec<u32>,
    held: Vec<u32>,
}

impl Card {
    fn overlap(self: &Self) -> usize {
        self.held
            .iter()
            .filter(|x| self.winning.contains(x))
            .count()
    }
}

fn parse_number(input: &str) -> IResult<&str, u32> {
    map_res(digit1, |s: &str| u32::from_str(s))(input)
}

fn parse_vec(input: &str) -> IResult<&str, Vec<u32>> {
    preceded(space0, separated_list0(space1, parse_number))(input)
}

fn parse_card(input: &str) -> IResult<&str, Card> {
    let (input, _) = tuple((is_not(":"), tag(":"), space0))(input)?;
    let (input, (winning, held)) = separated_pair(parse_vec, tag(" | "), parse_vec)(input)?;

    Ok((input, Card { winning, held }))
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
        .map(|card| card.overlap() as u32)
        .filter(|x| x > &0)
        .map(|x| 2_u32.pow(x - 1))
        .sum()
}

#[aoc(day4, part2)]
pub fn solve_b(cards: &[Card]) -> u32 {
    let mut counts: HashMap<usize, u32> = HashMap::new();
    for (i, card) in cards.iter().enumerate() {
        let cnt = *counts.entry(i).or_insert(1);
        (i + 1..=i + card.overlap()).for_each(|k| *counts.entry(k).or_insert(1) += cnt);
    }

    counts.values().sum()
}
