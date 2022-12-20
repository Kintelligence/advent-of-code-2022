use std::collections::{HashMap, VecDeque};

fn main() {
    let start = std::time::SystemTime::now();

    let data = parse("miq.txt");

    println!("Parse: {:?}", start.elapsed());

    let time_1 = std::time::SystemTime::now();
    part_1(&data);

    println!("Part 1: {:?}", time_1.elapsed());
    /*
    let time_2 = std::time::SystemTime::now();
    part_2(&data);

    println!("Part 2: {:?}", time_2.elapsed());

    println!("Total: {:?}", start.elapsed());*/
}

#[derive(Hash, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
struct Blueprint {
    ore: Resources,
    clay: Resources,
    obsidian: Resources,
    geode: Resources,
}

#[derive(Hash, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
struct Resources {
    ore: u8,
    clay: u8,
    obsidian: u8,
    geode: u8,
}

impl Resources {
    const fn new(ore: u8, clay: u8, obsidian: u8, geode: u8) -> Self {
        Self {
            ore,
            clay,
            obsidian,
            geode,
        }
    }

    fn maximize(&self, other: &Self) -> Self {
        Self::new(
            self.ore.max(other.ore),
            self.clay.max(other.clay),
            self.obsidian.max(other.obsidian),
            self.geode.max(other.geode),
        )
    }

    fn add(&self, other: &Self) -> Self {
        Self::new(
            self.ore.checked_add(other.ore).unwrap(),
            self.clay.checked_add(other.clay).unwrap(),
            self.obsidian.checked_add(other.obsidian).unwrap(),
            self.geode.checked_add(other.geode).unwrap(),
        )
    }

    fn checked_sub(&self, other: &Self) -> Option<Self> {
        if self.ore < other.ore
            || self.clay < other.clay
            || self.obsidian < other.obsidian
            || self.geode < other.geode
        {
            return None;
        }

        Some(Self::new(
            self.ore - other.ore,
            self.clay - other.clay,
            self.obsidian - other.obsidian,
            self.geode - other.geode,
        ))
    }

    fn mult(&self, other: u8) -> Self {
        Self::new(
            self.ore.checked_mul(other).unwrap(),
            self.clay.checked_mul(other).unwrap(),
            self.obsidian.checked_mul(other).unwrap(),
            self.geode.checked_mul(other).unwrap(),
        )
    }
}

fn parse(file: &str) -> Vec<Blueprint> {
    std::fs::read_to_string(shared::io::expand_file_name(file))
        .unwrap()
        .lines()
        .map(|l| {
            let segments: Vec<Vec<&str>> = l
                .split('.')
                .map(|s| s.trim().split(' ').collect())
                .collect();

            Blueprint {
                ore: Resources::new(segments[0][6].parse().unwrap(), 0, 0, 0),
                clay: Resources::new(segments[1][4].parse().unwrap(), 0, 0, 0),
                obsidian: Resources::new(
                    segments[2][4].parse().unwrap(),
                    segments[2][7].parse().unwrap(),
                    0,
                    0,
                ),
                geode: Resources::new(
                    segments[3][4].parse().unwrap(),
                    0,
                    segments[3][7].parse().unwrap(),
                    0,
                ),
            }
        })
        .collect()
}

const PRINT: bool = false;

fn part_1(data: &Vec<Blueprint>) {
    let mut result = 0;
    for i in 0..data.len() {
        let start = std::time::SystemTime::now();
        let potential = get_potential(&data[i]);
        result += (i + 1) * potential as usize;
        println!(
            "Blueprint: {:<3} - [ {:<2} : {:<3} : {:<4} ] {:?}",
            i + 1,
            potential,
            (i + 1) * potential as usize,
            result,
            start.elapsed()
        );
    }
    println!("{}", result);
}

fn get_potential(blueprint: &Blueprint) -> u8 {
    const MINUTES: u8 = 24;

    const START: State = State {
        resources: Resources::new(0, 0, 0, 0),
        bots: Resources::new(1, 0, 0, 0),
    };

    let mut map: HashMap<State, u8> = HashMap::new();
    let mut queue: VecDeque<(State, u8)> = VecDeque::new();

    insert_if_cheaper(&mut map, &START, MINUTES);
    queue.push_back((START, MINUTES));

    let mut max_geodes = 0;

    let total_costs = blueprint
        .clay
        .maximize(&blueprint.ore)
        .maximize(&blueprint.geode)
        .maximize(&blueprint.obsidian);

    while let Some((state, remaining)) = queue.pop_back() {
        if remaining == 0 {
            continue;
        }

        let potential_geodes = state.potential(remaining).geode;
        if potential_geodes > max_geodes {
            max_geodes = potential_geodes;
            println!(
                "New best state at {} -> Potential Geodes: {} {:?}",
                remaining, max_geodes, state
            );
        }

        {
            let wait = State {
                resources: state.resources.add(&state.bots),
                bots: state.bots,
            };
            if insert_if_cheaper(&mut map, &wait, remaining - 1) {
                queue.push_back((wait, remaining - 1));
            }
        }

        if state.bots.ore < total_costs.ore {
            if let Some(resources) = state.resources.checked_sub(&blueprint.ore) {
                let mut next = State {
                    resources: resources.add(&state.bots),
                    bots: state.bots,
                };

                next.bots.ore += 1;

                if insert_if_cheaper(&mut map, &next, remaining - 1) {
                    queue.push_back((next, remaining - 1));
                }
            }
        }

        if state.bots.clay < total_costs.clay {
            if let Some(resources) = state.resources.checked_sub(&blueprint.clay) {
                let mut next = State {
                    resources: resources.add(&state.bots),
                    bots: state.bots,
                };

                next.bots.clay += 1;

                if insert_if_cheaper(&mut map, &next, remaining - 1) {
                    queue.push_back((next, remaining - 1));
                }
            }
        }

        if state.bots.obsidian < total_costs.obsidian {
            if let Some(resources) = state.resources.checked_sub(&blueprint.obsidian) {
                let mut next = State {
                    resources: resources.add(&state.bots),
                    bots: state.bots,
                };

                next.bots.obsidian += 1;

                if insert_if_cheaper(&mut map, &next, remaining - 1) {
                    queue.push_back((next, remaining - 1));
                }
            }
        }

        if let Some(resources) = state.resources.checked_sub(&blueprint.geode) {
            let mut next = State {
                resources: resources.add(&state.bots),
                bots: state.bots,
            };

            next.bots.geode += 1;

            if insert_if_cheaper(&mut map, &next, remaining - 1) {
                queue.push_back((next, remaining - 1));
            }
        }
    }

    max_geodes
}

fn insert_if_cheaper(map: &mut HashMap<State, u8>, state: &State, remaining: u8) -> bool {
    //let key = state.potential(remaining);
    let key = *state;
    if let Some(cache) = map.get(&key) {
        if *cache <= remaining {
            return false;
        }
    }

    map.insert(key, remaining);
    return true;
}

#[derive(Hash, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
struct State {
    resources: Resources,
    bots: Resources,
}

impl State {
    fn potential(&self, remaining: u8) -> Resources {
        self.resources.add(&self.bots.mult(remaining))
    }
}
