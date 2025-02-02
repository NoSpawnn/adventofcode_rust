// https://adventofcode.com/2024/day/7

use std::{collections::HashMap, fs::read_to_string, time};

#[derive(Debug)]
enum Operation {
    Add,
    Mult,
}

fn main() {
    let input = read_to_string("inputs/07.in").unwrap();
    let start = time::Instant::now();

    let mut p1 = 0;
    for line in input.lines() {
        let (target, nums) = line.split_once(':').unwrap();
        let target: u64 = target.parse().unwrap();
        let nums: Vec<u64> = nums.split_whitespace().flat_map(|n| n.parse()).collect();
        let len = nums.len();

        // Generate all cases - https://www.reddit.com/r/rust/comments/91h6t8/generating_all_possible_case_variations_of_a/
        for i in 0..u64::pow(2, len as u32) {
            let mut acc = 0;
            for (idx, n) in nums.iter().enumerate() {
                if (i & (1 << idx)) == 0 {
                    acc += n;
                } else {
                    acc *= n;
                }
            }

            if acc == target {
                p1 += target;
                break;
            }
        }
    }

    let mut p2 = 0;

    let end = start.elapsed();
    println!("2024 Day 07:");
    println!("  Part 1: {p1}");
    println!("  Part 2: {p2}");
    println!("    Time: {end:.2?}");
}
