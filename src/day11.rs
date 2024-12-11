use failure::{err_msg, Error};

fn count_stones(mut stones: Vec<u64>) -> usize {
    for _ in 0..25 {
        let mut new_stones = vec![];

        for val in stones {
            if val == 0 {
                new_stones.push(1);
            } else {
                let width = val.ilog10() + 1;
                if width % 2 == 0 {
                    let base = 10u64.pow(width / 2);
                    new_stones.push(val / base);
                    new_stones.push(val % base);
                } else {
                    new_stones.push(val * 2024);
                }
            }
        }

        stones = new_stones;
    }

    stones.len()
}

pub struct Solver {}

impl super::Solver for Solver {
    type Problem = Vec<u64>;

    fn parse_input(data: String) -> Result<Self::Problem, Error> {
        data.split_whitespace()
            .map(|val| {
                val.parse()
                    .map_err(|_| err_msg(format!("Invalid value {}", val)))
            })
            .collect()
    }

    fn solve(stones: Self::Problem) -> (Option<String>, Option<String>) {
        let part1 = count_stones(stones);
        (Some(part1.to_string()), None)
    }
}
