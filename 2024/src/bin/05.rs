// https://adventofcode.com/2024/day/5

use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
    fs::read_to_string,
};

type AfterMap<'a> = HashMap<&'a str, HashSet<&'a str>>;

fn main() {
    let input = read_to_string("inputs/05.in").unwrap();
    let (rules, updates) = input.split_once("\n\n").unwrap();

    let mut afters: AfterMap = HashMap::new();
    for rule in rules.lines() {
        let (x, y) = rule.split_once('|').unwrap();
        afters.entry(x).or_default().insert(y);
    }

    let mut p1 = 0;
    let mut p2 = 0;
    for update in updates.lines() {
        let mut nums = update.split(',').collect();
        if is_valid(&nums, &afters) {
            p1 += nums[nums.len() / 2].parse::<i32>().unwrap_or(0);
        } else {
            nums.sort_by(|a, b| {
                match (
                    afters.get(a).is_some_and(|s| s.contains(b)),
                    afters.get(b).is_some_and(|s| s.contains(a)),
                ) {
                    (true, false) => Ordering::Less,    // a -> b
                    (false, true) => Ordering::Greater, // b -> a
                    _ => Ordering::Equal,
                }
            });
            p2 += nums[nums.len() / 2].parse::<i32>().unwrap_or(0);
        }
    }

    println!("2024 Day 05:");
    println!("  Part 1: {p1}");
    println!("  Part 2: {p2}");
}

fn is_valid(nums: &Vec<&str>, afters: &AfterMap) -> bool {
    let mut seen: HashSet<&str> = HashSet::new();

    for num in nums {
        seen.insert(*num);

        if !afters.contains_key(num) {
            continue;
        }

        if afters[num].iter().any(|n| seen.contains(n)) {
            return false;
        }
    }

    true
}
