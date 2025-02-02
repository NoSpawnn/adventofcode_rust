// https://adventofcode.com/2024/day/7

use std::{fs::read_to_string, time};

#[derive(Debug, PartialEq, Eq)]
enum Operator {
    Add,
    Mult,
    Concat,
}

impl Operator {
    const COUNT: u64 = 3;

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
        let nums: Vec<u64> = nums.split_whitespace().flat_map(&str::parse).collect();
        let len = nums.len();
        let mut updated_p1 = false;
        let mut updated_p2 = false;

        // Generate all possible equations - https://www.reddit.com/r/rust/comments/91h6t8/generating_all_possible_case_variations_of_a/
        for i in 0..u64::pow(Operator::COUNT, len as u32) {
            let mut steps = i;
            let mut has_concat = false;
            let equation = (0..len).map(|_| {
                let op = match steps % 3 {
                    0 => Operator::Add,
                    1 => Operator::Mult,
                    2 => {
                        has_concat = true;
                        Operator::Concat
                    }
                    _ => unreachable!(),
                };
                steps /= 3;
                op
            });

            if let Some(result) = equation.zip(nums.iter()).try_fold(0u64, |acc, (op, n)| {
                let next_acc = op.operate(acc, *n);
                (next_acc <= target).then_some(next_acc)
            }) {
                if result != target {
                    continue;
                }

                if !updated_p1 && !has_concat {
                    p1 += target;
                    updated_p1 = true;
                }

                if !updated_p2 {
                    p2 += target;
                    updated_p2 = true;
                }

                if updated_p1 && updated_p2 {
                    break;
                }
            }
        }
    }

    let end = start.elapsed();
    println!("2024 Day 07:");
    println!("  Part 1: {p1}");
    println!("  Part 2: {p2}");
    println!("    Time: {end:.2?}");
}
