use std::collections::{HashMap, VecDeque};

const PRINT: bool = false;

fn main() {
    let start = std::time::SystemTime::now();

    let data = parse("input.txt");

    println!("Parse: {:?}", start.elapsed());

    let time_1 = std::time::SystemTime::now();
    part_1(&data, 24);

    println!("Part 1: {:?}", time_1.elapsed());

    let time_2 = std::time::SystemTime::now();
    part_2(&data, 32);

    println!("Part 2: {:?}", time_2.elapsed());

    println!("Total: {:?}", start.elapsed());
    /*
    let state = State {
        geodes: 0,
        bots: Resources::new(1, 0, 0),
        resources: Resources::new(0, 0, 0),
    };

    dbg!(state.buy_robot(&Resources::new(2, 0, 0))); */
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
                ore: Resources::new(segments[0][6].parse().unwrap(), 0, 0),
                clay: Resources::new(segments[1][4].parse().unwrap(), 0, 0),
                obsidian: Resources::new(
                    segments[2][4].parse().unwrap(),
                    segments[2][7].parse().unwrap(),
                    0,
                ),
                geode: Resources::new(
                    segments[3][4].parse().unwrap(),
                    0,
                    segments[3][7].parse().unwrap(),
                ),
            }
        })
        .collect()
}

