use std::collections::HashMap;

use crate::common::{Direction, Position};
use failure::{err_msg, Error};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Side {
    Left,
    Right,
}

impl Side {
    fn other(self) -> Self {
        match self {
            Side::Left => Side::Right,
            Side::Right => Side::Left,
        }
    }

    fn as_direction(self) -> Direction {
        match self {
            Side::Left => Direction::West,
            Side::Right => Direction::East,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum Object {
    Wall,
    SmallBox,
    LargeBox(Side),
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
                        Some(Object::SmallBox) => "O",
                        Some(Object::LargeBox(Side::Left)) => "[",
                        Some(Object::LargeBox(Side::Right)) => "]",
                        Some(Object::Wall) => "#",
                    }
                }
            );
        }
        println!();
    }

    println!();
}

fn find_moved_objects(
    robot_position: Position,
    objects: &HashMap<Position, Object>,
    direction: Direction,
) -> Option<HashMap<Position, Object>> {
    let mut to_check = vec![robot_position.step(direction)];
    let mut moved_objects = HashMap::new();

    while let Some(pos) = to_check.pop() {
        if moved_objects.contains_key(&pos) {
            continue;
        }

        match objects.get(&pos) {
            None => {}
            Some(Object::Wall) => {
                return None;
            }
            Some(Object::SmallBox) => {
                moved_objects.insert(pos, Object::SmallBox);
                to_check.push(pos.step(direction));
            }
            Some(&Object::LargeBox(side)) => {
                moved_objects.insert(pos, Object::LargeBox(side));
                to_check.push(pos.step(direction));

                let other_side = side.other();
                let other_pos = pos.step(other_side.as_direction());
                moved_objects.insert(other_pos, Object::LargeBox(other_side));
                to_check.push(other_pos.step(direction));
            }
        }
    }

    Some(moved_objects)
}

fn perform_move(
    robot_position: &mut Position,
    objects: &mut HashMap<Position, Object>,
    direction: Direction,
) {
    if let Some(to_move) = find_moved_objects(*robot_position, objects, direction) {
        for (pos, _) in to_move.iter() {
            objects.remove(pos);
        }

        for (pos, object) in to_move {
            objects.insert(pos.step(direction), object);
        }

        *robot_position = robot_position.step(direction);
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
            if matches!(object, Object::SmallBox | Object::LargeBox(Side::Left)) {
                Some(position)
            } else {
                None
            }
        })
        .map(|pos| 100 * pos.y + pos.x)
        .sum()
}

fn expand_map(
    robot_position: Position,
    objects: &HashMap<Position, Object>,
) -> (Position, HashMap<Position, Object>) {
    let new_robot_position = Position {
        x: robot_position.x * 2,
        y: robot_position.y,
    };

    let new_objects = objects
        .iter()
        .flat_map(|(pos, obj)| {
            let new_pos_left = Position {
                x: pos.x * 2,
                y: pos.y,
            };
            let new_pos_right = Position {
                x: pos.x * 2 + 1,
                y: pos.y,
            };

            [new_pos_left, new_pos_right].into_iter().zip(match obj {
                Object::Wall => [Object::Wall, Object::Wall],
                Object::SmallBox => [Object::LargeBox(Side::Left), Object::LargeBox(Side::Right)],
                Object::LargeBox(_) => {
                    panic!("Can't expand large box");
                }
            })
        })
        .collect();

    (new_robot_position, new_objects)
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
                    'O' => Some(((x, y).into(), Object::SmallBox)),
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
        let part1 = find_box_location_sum(robot_position, objects.clone(), &moves);

        let (new_robot_position, expanded_objects) = expand_map(robot_position, &objects);
        let part2 = find_box_location_sum(new_robot_position, expanded_objects, &moves);

        (Some(part1.to_string()), Some(part2.to_string()))
    }
}
