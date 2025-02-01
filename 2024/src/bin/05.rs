// https://adventofcode.com/2024/day/5

use std::{cmp::Ordering, collections::HashSet, fs::read_to_string, time};

fn main() {
    let input = read_to_string("inputs/05.in").unwrap();
    let start = time::Instant::now();

    let (rules, updates) = input.split_once("\n\n").unwrap();
    let rules: HashSet<(i32, i32)> = rules
        .lines()
        .map(|line| {
            let (a, b) = line.split_once('|').unwrap();
            (a.parse().unwrap(), b.parse().unwrap())
        })
        .collect();

    let mut p1 = 0;
    let mut p2 = 0;
    for update in updates.lines() {
        let mut nums: Vec<i32> = update.split(',').map(|n| n.parse().unwrap()).collect();
        if nums.is_sorted_by(|a, b| rules.contains(&(*a, *b))) {
            p1 += nums[nums.len() / 2];
        } else {
            nums.sort_by(|a, b| {
                if rules.contains(&(*a, *b)) {
                    Ordering::Less
                } else if rules.contains(&(*b, *a)) {
                    Ordering::Greater
                } else {
                    Ordering::Equal
                }
            });
            p2 += nums[nums.len() / 2];
        }
    }

    let end = start.elapsed();
    println!("2024 Day 05:");
    println!("  Part 1: {p1}");
    println!("  Part 2: {p2}");
    println!("    Time: {end:.2?}");
}