fn part_1(data: &Vec<Blueprint>, minutes: u8) {
    let mut result = 0;
    for i in 0..data.len() {
        let start = std::time::SystemTime::now();
        let potential = get_potential(&data[i], minutes);
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

fn part_2(data: &Vec<Blueprint>, minutes: u8) {
    let mut result = 1;
    for i in 0..3 {
        let start = std::time::SystemTime::now();
        let potential = get_potential(&data[i], minutes);
        result *= potential;
        println!(
            "Blueprint: {:<3} - [ {:<2} : {:<4} ] {:?}",
            i + 1,
            potential,
            result,
            start.elapsed()
        );
    }
    println!("{}", result);
}

fn get_potential(blueprint: &Blueprint, minutes: u8) -> u16 {
    const START: State = State {
        resources: Resources::new(0, 0, 0),
        bots: Resources::new(1, 0, 0),
        geodes: 0,
    };

    let mut map: HashMap<State, u8> = HashMap::new();
    let mut queue: VecDeque<(State, u8)> = VecDeque::new();

    insert_if_cheaper(&mut map, &START, minutes);
    queue.push_back((START, minutes));

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

        if state.geodes + (0..=remaining as u16).sum::<u16>() + 1 <= max_geodes {
            continue;
        }

        if state.geodes > max_geodes {
            max_geodes = state.geodes;
            if PRINT {
                println!(
                    "New best state at {} -> Potential Geodes: {} {:?}",
                    remaining, max_geodes, state
                );
            }
        }

        if remaining > 2 {
            if state.bots.ore < total_costs.ore {
                if let Some((mut next, rounds)) = state.buy_robot(&blueprint.ore, remaining) {
                    next.bots.ore += 1;

                    if insert_if_cheaper(&mut map, &next, remaining - rounds) {
                        queue.push_back((next, remaining - rounds));
                    }
                }
            }

            if state.bots.clay < total_costs.clay {
                if let Some((mut next, rounds)) = state.buy_robot(&blueprint.clay, remaining) {
                    next.bots.clay += 1;

                    if insert_if_cheaper(&mut map, &next, remaining - rounds) {
                        queue.push_back((next, remaining - rounds));
                    }
                }
            }

            if state.bots.obsidian < total_costs.obsidian {
                if let Some((mut next, rounds)) = state.buy_robot(&blueprint.obsidian, remaining) {
                    next.bots.obsidian += 1;

                    if insert_if_cheaper(&mut map, &next, remaining - rounds) {
                        queue.push_back((next, remaining - rounds));
                    }
                }
            }
        }

        {
            if let Some((mut next, rounds)) = state.buy_robot(&blueprint.geode, remaining) {
                next.geodes += remaining as u16 - rounds as u16;

                if insert_if_cheaper(&mut map, &next, remaining - rounds) {
                    queue.push_back((next, remaining - rounds));
                }
            }
        }
    }

    max_geodes
}

fn insert_if_cheaper(map: &mut HashMap<State, u8>, state: &State, remaining: u8) -> bool {
    //let key = state.potential(remaining);
    let key = *state;
    if let Some(cache) = map.get(&key) {
        if *cache >= remaining {
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
    geodes: u16,
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
}

impl State {
    fn rounds_to_buy(&self, cost: &Resources) -> Option<u8> {
        if let Some(_) = self.resources.checked_sub(&cost) {
            return Some(0);
        }

        let missing = cost.saturating_sub(&self.resources);
        return missing.div_roof(&self.bots);
    }

    fn buy_robot(&self, cost: &Resources, remaining: u8) -> Option<(State, u8)> {
        if let Some(rounds) = self.rounds_to_buy(&cost) {
            //dbg!(rounds, self.bots, cost, self.resources);
            if rounds < remaining {
                let next = State {
                    resources: self
                        .resources
                        .add(&self.bots.mult(rounds))
                        .checked_sub(&cost)
                        .expect("Should be able to buy bot")
                        .add(&self.bots),
                    bots: self.bots,
                    geodes: self.geodes,
                };

                return Some((next, rounds + 1));
            }
        }
        return None;
    }
}

impl Resources {
    const fn new(ore: u8, clay: u8, obsidian: u8) -> Self {
        Self {
            ore,
            clay,
            obsidian,
        }
    }

    fn maximize(&self, other: &Self) -> Self {
        Self::new(
            self.ore.max(other.ore),
            self.clay.max(other.clay),
            self.obsidian.max(other.obsidian),
        )
    }

    fn add(&self, other: &Self) -> Self {
        Self::new(
            self.ore.saturating_add(other.ore),
            self.clay.saturating_add(other.clay),
            self.obsidian.saturating_add(other.obsidian),
        )
    }

    fn checked_sub(&self, other: &Self) -> Option<Self> {
        if self.ore < other.ore || self.clay < other.clay || self.obsidian < other.obsidian {
            return None;
        }

        Some(Self::new(
            self.ore - other.ore,
            self.clay - other.clay,
            self.obsidian - other.obsidian,
        ))
    }

    fn saturating_sub(&self, other: &Self) -> Self {
        Self::new(
            self.ore.saturating_sub(other.ore),
            self.clay.saturating_sub(other.clay),
            self.obsidian.saturating_sub(other.obsidian),
        )
    }

    fn mult(&self, other: u8) -> Self {
        Self::new(
            self.ore.checked_mul(other).unwrap(),
            self.clay.checked_mul(other).unwrap(),
            self.obsidian.checked_mul(other).unwrap(),
        )
    }

    fn div_roof(&self, other: &Self) -> Option<u8> {
        let mut result: Option<u8> = None;

        if other.ore > 0 {
            let val = (self.ore.checked_add(other.ore).unwrap() - 1)
                .checked_div(other.ore)
                .unwrap();

            if let Some(r) = result {
                result = Some(r.max(val));
            } else {
                result = Some(val);
            }
        } else if self.ore > 0 {
            return None;
        }

        if other.clay > 0 {
            let val = (self.clay.checked_add(other.clay).unwrap() - 1)
                .checked_div(other.clay)
                .unwrap();

            if let Some(r) = result {
                result = Some(r.max(val));
            } else {
                result = Some(val);
            }
        } else if self.clay > 0 {
            return None;
        }

        if other.obsidian > 0 {
            let val = (self.obsidian.checked_add(other.obsidian).unwrap() - 1)
                .checked_div(other.obsidian)
                .unwrap();

            if let Some(r) = result {
                result = Some(r.max(val));
            } else {
                result = Some(val);
            }
        } else if self.obsidian > 0 {
            return None;
        }

        result
    }
}
