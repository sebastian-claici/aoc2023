use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{alphanumeric1, multispace0, u32},
    multi::many1,
    sequence::{separated_pair, terminated},
    IResult,
};
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash, Ord, PartialOrd)]
enum Card {
    Joker,
    Num(u8),
    T,
    J,
    Q,
    K,
    A,
}

impl Card {
    fn new(c: char) -> Self {
        match c {
            '2'..='9' => Card::Num(c as u8 - '0' as u8),
            'J' => Card::J,
            'T' => Card::T,
            'Q' => Card::Q,
            'K' => Card::K,
            'A' => Card::A,
            _ => panic!("unknown character"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Rank {
    HighCard,
    Pair,
    TwoPair,
    Threes,
    FullHouse,
    Fours,
    Fives,
}

type Hand = [Card; 5];

fn swap(hand: &Hand, from: Card, to: Card) -> Hand {
    hand.into_iter()
        .map(|card| if *card == from { to } else { *card })
        .collect::<Vec<_>>()
        .try_into()
        .unwrap()
}

impl Rank {
    fn new(cards: &Hand) -> Self {
        let mut counts = HashMap::new();
        let _ = cards
            .iter()
            .for_each(|card| *counts.entry(card).or_insert(0) += 1);

        let counts: Vec<_> = counts.values().sorted().rev().collect();
        match counts[..] {
            [5] => Self::Fives,
            [4, 1] => Self::Fours,
            [3, 2] => Self::FullHouse,
            [3, 1, 1] => Self::Threes,
            [2, 2, 1] => Self::TwoPair,
            [2, 1, 1, 1] => Self::Pair,
            _ => Self::HighCard,
        }
    }

    fn best(hand: &Hand) -> Self {
        let mut opts: Vec<_> = (2..=9).map(|x| Card::Num(x)).collect();
        opts.extend([Card::T, Card::Q, Card::K, Card::A]);

        opts.iter()
            .map(|opt| {
                let new_hand = swap(&hand, Card::Joker, *opt);
                Rank::new(&new_hand)
            })
            .max()
            .unwrap()
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Round {
    rank: Rank,
    hand: Hand,
    bid: u32,
}

#[derive(Debug)]
pub struct Data {
    rounds: Vec<Round>,
}

fn parse_round(input: &str) -> IResult<&str, Round> {
    let (input, (hand, bid)) = separated_pair(alphanumeric1, tag(" "), u32)(input)?;

    let hand = hand
        .chars()
        .map(Card::new)
        .collect::<Vec<_>>()
        .try_into()
        .unwrap();
    let rank = Rank::new(&hand);

    Ok((input, Round { rank, hand, bid }))
}

fn parse_input(input: &str) -> IResult<&str, Data> {
    let (input, rounds) = many1(terminated(parse_round, multispace0))(input)?;

    Ok((input, Data { rounds }))
}

#[aoc_generator(day7)]
pub fn generate_input(input: &str) -> Data {
    parse_input(input).unwrap().1
}

#[aoc(day7, part1)]
pub fn solve_a(data: &Data) -> u32 {
    data.rounds
        .iter()
        .sorted()
        .enumerate()
        .map(|(i, round)| ((i + 1) as u32) * round.bid)
        .sum()
}

#[aoc(day7, part2)]
pub fn solve_b(data: &Data) -> u32 {
    data.rounds
        .iter()
        .map(|round| {
            let hand = swap(&round.hand, Card::J, Card::Joker);
            Round {
                hand,
                rank: Rank::best(&hand),
                bid: round.bid,
            }
        })
        .sorted()
        .enumerate()
        .map(|(i, round)| ((i + 1) as u32) * round.bid)
        .sum()
}
