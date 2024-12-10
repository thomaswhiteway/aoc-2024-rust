use std::collections::{HashMap, HashSet};

use crate::common::Position;

use failure::Error;
use num::integer::gcd;

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

trait AntinodeFinder {
    fn antinodes_for_antennas(
        grid: &Grid,
        left: Position,
        right: Position,
    ) -> impl Iterator<Item = Position> + '_;
}

struct BasicFinder {}

impl AntinodeFinder for BasicFinder {
    fn antinodes_for_antennas(
        grid: &Grid,
        left: Position,
        right: Position,
    ) -> impl Iterator<Item = Position> + '_ {
        let diff = right - left;
        assert!(diff.x % 3 != 0 || diff.y % 3 != 0);
        [left - diff, right + diff]
            .into_iter()
            .filter(|&pos| grid.inside(pos))
    }
}

struct FullFinder {}

impl AntinodeFinder for FullFinder {
    fn antinodes_for_antennas(
        grid: &Grid,
        left: Position,
        right: Position,
    ) -> impl Iterator<Item = Position> + '_ {
        let step = right - left;
        assert!(gcd(step.x, step.y) == 1);
        (0..)
            .map(move |offset: i64| left - step * offset)
            .take_while(|&pos| grid.inside(pos))
            .chain(
                (1..)
                    .map(move |offset: i64| left + step * offset)
                    .take_while(|&pos| grid.inside(pos)),
            )
    }
}

fn find_antinodes<'a, F: AntinodeFinder>(
    grid: &'a Grid,
    antennas: &'a [Position],
) -> impl Iterator<Item = Position> + 'a {
    (0..antennas.len()).flat_map(move |i| {
        (i + 1..antennas.len())
            .flat_map(move |j| F::antinodes_for_antennas(grid, antennas[i], antennas[j]))
    })
}

fn count_antinodes<F: AntinodeFinder>(grid: &Grid) -> usize {
    let antinodes: HashSet<Position> = grid
        .antennas
        .iter()
        .flat_map(|(_, positions)| find_antinodes::<F>(grid, positions))
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
        let part1 = count_antinodes::<BasicFinder>(&grid);
        let part2 = count_antinodes::<FullFinder>(&grid);
        (Some(part1.to_string()), Some(part2.to_string()))
    }
}
