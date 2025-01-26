// https://adventofcode.com/2024/day/3

use regex::Regex;
use std::fs::read_to_string;

fn main() {
    let input = read_to_string("inputs/03.in").unwrap();
    let mul_rgx = Regex::new(r"(mul\((\d{1,3}),(\d{1,3})\))|(do\(\))|(don\'t\(\))").unwrap();
    let mut p1 = 0;
    let mut p2 = 0;
    let mut enabled = true;

    for caps in mul_rgx.captures_iter(&input) {
        match caps.get(0).unwrap().as_str() {
            "don't()" => enabled = false,
            "do()" => enabled = true,
            _ => {
                if let (Some(n1_str), Some(n2_str)) = (caps.get(2), caps.get(3)) {
                    if let (Ok(n1), Ok(n2)) = (
                        n1_str.as_str().parse::<i32>(),
                        n2_str.as_str().parse::<i32>(),
                    ) {
                        p1 += n1 * n2;
                        if enabled {
                            p2 += n1 * n2;
                        }
                    }
                }
            }
        }
    }

    println!("2024 Day 03:");
    println!("  Part 1: {p1}");
    println!("  Part 2: {p2}");
}
