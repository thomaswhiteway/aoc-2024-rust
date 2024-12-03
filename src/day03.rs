use failure::Error;

use regex::Regex;

pub struct Solver {}

fn calculate(data: &str) -> u64 {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    re.captures_iter(data)
        .map(|m| {
            let x: u64 = m.get(1).unwrap().as_str().parse().unwrap();
            let y: u64 = m.get(2).unwrap().as_str().parse().unwrap();
            x * y
        })
        .sum()
}

impl super::Solver for Solver {
    type Problem = String;

    fn parse_input(data: String) -> Result<Self::Problem, Error> {
        Ok(data)
    }

    fn solve(data: Self::Problem) -> (Option<String>, Option<String>) {
        let part1 = calculate(&data);

        (Some(part1.to_string()), None)
    }
}
