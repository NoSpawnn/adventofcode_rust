// https://adventofcode.com/2024/day/4

use std::{fs::read_to_string, time};

const CARDINAL: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
const CORNERS: [(i32, i32); 4] = [(-1, -1), (-1, 1), (1, 1), (1, -1)];

fn main() {
    let input = read_to_string("inputs/04.in").unwrap();
    let start = time::Instant::now();

    let lines = input.lines().map(|l| l.to_owned()).collect::<Vec<_>>();
    let directions = &[CARDINAL, CORNERS].concat();

    let mut p1 = 0;
    let mut p2 = 0;
    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.char_indices() {
            match c {
                'X' => {
                    p1 += directions
                        .iter()
                        .filter(|&&dir| search(&lines, "XMAS", x as i32, y as i32, dir))
                        .count()
                }
                'A' => {
                    p2 += match CORNERS
                        .iter()
                        .map(|(dx, dy)| safe_get_at(&lines, x as i32 + *dx, y as i32 + *dy))
                        .collect::<String>()
                        .as_str()
                    {
                        "MMSS" | "MSSM" | "SSMM" | "SMMS" => 1,
                        _ => 0,
                    }
                }
                _ => continue,
            }
        }
    }

    let end = start.elapsed();
    println!("2024 Day 04:");
    println!("  Part 1: {p1}");
    println!("  Part 2: {p2}");
    println!("    Time: {end:.2?}");
}

fn search(haystack: &Vec<String>, needle: &str, x: i32, y: i32, dir: (i32, i32)) -> bool {
    if needle.is_empty() {
        return true;
    }

    if char_at(needle, 0) != safe_get_at(haystack, x, y) {
        return false;
    }

    search(haystack, &needle[1..], x + dir.0, y + dir.1, dir)
}

fn char_at(s: &str, i: usize) -> char {
    s.chars().nth(i).unwrap()
}

fn safe_get_at(grid: &[String], x: i32, y: i32) -> char {
    if (y as usize) < grid.len() && (x as usize) < grid.first().unwrap().len() {
        char_at(&grid[y as usize], x as usize)
    } else {
        ' '
    }
}
