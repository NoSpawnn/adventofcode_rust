// https://adventofcode.com/2024/day/4

use std::fs::read_to_string;

const XMAS: &str = "XMAS";
const SAMX: &str = "SAMX";

fn main() {
    let lines = read_to_string("inputs/day04.in")
        .unwrap()
        .lines()
        .map(|l| l.to_owned())
        .collect::<Vec<_>>();

    println!("{}", lines.iter().map(|row| transpose(&row)).sum::<i32>());
}

fn count_in_row(row: &str) -> i32 {
    let f_matches = row.match_indices(XMAS).count();
    let r_matches = row.match_indices(SAMX).count();

    (f_matches + r_matches) as i32
}

fn transpose(lines: &[&str]) -> Vec<String> {
    let mut transposed = vec![String::new(); lines.len()];

    for line in lines.iter() {
        for (i, c) in line.chars().enumerate() {
            transposed[i].push(c);
        }
    }

    transposed
}
