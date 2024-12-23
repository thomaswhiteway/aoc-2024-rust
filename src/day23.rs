use std::collections::{HashMap, HashSet};

use failure::Error;

#[derive(Debug, Clone, Default)]
struct ConnectionMap<'a> {
    connections: HashMap<&'a str, HashSet<&'a str>>,
}

impl<'a> ConnectionMap<'a> {
    fn add_connection(&mut self, a: &'a str, b: &'a str) {
        self.connections.entry(a).or_default().insert(b);
        self.connections.entry(b).or_default().insert(a);
    }

    fn computers(&self) -> impl Iterator<Item = &'a str> + '_ {
        self.connections.keys().copied()
    }

    fn connections_from(&self, name: &str) -> Option<&HashSet<&'a str>> {
        self.connections.get(name)
    }

    fn has_connection(&self, a: &str, b: &str) -> bool {
        self.connections_from(a)
            .map_or(false, |conns| conns.contains(b))
    }
}

fn build_connection_map(connections: &[(String, String)]) -> ConnectionMap<'_> {
    let mut connection_map = ConnectionMap::default();
    for (a, b) in connections.iter() {
        connection_map.add_connection(a, b);
    }
    connection_map
}

fn connected_pairs<'a>(
    connection_map: &'a ConnectionMap<'a>,
    computers: impl Iterator<Item = &'a str>,
) -> impl Iterator<Item = (&'a str, &'a str)> + 'a {
    let computers: Vec<_> = computers.collect();
    let num_computers = computers.len();

    (0..num_computers)
        .flat_map(move |i| (i + 1..num_computers).map(move |j| (i, j)))
        .map(move |(i, j)| (computers[i], computers[j]))
        .filter(|(a, b)| connection_map.has_connection(a, b))
}

fn find_triples(connections: &[(String, String)]) -> usize {
    let connection_map = build_connection_map(connections);

    let mut t_computers: Vec<_> = connection_map
        .computers()
        .filter(|name| name.starts_with("t"))
        .collect();
    t_computers.sort();

    t_computers
        .into_iter()
        .flat_map(|computer| {
            let candidates = connection_map
                .connections_from(computer)
                .unwrap()
                .iter()
                .copied()
                .filter(|&other| !other.starts_with("t") || other > computer);
            connected_pairs(&connection_map, candidates)
        })
        .count()
}

pub struct Solver {}

impl super::Solver for Solver {
    type Problem = Box<[(String, String)]>;

    fn parse_input(data: String) -> Result<Self::Problem, Error> {
        Ok(data
            .lines()
            .map(|line| {
                line.split_once("-")
                    .unwrap_or_else(|| panic!("Invalid connection {}", line))
            })
            .map(|(a, b)| (a.to_string(), b.to_string()))
            .collect::<Vec<_>>()
            .into_boxed_slice())
    }

    fn solve(connections: Self::Problem) -> (Option<String>, Option<String>) {
        let part1 = find_triples(&connections);
        (Some(part1.to_string()), None)
    }
}
