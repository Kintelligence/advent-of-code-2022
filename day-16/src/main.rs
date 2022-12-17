use std::collections::{HashMap, HashSet, VecDeque};

fn main() {
    let start = std::time::SystemTime::now();

    let data = parse("input.txt");

    println!("Parse: {:?}", start.elapsed());
    let time_1 = std::time::SystemTime::now();

    part_1(&data);

    println!("Part 1: {:?}", time_1.elapsed());

    let time_2 = std::time::SystemTime::now();

    part_2(&data);

    println!("Part 2: {:?}", time_2.elapsed());

    println!("Total: {:?}", start.elapsed());
}

fn parse(file: &str) -> HashMap<Id, Valve> {
    std::fs::read_to_string(shared::io::expand_file_name(file))
        .unwrap()
        .lines()
        .map(|line| {
            let valve = Valve::new(line);
            (valve.id, valve)
        })
        .collect()
}

const START: Id = Id {
    value: 'A' as u16 + (('A' as u16) << 8),
};

fn compact_graph(valves: &HashMap<Id, Valve>) -> (HashMap<Id, Valve>, Vec<Id>) {
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

    (map, node_list)
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

const ROUNDS: u16 = 30;

fn part_1(input: &HashMap<Id, Valve>) {
    let (map, a) = compact_graph(input);
    dbg!(a);

    let mut cost_map: HashMap<State, u16> = HashMap::new();
    let mut list: VecDeque<(State, u16)> = VecDeque::new();
    list.push_back((
        State {
            score: 0,
            position: START,
            opened_valves: [Id { value: 0 }; 10],
            valve_offset: 0,
        },
        0,
    ));

    while let Some((state, cost)) = list.pop_front() {
        //dbg!(state, cost);

        let valve = &map[&state.position];
        for i in 0..valve.connections.len() {
            let connection = valve.connections[i];

            if state.opened_valves.contains(&connection.id) {
                continue;
            }

            let next_cost = cost + connection.cost;

            if let Some(remaining) = ROUNDS.checked_sub(next_cost) {
                let next_valve = &map[&connection.id];
                let mut opened_valves = state.opened_valves.clone();
                opened_valves[state.valve_offset] = connection.id;
                //opened_valves.sort_unstable();

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

                list.push_back((next_state, next_cost));
                cost_map.insert(next_state, next_cost);
            }
        }
    }

    let mut vect: Vec<(&State, &u16)> = cost_map.iter().collect();
    vect.sort_unstable();
    dbg!(vect.last().unwrap());
}

const COOP_ROUND: u16 = 26;

fn part_2(input: &HashMap<Id, Valve>) {
    let (map, nodes) = compact_graph(input);

    let mut cost_map: HashMap<EState, (u16, u16)> = HashMap::new();
    let mut list: VecDeque<(EState, u16, u16)> = VecDeque::new();
    list.push_back((
        EState {
            score: 0,
            position: START,
            e_position: START,
            opened_valves: [Id { value: 0 }; 20],
            valve_offset: 0,
        },
        0,
        0,
    ));

    let mut count: u32 = 0;

    while let Some((state, cost, e_cost)) = list.pop_front() {
        count += 1;
        if count % 10000 == 0 {
            dbg!(count);
            dbg!(state);
        }

        //dbg!(state, cost);
        let valve = &map[&state.position];
        let e_valve = &map[&state.e_position];

        let options: Vec<(Id, u16, u16)> = valve
            .connections
            .iter()
            .filter_map(|connection| {
                if state.opened_valves.contains(&connection.id) {
                    return None;
                }

                let next_cost = cost + connection.cost;
                if let Some(remaining) = COOP_ROUND.checked_sub(next_cost) {
                    return Some((connection.id, next_cost, remaining));
                }

                return None;
            })
            .collect();

        let e_options: Vec<(Id, u16, u16)> = e_valve
            .connections
            .iter()
            .filter_map(|connection| {
                if state.opened_valves.contains(&connection.id) {
                    return None;
                }

                let next_cost = e_cost + connection.cost;
                if let Some(remaining) = COOP_ROUND.checked_sub(next_cost) {
                    return Some((connection.id, next_cost, remaining));
                }

                return None;
            })
            .collect();

        if options.len() == 0 {
            for e in e_options {
                let next_valve = &map[&e.0];

                let mut opened_valves = state.opened_valves.clone();
                opened_valves[state.valve_offset] = e.0;

                let next_state = EState {
                    score: state.score + next_valve.flow_rate * e.2,
                    position: state.position,
                    e_position: e.0,
                    opened_valves: opened_valves,
                    valve_offset: state.valve_offset + 1,
                };

                if let Some((co, ce)) = cost_map.get(&next_state) {
                    if co + ce <= e.1 + cost {
                        continue;
                    }
                }

                list.push_back((next_state, cost, e.1));
                cost_map.insert(next_state, (cost, e.1));
            }
        } else if e_options.len() == 0 {
            for o in options {
                let next_valve = &map[&o.0];

                let mut opened_valves = state.opened_valves.clone();
                opened_valves[state.valve_offset] = o.0;

                let next_state = EState {
                    score: state.score + next_valve.flow_rate * o.2,
                    position: o.0,
                    e_position: state.e_position,
                    opened_valves: opened_valves,
                    valve_offset: state.valve_offset + 1,
                };

                if let Some((co, ce)) = cost_map.get(&next_state) {
                    if co + ce <= e_cost + o.1 {
                        continue;
                    }
                }

                list.push_back((next_state, o.1, e_cost));
                cost_map.insert(next_state, (o.1, e_cost));
            }
        } else {
            for o in options {
                for i in 0..e_options.len() {
                    let e = e_options[i];

                    if o.0 == e.0 {
                        continue;
                    }

                    let o_valve = &map[&o.0];
                    let e_valve = &map[&e.0];

                    let mut opened_valves = state.opened_valves.clone();
                    opened_valves[state.valve_offset] = o.0;
                    opened_valves[state.valve_offset + 1] = e.0;

                    let next_state = EState {
                        score: state.score + (o_valve.flow_rate * o.2) + (e_valve.flow_rate * e.2),
                        position: o.0,
                        e_position: e.0,
                        opened_valves: opened_valves,
                        valve_offset: state.valve_offset + 2,
                    };

                    if let Some((co, ce)) = cost_map.get(&next_state) {
                        if co + ce <= e.1 + o.1 {
                            continue;
                        }
                    }

                    list.push_back((next_state, o.1, e.1));
                    cost_map.insert(next_state, (o.1, e.1));
                }
            }
        }
    }

    let mut vect: Vec<(&EState, &(u16, u16))> = cost_map.iter().collect();
    vect.sort_unstable();
    dbg!(vect.last().unwrap());
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Debug)]
struct State {
    score: u16,
    position: Id,
    opened_valves: [Id; 10],
    valve_offset: usize,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Debug)]
struct EState {
    score: u16,
    position: Id,
    e_position: Id,
    opened_valves: [Id; 20],
    valve_offset: usize,
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
