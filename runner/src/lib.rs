use std::{fs::read_to_string, path::PathBuf};

#[macro_export]
macro_rules! year {
    ($($day:ident), +) => {
        $(pub mod $day;)+
    };
}

#[macro_export]
macro_rules! run {
    ($year:ident, $day:ident) => {
        use runner::Solution;
        let s = Solution::new(stringify!($year), stringify!($day), {
            y$year::day$day::solve
        });
        println!("{:?}", s.solve());
    };
}

pub struct Solution {
    pub year: String,
    pub day: String,
    input_file: PathBuf,
    solve_func: fn(&str) -> (i32, i32),
}

impl Solution {
    pub fn new(year: &str, day: &str, solve_func: fn(&str) -> (i32, i32)) -> Self {
        Self {
            year: year.to_owned(),
            day: day.to_owned(),
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
