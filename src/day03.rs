use failure::Error;

use regex::Regex;

pub struct Solver {}

fn calculate(data: &str, always_enabled: bool) -> u64 {
    let re = Regex::new(r"mul\((\d+),(\d+)\)|do\(\)|don't\(\)").unwrap();

    re.captures_iter(data)
        .fold((0, true), |(mut total, mut enabled), instruction| {
            let command = instruction[0].split_once("(").unwrap().0;
            match command {
                "mul" => {
                    if enabled {
                        let x: u64 = instruction[1].parse().unwrap();
                        let y: u64 = instruction[2].parse().unwrap();
                        total += x * y
                    }
                }
                "do" => enabled = true,
                "don't" => enabled = always_enabled,
                cmd => panic!("Unexpected command: {}", cmd),
            }
            (total, enabled)
        })
        .0
}

impl super::Solver for Solver {
    type Problem = String;

    fn parse_input(data: String) -> Result<Self::Problem, Error> {
        Ok(data)
    }

    fn solve(data: Self::Problem) -> (Option<String>, Option<String>) {
        let part1 = calculate(&data, true);
        let part2 = calculate(&data, false);

        (Some(part1.to_string()), Some(part2.to_string()))
    }
}
