use std::collections::{HashMap, HashSet, VecDeque};

use rayon::prelude::{IntoParallelIterator, ParallelIterator};

fn main() {
    let start = std::time::SystemTime::now();

    let mut data = parse("input.txt");

    println!("Parse: {:?}", start.elapsed());
    let time_1 = std::time::SystemTime::now();

    part_1(&data);

    println!("Part 1: {:?}", time_1.elapsed());

    let time_2 = std::time::SystemTime::now();

    part_2(&mut data);

    println!("Part 2: {:?}", time_2.elapsed());

    println!("Total: {:?}", start.elapsed());
}

fn parse(file: &str) -> HashMap<Id, Valve> {
    compact_graph(
        std::fs::read_to_string(shared::io::expand_file_name(file))
            .unwrap()
            .lines()
            .map(|line| {
                let valve = Valve::new(line);
                (valve.id, valve)
            })
            .collect(),
    )
}

const START: Id = Id {
    value: 'A' as u16 + (('A' as u16) << 8),
};

fn compact_graph(valves: HashMap<Id, Valve>) -> HashMap<Id, Valve> {
    let mut map: HashMap<Id, Valve> = HashMap::new();
    let mut node_list: Vec<Id> = Vec::new();
    let mut node_set: HashSet<Id> = HashSet::new();

    valves
        .iter()
        .filter(|(_, valve)| valve.flow_rate > 0)
        .for_each(|(id, _)| {
            node_list.push(*id);
            node_set.insert(*id);
        });

    node_set.insert(START);
    node_list.push(START);

    for i in 0..node_list.len() {
        add_paths_for_valve(&mut map, node_list[i], &node_set, &valves);
    }

    map
}

fn add_paths_for_valve(
    map: &mut HashMap<Id, Valve>,
    id: Id,
    points_of_interest: &HashSet<Id>,
    valves: &HashMap<Id, Valve>,
) {
    let mut visited: HashSet<Id> = HashSet::new();
    let mut list: VecDeque<Connection> = VecDeque::new();
    let mut connections: Vec<Connection> = Vec::new();

    list.push_back(Connection { id: id, cost: 0 });

    while let Some(current) = list.pop_front() {
        let valve = &valves[&current.id];
        for i in 0..valve.connections.len() {
            let connection = valve.connections[i];

            if !visited.contains(&connection.id) {
                visited.insert(connection.id);
                list.push_back(Connection {
                    id: connection.id,
                    cost: current.cost + connection.cost,
                });

                if connection.id != id && points_of_interest.contains(&connection.id) {
                    connections.push(Connection {
                        id: connection.id,
                        cost: current.cost + connection.cost + 1,
                    });
                }
            }
        }
    }

    map.insert(
        id,
        Valve {
            id: id,
            connections: connections,
            flow_rate: valves[&id].flow_rate,
        },
    );
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Debug)]
struct State {
    score: u16,
    position: Id,
    opened_valves: [Id; 16],
    valve_offset: usize,
}

fn solve(map: &HashMap<Id, Valve>, rounds: u16) -> u16 {
    let start = State {
        score: 0,
        position: START,
        opened_valves: [Id { value: 0 }; 16],
        valve_offset: 0,
    };

    let mut cost_map: HashMap<State, u16> = HashMap::new();
    let mut list: Vec<(State, u16)> = Vec::new();

    list.push((start, 0));

    while let Some((state, cost)) = list.pop() {
        if let Some(valve) = &map.get(&state.position) {
            for i in 0..valve.connections.len() {
                let connection = valve.connections[i];

                if state.opened_valves.contains(&connection.id) {
                    continue;
                }

                let next_cost = cost + connection.cost;

                if let Some(remaining) = rounds.checked_sub(next_cost) {
                    if let Some(next_valve) = &map.get(&connection.id) {
                        let mut opened_valves = state.opened_valves.clone();
                        opened_valves[state.valve_offset] = connection.id;

                        let next_state = State {
                            score: state.score + next_valve.flow_rate * remaining,
                            position: connection.id,
                            opened_valves: opened_valves,
                            valve_offset: state.valve_offset + 1,
                        };

                        if let Some(cached_cost) = cost_map.get(&next_state) {
                            if cached_cost <= &next_cost {
                                continue;
                            }
                        }

                        list.push((next_state, next_cost));
                        cost_map.insert(next_state, next_cost);
                    }
                }
            }
        } else {
            dbg!(state, cost, map.keys());
        }
    }

    let mut vect: Vec<(&State, &u16)> = cost_map.iter().collect();
    vect.sort_unstable();
    if let Some((result, _)) = vect.last() {
        return result.score;
    }

    0
}

