// https://adventofcode.com/2024/day/2

use std::fs::read_to_string;

fn main() {
    let input = read_to_string("inputs/day02.in").unwrap();
    let nums: Vec<Vec<i32>> = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect()
        })
        .collect();

    let p1 = &nums.iter().filter(|nums| check(nums)).count();
    let p2 = &nums
        .iter()
        .filter(|nums| {
            check(nums)
                || (0..nums.len() - 1)
                    .map(|i| {
                        nums.iter()
                            .enumerate()
                            .filter_map(|(cur, n)| if cur != i { Some(*n) } else { None })
                            .collect::<Vec<i32>>()
                    })
                    .find(|with_removed| check(with_removed))
                    .is_some()
        })
        .count();

    println!("2024 Day 02:");
    println!("  Part 1: {p1}");
    println!("  Part 2: {p2}");
}

fn check(nums: &Vec<i32>) -> bool {
    let pairs = nums.iter().zip(nums.iter().skip(1));
    let safe_score = pairs.len();
    let actual_score = pairs
        .map(|(prev, this)| {
            let diff = prev - this;
            if diff.abs() > 3 {
                0
            } else {
                match diff {
                    1..=3 => 1,
                    0 => 0,
                    _ => -1,
                }
            }
        })
        .sum::<i32>()
        .abs();

    actual_score == safe_score as i32
}
