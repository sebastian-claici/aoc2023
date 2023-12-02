use anyhow::Result;
use nom::{
    bytes::complete::tag,
    character::complete::{digit1, space1},
    combinator::map_res,
    multi::separated_list0,
    sequence::separated_pair,
    IResult,
};
use std::fs::read_to_string;
use std::str::FromStr;

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

fn parse_color(input: &str) -> IResult<&str, (u32, &str)> {
    separated_pair(parse_number, space1, tag("red"))(input)
        .or(separated_pair(parse_number, space1, tag("green"))(input))
        .or(separated_pair(parse_number, space1, tag("blue"))(input))
}

fn parse_round(input: &str) -> IResult<&str, Round> {
    let (input, parts) = separated_list0(tag(", "), parse_color)(input)?;
    let mut round = Round {
        red: 0,
        green: 0,
        blue: 0,
    };

    for (amount, color) in parts {
        match color {
            "red" => round.red = amount,
            "green" => round.green = amount,
            "blue" => round.blue = amount,
            _ => {}
        }
    }

    Ok((input, round))
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
    let games: Vec<Game> = data
        .lines()
        .map(|line| parse_game(line).unwrap().1)
        .filter(possible)
        .collect();

    Ok(games.iter().map(|game| game.index).sum())
}

fn power(game: Game) -> u32 {
    let mut min_red = 0;
    let mut min_green = 0;
    let mut min_blue = 0;

    for round in &game.rounds {
        min_red = std::cmp::max(min_red, round.red);
        min_green = std::cmp::max(min_green, round.green);
        min_blue = std::cmp::max(min_blue, round.blue);
    }

    min_red * min_green * min_blue
}

pub fn solve_b(filename: &str) -> Result<u32> {
    let data = read_to_string(filename)?;
    Ok(data
        .lines()
        .map(|line| parse_game(line).unwrap().1)
        .map(power)
        .sum())
}
