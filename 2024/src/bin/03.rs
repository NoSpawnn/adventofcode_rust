// https://adventofcode.com/2024/day/3

use regex::Regex;
use std::fs::read_to_string;

fn main() {
    let input = read_to_string("inputs/day03.in").unwrap();
    let mul_rgx = Regex::new(r"(mul\((\d{1,3}),(\d{1,3})\))|(do\(\))|(don\'t\(\))").unwrap();
    let mut p1 = 0;
    let mut p2 = 0;
    let mut enabled = true;

    for caps in mul_rgx.captures_iter(&input) {
        match caps.get(0).unwrap().as_str() {
            "don't()" => enabled = false,
            "do()" => enabled = true,
            _ => {
                let n1: i32 = caps.get(2).unwrap().as_str().parse().unwrap();
                let n2: i32 = caps.get(3).unwrap().as_str().parse().unwrap();
                p1 += n1 * n2;
                if enabled {
                    p2 += n1 * n2
                }
            }
        }
    }

    println!("2024 Day 03:");
    println!("  Part 1: {p1}");
    println!("  Part 2: {p2}");
}
