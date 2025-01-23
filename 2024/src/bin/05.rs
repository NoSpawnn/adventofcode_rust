// https://adventofcode.com/2024/day/5

use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
};

type AfterMap = HashMap<i32, HashSet<i32>>;

fn main() {
    let input = read_to_string("inputs/05.in").unwrap();
    let (rules, updates) = input.split_once("\n\n").unwrap();

    let mut afters: AfterMap = HashMap::new();
    for rule in rules.lines() {
        let (x, y) = rule
            .split_once('|')
            .and_then(|(x, y)| Some((x.parse::<i32>().unwrap(), y.parse::<i32>().unwrap())))
            .unwrap();
        afters.entry(x).or_insert(HashSet::new()).insert(y);
    }

    let mut p1 = 0;
    let mut p2 = 0;
    for update in updates.lines() {
        let nums: Vec<i32> = update
            .split(',')
            .map(|n| n.parse::<i32>().unwrap())
            .collect();

        if is_valid(&nums, &afters) {
            p1 += nums.get(nums.len() / 2).unwrap();
        } else {
            p2 += get_correct_order(&nums, &afters)
                .get(nums.len() / 2)
                .unwrap();
        }
    }

    println!("2024 Day 05:");
    println!("  Part 1: {p1}");
    println!("  Part 2: {p2}");
}

fn is_valid(nums: &Vec<i32>, afters: &AfterMap) -> bool {
    let mut seen: HashSet<i32> = HashSet::new();

    for num in nums {
        seen.insert(*num);

        if !afters.contains_key(num) {
            continue;
        }

        if afters.get(num).unwrap().iter().any(|n| seen.contains(n)) {
            return false;
        }
    }

    true
}

fn get_correct_order(nums: &Vec<i32>, afters: &AfterMap) -> Vec<i32> {
    let correct = Vec::new();

    correct
}
