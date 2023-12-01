use anyhow::Result;
use std::fs::read_to_string;

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

fn calibrate(nums: Vec<u32>) -> u32 {
    nums.first().expect("Num must exist") * 10 + nums.last().expect("Num must exist")
}

pub fn solve_b(filename: &str) -> Result<u32> {
    let text = read_to_string(filename)?;
    let lines = text.lines();
    let nums: Vec<_> = lines
        .map(get_digits)
        .map(calibrate)
        .collect();

    Ok(nums.iter().sum())
}

pub fn solve_a(filename: &str) -> Result<u32> {
    let text = read_to_string(filename)?;
    let lines = text.lines();
    let nums: Vec<_> = lines
        .map(|line| {
            line.chars()
                .filter(|c| c.is_numeric())
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .map(calibrate)
        .collect();

    Ok(nums.iter().sum())
}
