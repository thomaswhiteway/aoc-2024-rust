use crate::common::Position;
use failure::Error;
use num::rational::Ratio;

mod parse {
    use failure::{err_msg, Error};
    use nom::{
        bytes::complete::tag,
        character::complete::newline,
        combinator::{all_consuming, map},
        multi::separated_list1,
        sequence::{delimited, preceded, separated_pair, tuple},
        IResult,
    };

    use crate::{common::Position, parsers::signed};

    use super::Machine;

    fn button(input: &str) -> IResult<&str, Position> {
        map(
            separated_pair(
                preceded(tag("X+"), signed),
                tag(", "),
                preceded(tag("Y+"), signed),
            ),
            Position::from,
        )(input)
    }

    fn prize(input: &str) -> IResult<&str, Position> {
        map(
            separated_pair(
                preceded(tag("X="), signed),
                tag(", "),
                preceded(tag("Y="), signed),
            ),
            Position::from,
        )(input)
    }

    fn machine(input: &str) -> IResult<&str, Machine> {
        map(
            tuple((
                delimited(tag("Button A: "), button, newline),
                delimited(tag("Button B: "), button, newline),
                delimited(tag("Prize: "), prize, newline),
            )),
            |(button_a, button_b, prize)| Machine {
                button_a,
                button_b,
                prize,
            },
        )(input)
    }

    fn machines(input: &str) -> IResult<&str, Box<[Machine]>> {
        map(separated_list1(newline, machine), Vec::into_boxed_slice)(input)
    }

    pub(super) fn parse_input(input: &str) -> Result<Box<[Machine]>, Error> {
        all_consuming(machines)(input)
            .map(|(_, machines)| machines)
            .map_err(|err| err_msg(format!("Failed to parse input: {}", err)))
    }
}

pub struct Machine {
    button_a: Position,
    button_b: Position,
    prize: Position,
}

fn is_valid_presses(presses: Ratio<i64>, max_presses: Option<i64>) -> bool {
    if presses < Ratio::ZERO {
        return false;
    }

    if !presses.is_integer() {
        return false;
    }

    if max_presses
        .map(Ratio::from_integer)
        .map(|p| presses > p)
        .unwrap_or_default()
    {
        return false;
    }

    true
}

impl Machine {
    fn with_prize_offset(&self, delta: Position) -> Self {
        Machine {
            button_a: self.button_a,
            button_b: self.button_b,
            prize: self.prize + delta,
        }
    }

    fn num_presses(&self, max_presses: Option<i64>) -> Option<(i64, i64)> {
        let determinant = self.button_a.x * self.button_b.y - self.button_a.y * self.button_b.x;
        assert!(determinant != 0);

        let a_presses = Ratio::new(
            self.prize.x * self.button_b.y - self.prize.y * self.button_b.x,
            determinant,
        );
        let b_presses = Ratio::new(
            self.prize.y * self.button_a.x - self.prize.x * self.button_a.y,
            determinant,
        );

        if is_valid_presses(a_presses, max_presses) && is_valid_presses(b_presses, max_presses) {
            Some((a_presses.to_integer(), b_presses.to_integer()))
        } else {
            None
        }
    }

    fn num_tokens(&self, max_presses: Option<i64>) -> Option<i64> {
        self.num_presses(max_presses)
            .map(|(a_presses, b_presses)| a_presses * 3 + b_presses)
    }
}

fn get_total_tokens(machines: &[Machine], max_presses: Option<i64>) -> i64 {
    machines
        .iter()
        .filter_map(|machine| machine.num_tokens(max_presses))
        .sum()
}

pub struct Solver {}

impl super::Solver for Solver {
    type Problem = Box<[Machine]>;

    fn parse_input(data: String) -> Result<Self::Problem, Error> {
        parse::parse_input(&data)
    }

    fn solve(machines: Self::Problem) -> (Option<String>, Option<String>) {
        let part1 = get_total_tokens(&machines, Some(100));
        let delta = Position {
            x: 10000000000000,
            y: 10000000000000,
        };
        let updated_machines: Vec<_> = machines
            .iter()
            .map(|machine| machine.with_prize_offset(delta))
            .collect();
        let part2 = get_total_tokens(&updated_machines, None);
        (Some(part1.to_string()), Some(part2.to_string()))
    }
}
