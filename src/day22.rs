use failure::{err_msg, Error};
use itertools::{iproduct, Itertools};
use std::{
    collections::{hash_map, HashMap},
    str::FromStr,
};

fn mix(a: i64, b: i64) -> i64 {
    a ^ b
}

fn prune(a: i64) -> i64 {
    a % 16777216
}

#[derive(Clone, Debug, Copy)]
struct SecretNumberSequence(i64);

impl Iterator for SecretNumberSequence {
    type Item = i64;

    fn next(&mut self) -> Option<Self::Item> {
        let val = self.0;

        self.0 = prune(mix(self.0, self.0 * 64));
        self.0 = prune(mix(self.0, self.0 / 32));
        self.0 = prune(mix(self.0, self.0 * 2048));

        Some(val)
    }
}

fn price_seq(num: i64) -> impl Iterator<Item = i64> {
    SecretNumberSequence(num).map(|n| n % 10)
}

fn find_secret_number_sum(numbers: &[i64], index: usize) -> i64 {
    numbers
        .iter()
        .map(|&num| SecretNumberSequence(num).nth(index).unwrap())
        .sum()
}

fn find_delta_seq_prices(num: i64, count: usize) -> HashMap<(i64, i64, i64, i64), i64> {
    let mut delta_seq_prices = HashMap::new();

    let delta_seqs = price_seq(num)
        .take(count)
        .tuple_windows()
        .map(|(p1, p2)| p2 - p1)
        .tuple_windows();
    for (delta_seq, price) in delta_seqs.zip(price_seq(num).skip(4)) {
        if let hash_map::Entry::Vacant(vacant) = delta_seq_prices.entry(delta_seq) {
            vacant.insert(price);
        }
    }

    delta_seq_prices
}

fn max_num_bananas(numbers: &[i64], max_numbers: usize) -> i64 {
    let delta_seq_prices: Vec<_> = numbers
        .iter()
        .map(|&num| find_delta_seq_prices(num, max_numbers + 1))
        .collect();

    iproduct!(-9..=9, -9..=9, -9..=9, -9..=9)
        .filter(|&(a, b,c, d)| (-9..=9).contains(&(a + b + c + d)))
        .map(|delta_seq| {
            delta_seq_prices
                .iter()
                .filter_map(|prices| prices.get(&delta_seq))
                .sum::<i64>()
        })
        .max()
        .unwrap()
}

pub struct Solver {}

impl super::Solver for Solver {
    type Problem = Box<[i64]>;

    fn parse_input(data: String) -> Result<Self::Problem, Error> {
        data.lines()
            .map(i64::from_str)
            .collect::<Result<Vec<_>, _>>()
            .map(Vec::into_boxed_slice)
            .map_err(|err| err_msg(format!("Failed to parse input: {}", err)))
    }

    fn solve(numbers: Self::Problem) -> (Option<String>, Option<String>) {
        let part1 = find_secret_number_sum(&numbers, 2000);
        let part2 = max_num_bananas(&numbers, 2000);
        (Some(part1.to_string()), Some(part2.to_string()))
    }
}
