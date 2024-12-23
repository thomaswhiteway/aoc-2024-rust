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

impl<'a> FromIterator<&'a (String, String)> for ConnectionMap<'a> {
    fn from_iter<T: IntoIterator<Item = &'a (String, String)>>(connections: T) -> Self {
        let mut connection_map = ConnectionMap::default();
        for (a, b) in connections {
            connection_map.add_connection(a, b);
        }
        connection_map
    }
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

fn find_triples(connection_map: &ConnectionMap<'_>) -> usize {
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
            connected_pairs(connection_map, candidates)
        })
        .count()
}

fn find_connected_sets<'a>(connection_map: &ConnectionMap<'a>) -> Vec<HashSet<&'a str>> {
    let mut connected_sets: Vec<HashSet<&'a str>> = vec![];

    let mut computers: Vec<_> = connection_map.computers().collect();
    computers.sort();

    for computer in connection_map.computers() {
        let connected = connection_map.connections_from(computer).unwrap();

        let mut unused = connected.clone();

        for existing_set in connected_sets.iter_mut() {
            if existing_set.is_subset(connected) {
                unused.retain(|&u| !existing_set.contains(u));
                existing_set.insert(computer);
            }
        }

        connected_sets.extend(
            unused
                .into_iter()
                .filter(|&u| u < computer)
                .map(|other| [computer, other].into_iter().collect()),
        );
    }

    connected_sets
}

fn find_largest_connected_set<'a>(connection_map: &ConnectionMap<'a>) -> HashSet<&'a str> {
    find_connected_sets(connection_map)
        .into_iter()
        .max_by_key(|set| set.len())
        .unwrap()
}

fn find_password(connections: &ConnectionMap<'_>) -> String {
    let mut computers: Vec<_> = find_largest_connected_set(connections)
        .into_iter()
        .collect();
    computers.sort();
    computers.join(",")
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
        let connection_map = connections.iter().collect();
        let part1 = find_triples(&connection_map);
        let part2 = find_password(&connection_map);
        (Some(part1.to_string()), Some(part2))
    }
}
