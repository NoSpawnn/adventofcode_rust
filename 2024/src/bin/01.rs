// https://adventofcode.com/2024/day/1

use std::collections::HashMap;
use std::fs::read_to_string;
use std::iter::zip;
use std::time;

fn main() {
    let input = read_to_string("inputs/01.in").unwrap();
    let start = time::Instant::now();

    let (mut left, mut right): (Vec<_>, Vec<_>) = input
        .lines()
        .map(|line| {
            let (a, b) = line.split_once("   ").unwrap();
            (a.parse::<i32>().unwrap(), b.parse::<i32>().unwrap())
        })
        .unzip();

    left.sort_unstable();
    right.sort_unstable();

    let mut occurrences = HashMap::new();
    for n in &right {
        *occurrences.entry(n).or_insert(0) += 1;
    }

    let p1 = zip(&left, &right).map(|(l, r)| (l - r).abs()).sum::<i32>();
    let p2 = &left
        .iter()
        .map(|n| occurrences.get(n).unwrap_or(&0) * n)
        .sum::<i32>();

    let end = start.elapsed();
    println!("2024 Day 01:");
    println!("  Part 1: {p1}");
    println!("  Part 2: {p2}");
    println!("    Time: {end:.2?}");
}
