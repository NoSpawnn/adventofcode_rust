// https://adventofcode.com/2024/day/4

use std::fs::read_to_string;

fn main() {
    let input = read_to_string("inputs/day04.in")
        .unwrap()
        .lines()
        .map(|l| l.to_owned())
        .collect::<Vec<_>>();
}