fn part_1(map: &HashMap<Id, Valve>) {
    println!("{}", solve(map, 30));
}

fn part_2(map: &mut HashMap<Id, Valve>) {
    let nodes: Vec<&Id> = map.keys().collect();

    let len = nodes.len();

    let permutations: Vec<Vec<&Id>> = ((len * 4 / 9)..=(len / 2))
        .map(|count| itertools::Itertools::combinations(nodes.clone().into_iter(), count))
        .flatten()
        .collect();

    println!("Solving for {} permutations", permutations.len());

    let result = permutations
        .into_par_iter()
        .map(|permutation| {
            let score = solve(
                &map.iter()
                    .filter_map(|(id, valve)| {
                        if permutation.contains(&id) || id.value.eq(&START.value) {
                            return Some((*id, valve.clone()));
                        }
                        return None;
                    })
                    .collect(),
                26,
            );

            score
                + solve(
                    &map.iter()
                        .filter_map(|(id, valve)| {
                            if !permutation.contains(&id) || id.value.eq(&START.value) {
                                return Some((*id, valve.clone()));
                            }
                            return None;
                        })
                        .collect(),
                    26,
                )
        })
        .max()
        .unwrap();

    println!("{}", result);
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
struct Connection {
    id: Id,
    cost: u16,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
struct Id {
    value: u16,
}

impl Id {
    fn new(a: u8, b: u8) -> Self {
        Self {
            value: a as u16 + ((b as u16) << 8),
        }
    }
}

impl core::fmt::Display for Id {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut id_string = String::new();
        id_string.push((self.value as u8) as char);
        id_string.push((self.value >> 8) as u8 as char);
        f.write_str(&id_string)
    }
}

impl core::fmt::Debug for Id {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut id_string = String::new();
        id_string.push((self.value as u8) as char);
        id_string.push((self.value >> 8) as u8 as char);
        f.write_str(&id_string)
    }
}

#[derive(Clone, Debug)]
struct Valve {
    id: Id,
    connections: Vec<Connection>,
    flow_rate: u16,
}

impl Valve {
    fn new(line: &str) -> Self {
        let bytes = line.as_bytes();
        let mut positions: [usize; 8] = [bytes.len(); 8];
        let mut position_offset: usize = 0;
        let mut destinations: usize = 1;

        for (i, byte) in bytes.iter().enumerate() {
            match byte {
                61 => {
                    positions[position_offset] = i + 1;
                    position_offset += 1;
                }
                44 => {
                    positions[position_offset] = i;
                    position_offset += 1;
                    destinations += 1;
                }
                59 => {
                    positions[position_offset] = i;
                    position_offset += 1;
                }
                _ => (),
            }
        }

        let mut connections = Vec::with_capacity(destinations);

        for i in 1..destinations {
            connections.push(Connection {
                id: Id::new(bytes[positions[i + 1] - 2], bytes[positions[i + 1] - 1]),
                cost: 1,
            });
        }

        connections.push(Connection {
            id: Id::new(bytes[bytes.len() - 2], bytes[bytes.len() - 1]),
            cost: 1,
        });

        Valve {
            id: Id::new(bytes[6], bytes[7]),
            connections: connections,
            flow_rate: line[positions[0]..positions[1]].parse().unwrap(),
        }
    }
}
