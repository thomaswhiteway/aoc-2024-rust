use failure::{err_msg, Error};
use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct DeltaSeq(u32);

impl From<(i8, i8, i8, i8)> for DeltaSeq {
    fn from((a, b, c, d): (i8, i8, i8, i8)) -> Self {
        DeltaSeq(
            ((a as u8) as u32) << 24
                | ((b as u8) as u32) << 16
                | ((c as u8) as u32) << 8
                | (d as u8) as u32,
        )
    }
}

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
        self.0 = prune(mix(self.0, self.0 << 6));
        self.0 = prune(mix(self.0, self.0 >> 5));
        self.0 = prune(mix(self.0, self.0 << 11));

        Some(val)
    }
}

fn price_seq(num: i64) -> impl Iterator<Item = i8> {
    SecretNumberSequence(num).map(|n| (n % 10) as i8)
}

fn find_secret_number_sum(numbers: &[i64], index: usize) -> i64 {
    numbers
        .iter()
        .map(|&num| SecretNumberSequence(num).nth(index).unwrap())
        .sum()
}

fn update_delta_seq_prices(num: i64, count: usize, prices: &mut HashMap<DeltaSeq, i64>) {
    let mut seen = HashSet::new();

    let delta_seqs = price_seq(num)
        .take(count)
        .tuple_windows()
        .map(|(p1, p2)| p2 - p1)
        .tuple_windows()
        .map(|seq: (i8, i8, i8, i8)| seq.into());

    for (delta_seq, price) in delta_seqs.zip(price_seq(num).skip(4)) {
        if !seen.contains(&delta_seq) {
            seen.insert(delta_seq);
            *prices.entry(delta_seq).or_default() += price as i64;
        }
    }
}

fn max_num_bananas(numbers: &[i64], max_numbers: usize) -> i64 {
    let mut delta_seq_prices = HashMap::new();

    for &num in numbers {
        update_delta_seq_prices(num, max_numbers + 1, &mut delta_seq_prices);
    }

    *delta_seq_prices.values().max().unwrap()
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
