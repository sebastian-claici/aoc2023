use anyhow::Result;
use std::fs::read_to_string;

pub fn get_digits(line: &str) -> Vec<u32> {
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

pub fn solve_b(filename: &str) -> Result<u32> {
    let text = read_to_string(filename)?;
    let lines = text.lines();
    let nums: Vec<_> = lines
        .map(|line| get_digits(line))
        .map(|nums| {
            nums.first().expect("Num must exist") * 10 + nums.last().expect("Num must exist")
        })
        .collect();

    Ok(nums.iter().sum())
}

pub fn solve_a(filename: &str) -> Result<u32> {
    let text = read_to_string(filename)?;
    let lines = text.lines();
    let nums: Vec<_> = lines
        .map(|line| line.chars().filter(|c| c.is_numeric()).collect::<Vec<_>>())
        .map(|nums| {
            nums.first().expect("Num must exist").to_digit(10).unwrap() * 10
                + nums.last().expect("Num must exist").to_digit(10).unwrap()
        })
        .collect();

    Ok(nums.iter().sum())
}
