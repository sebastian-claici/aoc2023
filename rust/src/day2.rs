use anyhow::Result;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, space1},
    combinator::map_res,
    multi::separated_list0,
    sequence::separated_pair,
    IResult,
};
use std::str::FromStr;
use std::{cmp::max, fs::read_to_string};

#[derive(Debug)]
struct Round {
    red: u32,
    green: u32,
    blue: u32,
}

#[derive(Debug)]
struct Game {
    index: u32,
    rounds: Vec<Round>,
}

fn parse_number(input: &str) -> IResult<&str, u32> {
    map_res(digit1, |s: &str| u32::from_str(s))(input)
}

fn parse_round(input: &str) -> IResult<&str, Round> {
    let (input, parts) = separated_list0(
        tag(", "),
        separated_pair(
            parse_number,
            space1,
            alt((tag("red"), tag("green"), tag("blue"))),
        ),
    )(input)?;
    let (mut red, mut green, mut blue) = (0, 0, 0);

    for (amount, color) in parts {
        match color {
            "red" => red = amount,
            "green" => green = amount,
            "blue" => blue = amount,
            _ => {}
        }
    }

    Ok((input, Round { red, green, blue }))
}

fn parse_game(input: &str) -> IResult<&str, Game> {
    let (input, _) = tag("Game ")(input)?;
    let (input, index) = map_res(digit1, |s: &str| u32::from_str(s))(input)?;
    let (input, _) = tag(": ")(input)?;
    let (input, rounds) = separated_list0(tag("; "), parse_round)(input)?;

    Ok((input, Game { index, rounds }))
}

fn possible(game: &Game) -> bool {
    game.rounds
        .iter()
        .all(|round| round.red <= 12 && round.green <= 13 && round.blue <= 14)
}

pub fn solve_a(filename: &str) -> Result<u32> {
    let data = read_to_string(filename)?;
    Ok(data
        .lines()
        .map(|line| parse_game(line).unwrap().1)
        .filter(possible)
        .map(|game| game.index)
        .sum())
}

fn power(game: Game) -> u32 {
    let best = game.rounds.iter().fold((0, 0, 0), |acc, r| {
        (max(acc.0, r.red), max(acc.1, r.green), max(acc.2, r.blue))
    });

    best.0 * best.1 * best.2
}

pub fn solve_b(filename: &str) -> Result<u32> {
    let data = read_to_string(filename)?;
    Ok(data
        .lines()
        .map(|line| parse_game(line).unwrap().1)
        .map(power)
        .sum())
}
