use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{multispace1, space0, space1, u64},
    multi::separated_list0,
    sequence::preceded,
    IResult,
};

#[derive(Debug)]
pub struct Data {
    time: Vec<u64>,
    distance: Vec<u64>,
}

fn parse_vec(input: &str) -> IResult<&str, Vec<u64>> {
    preceded(space0, separated_list0(space1, u64))(input)
}

fn parse_input(input: &str) -> IResult<&str, Data> {
    let (input, time) = preceded(tag("Time: "), parse_vec)(input)?;
    let (input, _) = multispace1(input)?;
    let (input, distance) = preceded(tag("Distance: "), parse_vec)(input)?;

    Ok((input, Data { time, distance }))
}

#[aoc_generator(day6)]
pub fn generate_input(input: &str) -> Data {
    parse_input(input).unwrap().1
}

#[aoc(day6, part1)]
pub fn solve_a(data: &Data) -> usize {
    data.time
        .iter()
        .zip(data.distance.iter())
        .map(|(&time, &distance)| (0..=time).filter(|x| (time - x) * x > distance).count())
        .product()
}

#[aoc(day6, part2)]
pub fn solve_b(data: &Data) -> usize {
    let time = Itertools::join(&mut data.time.iter(), "")
        .parse::<u64>()
        .unwrap();
    let distance = Itertools::join(&mut data.distance.iter(), "")
        .parse::<u64>()
        .unwrap();

    (0..=time).filter(|x| (time - x) * x > distance).count()
}
