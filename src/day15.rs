use std::collections::HashMap;

use crate::common::{Direction, Position};
use failure::{err_msg, Error};

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum Object {
    Wall,
    Box,
}

#[allow(unused)]
fn display_map(robot_position: Position, objects: &HashMap<Position, Object>) {
    let max_x = objects.keys().map(|pos| pos.x).max().unwrap();
    let max_y = objects.keys().map(|pos| pos.y).max().unwrap();

    for y in 0..=max_y {
        for x in 0..=max_x {
            let pos = Position { x, y };

            print!(
                "{}",
                if pos == robot_position {
                    "@"
                } else {
                    match objects.get(&pos) {
                        None => ".",
                        Some(Object::Box) => "O",
                        Some(Object::Wall) => "#",
                    }
                }
            );
        }
        println!();
    }

    println!();
}

fn next_free_position(
    robot_position: Position,
    objects: &HashMap<Position, Object>,
    direction: Direction,
) -> Option<Position> {
    (1..)
        .find_map(|steps| {
            let this_position = robot_position.step_by(direction, steps);
            match objects.get(&this_position) {
                None => Some(Some(this_position)),
                Some(Object::Wall) => Some(None),
                Some(Object::Box) => None,
            }
        })
        .unwrap()
}

fn perform_move(
    robot_position: &mut Position,
    objects: &mut HashMap<Position, Object>,
    direction: Direction,
) {
    if let Some(next_free) = next_free_position(*robot_position, objects, direction) {
        let next_step = robot_position.step(direction);
        if let Some(object) = objects.remove(&next_step) {
            objects.insert(next_free, object);
        }
        *robot_position = next_step;
    }
}

fn perform_moves(
    robot_position: &mut Position,
    objects: &mut HashMap<Position, Object>,
    moves: &[Direction],
) {
    for &next_move in moves {
        perform_move(robot_position, objects, next_move);
    }
}

fn find_box_location_sum(
    mut robot_position: Position,
    mut objects: HashMap<Position, Object>,
    moves: &[Direction],
) -> i64 {
    perform_moves(&mut robot_position, &mut objects, moves);
    objects
        .iter()
        .filter_map(|(&position, &object)| {
            if object == Object::Box {
                Some(position)
            } else {
                None
            }
        })
        .map(|pos| 100 * pos.y + pos.x)
        .sum()
}

pub struct Solver {}

impl super::Solver for Solver {
    type Problem = (Position, HashMap<Position, Object>, Box<[Direction]>);

    fn parse_input(data: String) -> Result<Self::Problem, Error> {
        let lines = data.lines().collect::<Vec<_>>();

        let start = lines
            .iter()
            .enumerate()
            .find_map(|(y, line)| {
                line.char_indices()
                    .find_map(|(x, c)| if c == '@' { Some((x, y).into()) } else { None })
            })
            .ok_or(err_msg("Failed to find start position"))?;

        let objects = lines
            .iter()
            .take_while(|line| !line.is_empty())
            .enumerate()
            .flat_map(|(y, line)| {
                line.char_indices().filter_map(move |(x, c)| match c {
                    '#' => Some(((x, y).into(), Object::Wall)),
                    'O' => Some(((x, y).into(), Object::Box)),
                    _ => None,
                })
            })
            .collect();

        let moves = lines
            .iter()
            .skip_while(|line| !line.is_empty())
            .skip(1)
            .flat_map(|line| line.chars())
            .map(Direction::try_from)
            .collect::<Result<Vec<_>, _>>()?
            .into_boxed_slice();

        Ok((start, objects, moves))
    }

    fn solve((robot_position, objects, moves): Self::Problem) -> (Option<String>, Option<String>) {
        let part1 = find_box_location_sum(robot_position, objects, &moves);
        (Some(part1.to_string()), None)
    }
}
