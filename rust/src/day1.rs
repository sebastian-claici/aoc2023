use anyhow::Result;
use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<String> {
    input.lines().map(|s| s.trim().to_string()).collect()
}

fn get_digits(line: &str) -> Vec<u32> {
    let nums = vec![
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let digits = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];

    let mut result = vec![];
    for (i, c) in line.chars().enumerate() {
        if c.is_numeric() {
            result.push(c.to_digit(10).unwrap());
            continue;
        }
        for (j, num) in nums.iter().enumerate() {
            if line[i..].starts_with(num) {
                result.push(digits[j]);
                break;
            }
        }
    }
    result
}

fn calibrate(nums: &[u32]) -> u32 {
    nums.first().expect("Num must exist") * 10 + nums.last().expect("Num must exist")
}

#[aoc(day1, part2)]
pub fn solve_b(input: &[String]) -> u32 {
    input
        .iter()
        .map(|line| get_digits(&line))
        .map(|digits| calibrate(&digits))
        .sum()
}

#[aoc(day1, part1)]
pub fn solve_a(input: &[String]) -> u32 {
    input
        .iter()
        .map(|line| {
            line.chars()
                .filter(|c| c.is_numeric())
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .map(|digits| calibrate(&digits))
        .sum()
}
