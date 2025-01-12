use std::{fs::read_to_string, path::PathBuf};

pub type SolutionFunc = fn(&str) -> (i32, i32);

macro_rules! run {
    ($year:ident $day:ident) => {
        include!(y2024::day01::solve)
        let input_file = PathBuf::from(concat!("inputs/", $year, "/day", $day, ".in"));
    };
}

pub struct Solution {
    pub year: String,
    pub day: String,
    input_file: PathBuf,
    solve_func: SolutionFunc,
}

impl Solution {
    pub fn new(year: i32, day: i32, solve_func: SolutionFunc) -> Self {
        Self {
            year: year.to_string(),
            day: format!("{day}:02d"),
            input_file: PathBuf::from(format!("inputs/{year}/day{day}.in")),
            solve_func,
        }
    }

    pub fn solve(self) -> Result<(i32, i32), std::io::Error> {
        match read_to_string(self.input_file) {
            Ok(input) => Ok((self.solve_func)(&input)),
            Err(e) => Err(e),
        }
    }
}
