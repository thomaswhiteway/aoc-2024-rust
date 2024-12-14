use std::cmp::Ordering;

use failure::{err_msg, Error};
use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::newline,
    combinator::{all_consuming, map},
    multi::many1,
    sequence::{preceded, separated_pair, terminated},
    IResult,
};

use crate::{common::Position, parsers::signed};

pub struct Robot {
    position: Position,
    velocity: Position,
}

impl Robot {
    fn position_after(&self, seconds: i64, size: Position) -> Position {
        let mut position = self.position + self.velocity * seconds;
        position.x %= size.x;
        position.y %= size.y;

        if position.x < 0 {
            position.x += size.x;
        }
        if position.y < 0 {
            position.y += size.y;
        }

        position
    }
}

#[allow(unused)]
fn display_robots(positions: impl Iterator<Item = Position>, size: Position) {
    let counts = positions.map(|pos| (pos, 1)).into_grouping_map().sum();

    for y in 0..size.y {
        for x in 0..size.x {
            let pos = Position { x, y };
            print!(
                "{}",
                counts
                    .get(&pos)
                    .map(|c| c.to_string())
                    .unwrap_or_else(|| ".".to_string())
            );
        }
        println!();
    }
}

fn find_safety_factor(robots: &[Robot], seconds: i64, size: Position) -> usize {
    assert!(size.x % 2 == 1);
    assert!(size.y % 2 == 1);
    let mid = Position {
        x: size.x / 2,
        y: size.y / 2,
    };

    let mut quadrants = robots
        .iter()
        .map(|robot| robot.position_after(seconds, size))
        .map(|pos| (pos.x.cmp(&mid.x), pos.y.cmp(&mid.y)))
        .filter(|(ord_a, ord_b)| !ord_a.is_eq() && !ord_b.is_eq())
        .map(|ord| (ord, 1))
        .into_grouping_map()
        .sum();

    for x_ord in [Ordering::Less, Ordering::Greater] {
        for y_ord in [Ordering::Less, Ordering::Greater] {
            quadrants.entry((x_ord, y_ord)).or_default();
        }
    }

    quadrants.values().product()
}

pub struct Solver {}

impl super::Solver for Solver {
    type Problem = Box<[Robot]>;

    fn parse_input(data: String) -> Result<Self::Problem, Error> {
        fn vec(input: &str) -> IResult<&str, Position> {
            map(
                separated_pair(signed, tag(","), signed),
                |(x, y): (i64, i64)| Position { x, y },
            )(input)
        }
        let position = preceded(tag("p="), vec);
        let velocity = preceded(tag("v="), vec);
        let robot = map(
            separated_pair(position, tag(" "), velocity),
            |(position, velocity)| Robot { position, velocity },
        );
        let robots = many1(terminated(robot, newline));

        all_consuming(robots)(&data)
            .map(|(_, robots)| robots.into_boxed_slice())
            .map_err(|err| err_msg(format!("Failed to parse input: {}", err)))
    }

    fn solve(robots: Self::Problem) -> (Option<String>, Option<String>) {
        let size = Position { x: 101, y: 103 };
        let part1 = find_safety_factor(&robots, 100, size);
        (Some(part1.to_string()), None)
    }
}

#[cfg(test)]
mod test {
    use super::Robot;
    use crate::common::Position;

    #[test]
    fn test_robot() {
        let robot = Robot {
            position: Position { x: 2, y: 4 },
            velocity: Position { x: 2, y: -3 },
        };

        let size = Position { x: 11, y: 7 };

        assert_eq!(robot.position_after(0, size), Position { x: 2, y: 4 });
        assert_eq!(robot.position_after(1, size), Position { x: 4, y: 1 });
        assert_eq!(robot.position_after(2, size), Position { x: 6, y: 5 });
        assert_eq!(robot.position_after(3, size), Position { x: 8, y: 2 });
        assert_eq!(robot.position_after(4, size), Position { x: 10, y: 6 });
        assert_eq!(robot.position_after(5, size), Position { x: 1, y: 3 });
    }
}
