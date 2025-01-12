// https://adventofcode.com/2024/day/1

use std::collections::HashMap;
use std::iter::zip;
use std::num::ParseIntError;

pub fn solve(input: &str) -> (i32, i32) {
    let pairs: Vec<(i32, i32)> = input.lines().flat_map(|line| read_pair(line)).collect();
    (part_1(&pairs), part_2(&pairs))
}

fn part_1(pairs: &Vec<(i32, i32)>) -> i32 {
    let (mut left, mut right): (Vec<_>, Vec<_>) = pairs.iter().cloned().unzip();

    left.sort_unstable();
    right.sort_unstable();

    zip(left, right).map(|(l, r)| (l - r).abs()).sum()
}

fn part_2(pairs: &Vec<(i32, i32)>) -> i32 {
    let (left, right): (Vec<_>, Vec<_>) = pairs.iter().cloned().unzip();

    let mut occurrences = HashMap::new();
    for n in right {
        *occurrences.entry(n).or_insert(0) += 1;
    }

    left.iter()
        .map(|n| occurrences.get(n).unwrap_or(&0) * n)
        .sum()
}

fn read_pair(line: &str) -> Result<(i32, i32), ParseIntError> {
    let mut ns = line.split_whitespace().take(2);
    match (ns.next(), ns.next()) {
        (Some(a), Some(b)) => Ok((a.parse()?, b.parse()?)),
        _ => panic!("Failed to parse line {line}",),
    }
}
