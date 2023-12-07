use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{alphanumeric1, multispace0, u32},
    multi::many1,
    sequence::{separated_pair, terminated},
    IResult,
};
use std::cmp;
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
enum Hand {
    HighCard,
    Pair,
    TwoPair,
    Threes,
    FullHouse,
    Fours,
    Fives,
}

impl Hand {
    fn new(cards: &[Card]) -> Self {
        let mut counts = HashMap::new();
        let _ = cards
            .iter()
            .for_each(|card| *counts.entry(card).or_insert(0) += 1);

        let counts: Vec<_> = counts
            .values()
            .sorted()
            .rev()
            .collect();
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

    fn best(cards: &[Card]) -> Self {
        let mut opts: Vec<_> = (2..=9).map(|x| Card::Num(x)).collect();
        opts.extend([Card::T, Card::Q, Card::K, Card::A]);

        opts.iter()
            .map(|opt| {
                let mut tmp_cards = [Card::J; 5];
                for (i, card) in cards.iter().enumerate() {
                    tmp_cards[i] = match *card {
                        Card::Joker => *opt,
                        _ => *card,
                    }
                }
                Hand::new(&tmp_cards)
            })
            .max()
            .unwrap()
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Round {
    cards: [Card; 5],
    hand: Hand,
    bid: u32,
}

impl Ord for Round {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        if self.hand.cmp(&other.hand) != cmp::Ordering::Equal {
            return self.hand.cmp(&other.hand);
        }
        for (c1, c2) in self.cards.iter().zip(other.cards.iter()) {
            if c1.cmp(c2) != cmp::Ordering::Equal {
                return c1.cmp(c2);
            }
        }
        cmp::Ordering::Equal
    }
}

impl PartialOrd for Round {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug)]
pub struct Data {
    rounds: Vec<Round>,
}

fn parse_round(input: &str) -> IResult<&str, Round> {
    let (input, (hand, bid)) = separated_pair(alphanumeric1, tag(" "), u32)(input)?;

    let mut cards = [Card::A; 5];
    for (i, c) in hand.chars().enumerate() {
        cards[i] = Card::new(c);
    }
    let hand = Hand::new(&cards);

    Ok((input, Round { cards, hand, bid }))
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
    let v: Vec<_> = data.rounds.iter().sorted().collect();
    v.iter()
        .enumerate()
        .map(|(i, round)| ((i + 1) as u32) * round.bid)
        .sum()
}

#[aoc(day7, part2)]
pub fn solve_b(data: &Data) -> u32 {
    let v: Vec<_> = data
        .rounds
        .iter()
        .map(|round| {
            let mut new_cards = [Card::J; 5];
            for (i, card) in round.cards.into_iter().enumerate() {
                new_cards[i] = match card {
                    Card::J => Card::Joker,
                    _ => card,
                }
            }
            Round {
                cards: new_cards,
                hand: Hand::best(&new_cards),
                bid: round.bid,
            }
        })
        .sorted()
        .collect();

    v.iter()
        .enumerate()
        .map(|(i, round)| ((i + 1) as u32) * round.bid)
        .sum()
}
