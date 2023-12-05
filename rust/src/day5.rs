use aoc_runner_derive::{aoc, aoc_generator};
use rayon::prelude::*;

pub struct Data {
    seeds: Vec<u64>,
    maps: Vec<Vec<(u64, u64, u64)>>,
}

#[aoc_generator(day5)]
pub fn generate_input(input: &str) -> Data {
    let lines: Vec<_> = input
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .collect();

    let seeds = lines[0]
        .split_once(":")
        .unwrap()
        .1
        .trim()
        .split(" ")
        .map(|s| s.trim().parse::<u64>().unwrap())
        .collect();

    let mut current = vec![];
    let mut maps = vec![];
    for line in lines[2..].iter() {
        if line.ends_with(":") {
            maps.push(current.clone());
            current.clear();
        } else {
            let nums: Vec<_> = line
                .split(" ")
                .map(|s| s.trim().parse::<u64>().unwrap())
                .collect();
            current.push((nums[0], nums[1], nums[2]))
        }
    }
    maps.push(current);

    Data { seeds, maps }
}

fn apply_r(maps: &Vec<Vec<(u64, u64, u64)>>, seed: u64) -> u64 {
    let mut result = seed;
    for map in maps {
        for (dst, src, len) in map.iter() {
            if src <= &result && result <= src + len - 1 {
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
    let seeds: Vec<_> = data.seeds.chunks(2).map(|v| (v[0], v[1])).collect();

    seeds
        .into_par_iter()
        .map(|(st, l)| {
            (st..=st + l - 1)
                .map(|x| apply_r(&data.maps, x))
                .min()
                .unwrap()
        })
        .min()
        .unwrap()
}
