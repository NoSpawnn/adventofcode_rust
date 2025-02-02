// https://adventofcode.com/2024/day/7

use std::{collections::HashMap, fs::read_to_string, time};

#[derive(Debug, PartialEq, Eq)]
enum Operator {
    Add,
    Mult,
    Concat,
}

impl Operator {
    fn operate(&self, lhs: u64, rhs: u64) -> u64 {
        match self {
            Operator::Add => lhs + rhs,
            Operator::Mult => lhs * rhs,
            Operator::Concat => format!("{}{}", lhs, rhs).parse().unwrap(),
        }
    }
}

fn main() {
    let input = read_to_string("inputs/07.in").unwrap();
    let start = time::Instant::now();

    let mut p1 = 0;
    let mut p2 = 0;
    for line in input.lines() {
        let (target, nums) = line.split_once(':').unwrap();
        let target: u64 = target.parse().unwrap();
        let nums: Vec<u64> = nums.split_whitespace().flat_map(|n| n.parse()).collect();
        let len = nums.len();
        let mut updated_p1 = false;
        let mut updated_p2 = false;

        // Generate all possible equations - https://www.reddit.com/r/rust/comments/91h6t8/generating_all_possible_case_variations_of_a/
        for i in 0..u64::pow(3, len as u32) {
            let mut steps = i;
            let mut has_concat = false;
            let mut case = Vec::with_capacity(len);
            for _ in 0..len {
                case.push(match steps % 3 {
                    0 => Operator::Add,
                    1 => Operator::Mult,
                    2 => {
                        has_concat = true;
                        Operator::Concat
                    }
                    _ => unreachable!(),
                });
                steps /= 3;
            }

            let valid = case
                .iter()
                .enumerate()
                .fold(0u64, |acc, (i, op)| op.operate(acc, nums[i]))
                == target;

            if !updated_p1 && !has_concat && valid {
                p1 += target;
                updated_p1 = true;
            }

            if !updated_p2 && valid {
                p2 += target;
                updated_p2 = true;
            }

            if updated_p1 && updated_p2 {
                break;
            }
        }
    }

    let end = start.elapsed();
    println!("2024 Day 07:");
    println!("  Part 1: {p1}");
    println!("  Part 2: {p2}");
    println!("    Time: {end:.2?}");
}
