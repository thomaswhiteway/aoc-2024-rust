use crate::common::{Direction, Position};
use failure::Error;
use itertools::iproduct;
use lazy_static::lazy_static;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::str::FromStr;

lazy_static! {
    static ref NUMERIC_POSITIONS: HashMap<char, Position> = {
        let mut positions = HashMap::new();
        positions.insert('7', (0i64, 0).into());
        positions.insert('8', (1i64, 0).into());
        positions.insert('9', (2i64, 0).into());
        positions.insert('4', (0i64, 1).into());
        positions.insert('5', (1i64, 1).into());
        positions.insert('6', (2i64, 1).into());
        positions.insert('1', (0i64, 2).into());
        positions.insert('2', (1i64, 2).into());
        positions.insert('3', (2i64, 2).into());
        positions.insert('0', (1i64, 3).into());
        positions.insert('A', (2i64, 3).into());
        positions
    };
    static ref NUMERIC_BUTTONS: HashMap<Position, char> =
        NUMERIC_POSITIONS.iter().map(|(&c, &p)| (p, c)).collect();
    static ref DIRECTIONAL_POSITIONS: HashMap<char, Position> = {
        let mut positions = HashMap::new();
        positions.insert('^', (1i64, 0).into());
        positions.insert('A', (2i64, 0).into());
        positions.insert('<', (0i64, 1).into());
        positions.insert('v', (1i64, 1).into());
        positions.insert('>', (2i64, 1).into());
        positions
    };
    static ref DIRECTIONAL_BUTTONS: HashMap<Position, char> = {
        DIRECTIONAL_POSITIONS
            .iter()
            .map(|(&c, &p)| (p, c))
            .collect()
    };
}

fn get_output(presses: &str, buttons: &HashMap<Position, char>, start: Position) -> String {
    let mut output = String::new();

    let mut pos = start;

    for press in presses.chars() {
        if press == 'A' {
            output.push(*buttons.get(&pos).unwrap());
        } else {
            let dir = Direction::try_from(press).unwrap();
            pos = pos.step(dir);
        }
    }

    output
}

fn get_code(presses: &str) -> String {
    let dir_start = *DIRECTIONAL_POSITIONS.get(&'A').unwrap();
    let num_start = *NUMERIC_POSITIONS.get(&'A').unwrap();
    let output = get_output(presses, &DIRECTIONAL_BUTTONS, dir_start);
    let output = get_output(&output, &DIRECTIONAL_BUTTONS, dir_start);
    get_output(&output, &NUMERIC_BUTTONS, num_start)
}

fn find_possible_presses(
    presses: &str,
    positions: &HashMap<char, Position>,
    buttons: &HashMap<Position, char>,
    start: Position,
) -> impl Iterator<Item = String> {
    let mut candidates = vec!["".to_string()];

    let mut pos = start;

    for c in presses.chars() {
        let next = *positions.get(&c).unwrap();
        let delta = next - pos;

        let mut components: Vec<String> = vec![];

        match delta.x.cmp(&0) {
            Ordering::Less => components.push("<".repeat(delta.x.unsigned_abs() as usize)),
            Ordering::Greater => components.push(">".repeat(delta.x.unsigned_abs() as usize)),
            _ => {}
        }

        match delta.y.cmp(&0) {
            Ordering::Less => components.push("^".repeat(delta.y.unsigned_abs() as usize)),
            Ordering::Greater => components.push("v".repeat(delta.y.unsigned_abs() as usize)),
            Ordering::Equal => {}
        }

        let new_presses = match components.len() {
            0 => vec!["A".to_string()],
            1 => vec![format!("{}A", components[0])],
            2 => {
                let mut presses = vec![];

                if buttons.contains_key(&(pos + Position { x: delta.x, y: 0 })) {
                    presses.push(format!("{}{}A", components[0], components[1]));
                }

                if buttons.contains_key(&(pos + Position { x: 0, y: delta.y })) {
                    presses.push(format!("{}{}A", components[1], components[0]));
                }

                presses
            }
            _ => unreachable!(),
        };

        candidates = iproduct!(candidates, new_presses)
            .map(|(first, second)| format!("{}{}", first, second))
            .collect();

        pos = next;
    }

    let buttons: HashMap<_, _> = positions.iter().map(|(&c, &p)| (p, c)).collect();
    for candidate in candidates.iter() {
        assert_eq!(get_output(candidate, &buttons, start), presses);
    }

    candidates.into_iter()
}

fn shortest_path(code: &str) -> String {
    find_possible_presses(
        code,
        &NUMERIC_POSITIONS,
        &NUMERIC_BUTTONS,
        *NUMERIC_POSITIONS.get(&'A').unwrap(),
    )
    .flat_map(|sequence| {
        find_possible_presses(
            &sequence,
            &DIRECTIONAL_POSITIONS,
            &DIRECTIONAL_BUTTONS,
            *DIRECTIONAL_POSITIONS.get(&'A').unwrap(),
        )
    })
    .flat_map(|sequence| {
        find_possible_presses(
            &sequence,
            &DIRECTIONAL_POSITIONS,
            &DIRECTIONAL_BUTTONS,
            *DIRECTIONAL_POSITIONS.get(&'A').unwrap(),
        )
    })
    .min_by_key(|sequence| sequence.len())
    .unwrap()
}

fn get_complexity(code: &str) -> usize {
    let sequence = shortest_path(code);
    assert_eq!(get_code(&sequence), code);
    println!("{} {}", code, sequence.len());
    sequence.len() * usize::from_str(&code[..code.len() - 1]).unwrap()
}

fn get_complexity_sum(codes: &[String]) -> usize {
    codes.iter().map(|code| get_complexity(code)).sum()
}

pub struct Solver {}

impl super::Solver for Solver {
    type Problem = Box<[String]>;

    fn parse_input(data: String) -> Result<Self::Problem, Error> {
        Ok(data
            .lines()
            .map(str::to_string)
            .collect::<Vec<_>>()
            .into_boxed_slice())
    }

    fn solve(codes: Self::Problem) -> (Option<String>, Option<String>) {
        let part1 = get_complexity_sum(&codes);
        (Some(part1.to_string()), None)
    }
}
