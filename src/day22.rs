use failure::{err_msg, Error};
use std::str::FromStr;

fn mix(a: u64, b: u64) -> u64 {
    a ^ b
}

fn prune(a: u64) -> u64 {
    a % 16777216
}

struct SecretNumberSequence(u64);

impl Iterator for SecretNumberSequence {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let val = self.0;

        self.0 = prune(mix(self.0, self.0 * 64));
        self.0 = prune(mix(self.0, self.0 / 32));
        self.0 = prune(mix(self.0, self.0 * 2048));

        Some(val)
    }
}

fn find_secret_number_sum(numbers: &[u64], index: usize) -> u64 {
    numbers
        .iter()
        .map(|&num| SecretNumberSequence(num).nth(index).unwrap())
        .sum()
}

pub struct Solver {}

impl super::Solver for Solver {
    type Problem = Box<[u64]>;

    fn parse_input(data: String) -> Result<Self::Problem, Error> {
        data.lines()
            .map(u64::from_str)
            .collect::<Result<Vec<_>, _>>()
            .map(Vec::into_boxed_slice)
            .map_err(|err| err_msg(format!("Failed to parse input: {}", err)))
    }

    fn solve(numbers: Self::Problem) -> (Option<String>, Option<String>) {
        let part1 = find_secret_number_sum(&numbers, 2000);
        (Some(part1.to_string()), None)
    }
}
