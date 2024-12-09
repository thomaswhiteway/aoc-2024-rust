use std::collections::{HashMap, HashSet};

use crate::common::Position;

use failure::Error;

pub struct Grid {
    antennas: HashMap<char, Vec<Position>>,
    width: i64,
    height: i64,
}

impl Grid {
    fn inside(&self, position: Position) -> bool {
        (0..self.width).contains(&position.x) && (0..self.height).contains(&position.y)
    }
}

fn antinodes_for_antennas(left: Position, right: Position) -> impl Iterator<Item = Position> {
    let diff = right - left;
    assert!(diff.x % 3 != 0 || diff.y % 3 != 0);
    [left - diff, right + diff].into_iter()
}

fn find_antinodes(antennas: &[Position]) -> impl Iterator<Item = Position> + '_ {
    (0..antennas.len()).flat_map(move |i| {
        (i + 1..antennas.len()).flat_map(move |j| antinodes_for_antennas(antennas[i], antennas[j]))
    })
}

fn count_antinodes(grid: &Grid) -> usize {
    let antinodes: HashSet<Position> = grid
        .antennas
        .iter()
        .flat_map(|(_, positions)| find_antinodes(&positions))
        .filter(|&pos| grid.inside(pos))
        .collect();

    antinodes.len()
}

pub struct Solver {}

impl super::Solver for Solver {
    type Problem = Grid;

    fn parse_input(data: String) -> Result<Self::Problem, Error> {
        let lines: Vec<_> = data.lines().collect();

        let height = lines.len() as i64;
        let width = lines[0].len() as i64;

        let mut antennas: HashMap<char, Vec<Position>> = HashMap::new();

        for (y, line) in lines.into_iter().enumerate() {
            for (x, c) in line.char_indices() {
                if c != '.' {
                    antennas.entry(c).or_default().push((x, y).into());
                }
            }
        }

        Ok(Grid {
            antennas,
            height,
            width,
        })
    }

    fn solve(grid: Self::Problem) -> (Option<String>, Option<String>) {
        let part1 = count_antinodes(&grid);
        (Some(part1.to_string()), None)
    }
}
