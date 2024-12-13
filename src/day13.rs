use crate::common::Position;
use failure::Error;

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

impl Machine {
    fn min_tokens(&self) -> Option<i64> {
        (0..=100)
            .map(|a_presses| (a_presses, self.button_a * a_presses))
            .take_while(|(_, a_pos)| a_pos.x <= self.prize.x && a_pos.y <= self.prize.y)
            .filter_map(|(a_presses, a_pos)| {
                let diff = self.prize - a_pos;
                if diff.x % self.button_b.x == 0 {
                    let b_presses = diff.x / self.button_b.x;
                    if b_presses <= 100 && self.button_b.y * b_presses == diff.y {
                        Some((a_presses, b_presses))
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .map(|(a_presses, b_presses)| a_presses * 3 + b_presses)
            .min()
    }
}

fn get_total_tokens(machines: &[Machine]) -> i64 {
    machines.iter().filter_map(Machine::min_tokens).sum()
}

pub struct Solver {}

impl super::Solver for Solver {
    type Problem = Box<[Machine]>;

    fn parse_input(data: String) -> Result<Self::Problem, Error> {
        parse::parse_input(&data)
    }

    fn solve(machines: Self::Problem) -> (Option<String>, Option<String>) {
        let part1 = get_total_tokens(&machines);
        (Some(part1.to_string()), None)
    }
}
