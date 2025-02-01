// https://adventofcode.com/2024/day/6

use std::{collections::HashSet, fs::read_to_string, time};

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

trait CheckedIncrement {
    fn inc_unless_max(self, max: usize) -> Option<usize>;
}

impl CheckedIncrement for usize {
    fn inc_unless_max(self, max: usize) -> Option<usize> {
        if self + 1 < max {
            Some(self + 1)
        } else {
            None
        }
    }
}

fn main() {
    let input = read_to_string("inputs/06.in").unwrap();
    let start = time::Instant::now();

    let grid: Vec<_> = input.lines().collect();
    let mut obstacles: HashSet<(usize, usize)> = HashSet::new();
    let mut start_pos: Option<(usize, usize)> = None;
    for (r, line) in grid.iter().enumerate() {
        for (c, chr) in line.char_indices() {
            if chr == '^' {
                start_pos = Some((r, c));
            } else if chr == '#' {
                obstacles.insert((r, c));
            }
        }
    }
    let start_pos = start_pos.unwrap();
    let grid_size = (grid.len(), grid[0].len());

    let (visited, _) = patrol(grid_size, &obstacles, start_pos);
    let p1 = visited.len();
    let p2 = {
        visited
            .iter()
            .filter(|&pos| {
                obstacles.insert(*pos);
                let (_, is_loop) = patrol(grid_size, &obstacles, start_pos);
                obstacles.remove(pos);
                is_loop
            })
            .count()
    };

    let end = start.elapsed();
    println!("2024 Day 06:");
    println!("  Part 1: {p1}");
    println!("  Part 2: {p2}");
    println!("    Time: {end:.2?}");
}

fn patrol(
    (rows, cols): (usize, usize),
    obstacles: &HashSet<(usize, usize)>,
    start_pos: (usize, usize),
) -> (HashSet<(usize, usize)>, bool) {
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut seen_obstacles: HashSet<(usize, usize, Dir)> = HashSet::new();
    let mut facing = Dir::Up;
    let mut current_pos = start_pos;

    loop {
        visited.insert(current_pos);

        let (next_row, next_col) = match facing {
            Dir::Up => (current_pos.0.checked_sub(1), Some(current_pos.1)),
            Dir::Down => (current_pos.0.inc_unless_max(rows), Some(current_pos.1)),
            Dir::Left => (Some(current_pos.0), current_pos.1.checked_sub(1)),
            Dir::Right => (Some(current_pos.0), current_pos.1.inc_unless_max(cols)),
        };

        if next_row.is_none() || next_col.is_none() {
            break;
        }

        let (next_row, next_col) = (next_row.unwrap(), next_col.unwrap());
        if seen_obstacles.contains(&(next_row, next_col, facing)) {
            return (visited, true);
        }

        let next_pos = (next_row, next_col);
        if obstacles.contains(&next_pos) {
            seen_obstacles.insert((next_row, next_col, facing));
            facing = match facing {
                Dir::Up => Dir::Right,
                Dir::Down => Dir::Left,
                Dir::Left => Dir::Up,
                Dir::Right => Dir::Down,
            }
        } else {
            visited.insert(next_pos);
            current_pos = next_pos;
        }
    }

    (visited, false)
}
