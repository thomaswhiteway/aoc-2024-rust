use crate::common::{Direction, Position};
use failure::Error;
use itertools::{Either, Itertools};
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::hash::Hash;
use std::str::FromStr;

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
enum Moves {
    North,
    NorthEast,
    EastNorth,
    East,
    EastSouth,
    SouthEast,
    South,
    SouthWest,
    WestSouth,
    West,
    WestNorth,
    NorthWest,
}

impl Moves {
    fn all() -> impl Iterator<Item = Moves> {
        use Moves::*;

        [
            North, NorthEast, EastNorth, East, EastSouth, SouthEast, South, SouthWest, WestSouth,
            West, WestNorth, NorthWest,
        ]
        .into_iter()
    }

    fn components(self) -> impl Iterator<Item = Direction> {
        use Direction::*;
        match self {
            Moves::North => Either::Left([North].into_iter()),
            Moves::NorthEast => Either::Right([North, East].into_iter()),
            Moves::EastNorth => Either::Right([East, North].into_iter()),
            Moves::East => Either::Left([East].into_iter()),
            Moves::EastSouth => Either::Right([East, South].into_iter()),
            Moves::SouthEast => Either::Right([South, East].into_iter()),
            Moves::South => Either::Left([South].into_iter()),
            Moves::SouthWest => Either::Right([South, West].into_iter()),
            Moves::WestSouth => Either::Right([West, South].into_iter()),
            Moves::West => Either::Left([West].into_iter()),
            Moves::WestNorth => Either::Right([West, North].into_iter()),
            Moves::NorthWest => Either::Right([North, West].into_iter()),
        }
    }

    fn for_direction(dir: Direction) -> impl Iterator<Item = Self> {
        use Moves::*;
        match dir {
            Direction::North => Either::Left([North].into_iter()),
            Direction::NorthEast => Either::Right([NorthEast, EastNorth].into_iter()),
            Direction::East => Either::Left([East].into_iter()),
            Direction::SouthEast => Either::Right([SouthEast, EastSouth].into_iter()),
            Direction::South => Either::Left([South].into_iter()),
            Direction::SouthWest => Either::Right([SouthWest, WestSouth].into_iter()),
            Direction::West => Either::Left([West].into_iter()),
            Direction::NorthWest => Either::Right([NorthWest, WestNorth].into_iter()),
        }
    }
}

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

#[allow(unused)]
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

#[allow(unused)]
fn get_code(presses: &str) -> String {
    let dir_start = *DIRECTIONAL_POSITIONS.get(&'A').unwrap();
    let num_start = *NUMERIC_POSITIONS.get(&'A').unwrap();
    let output = get_output(presses, &DIRECTIONAL_BUTTONS, dir_start);
    let output = get_output(&output, &DIRECTIONAL_BUTTONS, dir_start);
    get_output(&output, &NUMERIC_BUTTONS, num_start)
}

fn shortest_route(
    from: Position,
    to: Position,
    cost: &HashMap<Moves, usize>,
    buttons: &HashMap<Position, char>,
) -> usize {
    Moves::for_direction(from.direction_to(to))
        .filter(|moves| {
            if let &[first, _] = moves.components().collect::<Vec<_>>().as_slice() {
                let corner = match first {
                    Direction::North | Direction::South => Position { x: from.x, y: to.y },
                    Direction::East | Direction::West => Position { x: to.x, y: from.y },
                    _ => unreachable!(),
                };

                buttons.contains_key(&corner)
            } else {
                true
            }
        })
        .map(|moves| cost.get(&moves).unwrap() + from.manhattan_distance_to(&to) as usize)
        .min()
        .unwrap()
}

fn min_cost_for_route(
    route: impl Iterator<Item = Position>,
    cost: &HashMap<Moves, usize>,
    buttons: &HashMap<Position, char>,
) -> usize {
    route
        .tuple_windows()
        .map(|(from, to)| shortest_route(from, to, cost, buttons))
        .sum()
}

fn get_route<'a>(
    presses: impl Iterator<Item = char> + 'a,
    positions: &'a HashMap<char, Position>,
) -> impl Iterator<Item = Position> + 'a {
    [positions[&'A']]
        .into_iter()
        .chain(presses.map(|c| positions[&c]))
}

fn min_cost_for_moves(
    moves: Moves,
    cost: &HashMap<Moves, usize>,
    positions: &HashMap<char, Position>,
    buttons: &HashMap<Position, char>,
) -> usize {
    let route = get_route(
        moves.components().map(|d| d.as_char()).chain(['A']),
        positions,
    );
    min_cost_for_route(route, cost, buttons)
}

fn min_cost_for_code(
    code: &str,
    cost: &HashMap<Moves, usize>,
    positions: &HashMap<char, Position>,
    buttons: &HashMap<Position, char>,
) -> usize {
    let route = get_route(code.chars(), positions);
    min_cost_for_route(route, cost, buttons) + code.len()
}

fn update_costs(
    costs: &HashMap<Moves, usize>,
    positions: &HashMap<char, Position>,
    buttons: &HashMap<Position, char>,
) -> HashMap<Moves, usize> {
    Moves::all()
        .map(|moves| (moves, min_cost_for_moves(moves, costs, positions, buttons)))
        .collect()
}

fn shortest_path_len(code: &str, intermediate_keypads: usize) -> usize {
    let mut costs: HashMap<Moves, usize> = Moves::all().map(|m| (m, 0)).collect();

    for _ in 0..intermediate_keypads {
        costs = update_costs(&costs, &DIRECTIONAL_POSITIONS, &DIRECTIONAL_BUTTONS);
    }

    min_cost_for_code(code, &costs, &NUMERIC_POSITIONS, &NUMERIC_BUTTONS)
}

fn get_complexity(code: &str, intermediate_keypads: usize) -> usize {
    let sequence_len = shortest_path_len(code, intermediate_keypads);
    sequence_len * usize::from_str(&code[..code.len() - 1]).unwrap()
}

fn get_complexity_sum(codes: &[String], intermediate_keypads: usize) -> usize {
    codes
        .iter()
        .map(|code| get_complexity(code, intermediate_keypads))
        .sum()
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
        let part1 = get_complexity_sum(&codes, 2);
        let part2 = get_complexity_sum(&codes, 25);
        (Some(part1.to_string()), Some(part2.to_string()))
    }
}
