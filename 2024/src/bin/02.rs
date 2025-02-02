// https://adventofcode.com/2024/day/2

use std::{fs::read_to_string, time};

fn main() {
    let input = read_to_string("inputs/02.in").unwrap();
    let start = time::Instant::now();

    let nums: Vec<Vec<i32>> = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect()
        })
        .collect();

    let p1 = nums.iter().filter(|ns| check(ns)).count();
    let p2 = nums
        .iter()
        .filter(|ns| (0..ns.len()).any(|i| check(&[&ns[..i], &ns[i + 1..]].concat())))
        .count();

    let end = start.elapsed();
    println!("2024 Day 02:");
    println!("  Part 1: {p1}");
    println!("  Part 2: {p2}");
    println!("    Time: {end:.2?}");
}

fn check(nums: &[i32]) -> bool {
    let pairs = nums.iter().zip(nums.iter().skip(1));
    let safe_score = pairs.len() as i32;
    let actual_score = pairs
        .map(|(prev, this)| match (prev - this).abs() {
            0..=3 => (prev - this).clamp(-1, 1),
            _ => 0,
        })
        .sum::<i32>()
        .abs();

    actual_score == safe_score
}
