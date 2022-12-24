use std::collections::{HashMap, HashSet};

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

const PRINT: bool = false;
fn part_1(map: &HashSet<Pos>) {
    let mut map = map.clone();
    draw(&map);

    let mut directions = vec![Dir::N, Dir::S, Dir::W, Dir::E];

    for i in 0..10 {
        let mut dest: HashMap<Pos, bool> = HashMap::new();

        for elf in map.iter() {
            if is_alone(elf, &map) {
                dest.insert(*elf, true);
            } else {
                let mut still = true;
                for option in directions.iter() {
                    if let Some(next) = is_free(*option, &elf, &map) {
                        if let Some(_) = dest.insert(next, true) {
                            dest.insert(next, false);
                        }
                        still = false;
                        break;
                    }
                }
                if still {
                    dest.insert(*elf, true);
                }
            }
        }

        for elf in map.iter() {
            for option in directions.iter() {
                if let Some(next) = is_free(*option, &elf, &map) {
                    if let Some(value) = dest.get(&next) {
                        if *value == false {
                            if PRINT {
                                println!("conflicted {:?}->{:?}", *elf, next);
                            }
                            dest.insert(*elf, true);
                        }
                    }
                    break;
                }
            }
        }

        map = dest
            .iter()
            .filter_map(|(k, v)| {
                if *v {
                    return Some(*k);
                } else {
                    return None;
                }
            })
            .collect();

        if PRINT {
            println!("Round: {}", i + 1);
            draw(&map);
        }
        let top = directions.remove(0);
        directions.push(top);
    }

    draw(&map);
}

fn part_2(map: &HashSet<Pos>) {
    let mut map = map.clone();
    draw(&map);

    let mut directions = vec![Dir::N, Dir::S, Dir::W, Dir::E];

    for i in 0..usize::MAX {
        let mut dest: HashMap<Pos, bool> = HashMap::new();

        for elf in map.iter() {
            if is_alone(elf, &map) {
                dest.insert(*elf, true);
            } else {
                let mut still = true;
                for option in directions.iter() {
                    if let Some(next) = is_free(*option, &elf, &map) {
                        if let Some(_) = dest.insert(next, true) {
                            dest.insert(next, false);
                        }
                        still = false;
                        break;
                    }
                }
                if still {
                    dest.insert(*elf, true);
                }
            }
        }

        for elf in map.iter() {
            for option in directions.iter() {
                if let Some(next) = is_free(*option, &elf, &map) {
                    if let Some(value) = dest.get(&next) {
                        if *value == false {
                            if PRINT {
                                println!("conflicted {:?}->{:?}", *elf, next);
                            }
                            dest.insert(*elf, true);
                        }
                    }
                    break;
                }
            }
        }

        let dest: HashSet<Pos> = dest
            .iter()
            .filter_map(|(k, v)| {
                if *v {
                    return Some(*k);
                } else {
                    return None;
                }
            })
            .collect();

        let mut stopped = true;
        for d in dest.iter() {
            if let None = map.get(d) {
                stopped = false;
                break;
            }
        }
        if stopped {
            println!("Stopped at {}", i + 1);
            break;
        }

        map = dest;

        if PRINT {
            println!("Round: {}", i + 1);
            draw(&map);
        }
        let top = directions.remove(0);
        directions.push(top);
    }
}

#[derive(Clone, Copy)]
enum Dir {
    NW,
    N,
    NE,
    E,
    SE,
    S,
    SW,
    W,
}

const NORTH: [Dir; 3] = [Dir::NW, Dir::N, Dir::NE];
const EAST: [Dir; 3] = [Dir::SE, Dir::E, Dir::NE];
const WEST: [Dir; 3] = [Dir::SW, Dir::W, Dir::NW];
const SOUTH: [Dir; 3] = [Dir::SE, Dir::S, Dir::SW];

fn is_alone(position: &Pos, map: &HashSet<Pos>) -> bool {
    const DIRECTIONS: [(isize, isize); 8] = [
        (1, 0),
        (1, 1),
        (0, 1),
        (-1, 0),
        (-1, 1),
        (-1, -1),
        (1, -1),
        (0, -1),
    ];

    for dir in DIRECTIONS {
        if let Some(_) = map.get(&Pos::new(position.x + dir.0, position.y + dir.1)) {
            return false;
        }
    }

    return true;
}

fn is_free(direction: Dir, position: &Pos, map: &HashSet<Pos>) -> Option<Pos> {
    let directions = match direction {
        Dir::N => NORTH,
        Dir::S => SOUTH,
        Dir::E => EAST,
        Dir::W => WEST,
        _ => panic!("Not allowed direction"),
    };

    for dir in directions {
        let pos = match dir {
            Dir::NW => Pos::new(position.x - 1, position.y - 1),
            Dir::N => Pos::new(position.x, position.y - 1),
            Dir::NE => Pos::new(position.x + 1, position.y - 1),
            Dir::E => Pos::new(position.x + 1, position.y),
            Dir::SE => Pos::new(position.x + 1, position.y + 1),
            Dir::S => Pos::new(position.x, position.y + 1),
            Dir::SW => Pos::new(position.x - 1, position.y + 1),
            Dir::W => Pos::new(position.x - 1, position.y),
        };

        if let Some(_) = map.get(&pos) {
            return None;
        }
    }

    match direction {
        Dir::N => Some(Pos::new(position.x, position.y - 1)),
        Dir::S => Some(Pos::new(position.x, position.y + 1)),
        Dir::E => Some(Pos::new(position.x + 1, position.y)),
        Dir::W => Some(Pos::new(position.x - 1, position.y)),
        _ => panic!("Oh no"),
    }
}

#[derive(Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
struct Pos {
    x: isize,
    y: isize,
}

impl Pos {
    const fn new(x: isize, y: isize) -> Self {
        Pos { x, y }
    }
}

fn parse(file: &str) -> HashSet<Pos> {
    let mut map: HashSet<Pos> = HashSet::new();

    for (y, line) in std::fs::read_to_string(shared::io::expand_file_name(file))
        .unwrap()
        .lines()
        .enumerate()
    {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                map.insert(Pos::new(x as isize, y as isize));
            }
        }
    }

    map
}

fn draw(map: &HashSet<Pos>) {
    let list = map.iter().collect::<Vec<&Pos>>();
    let x_max = list.iter().max_by_key(|p| p.x).unwrap().x;
    let x_min = list.iter().min_by_key(|p| p.x).unwrap().x;
    let y_max = list.iter().max_by_key(|p| p.y).unwrap().y;
    let y_min = list.iter().min_by_key(|p| p.y).unwrap().y;

    let x_range = x_max - x_min + 1;
    let y_range = y_max - y_min + 1;

    println!(
        "{} x {} = {} - {} = {}",
        x_range,
        y_range,
        x_range * y_range,
        list.len(),
        x_range * y_range - list.len() as isize
    );

    if PRINT {
        for y in y_min..=y_max {
            for x in x_min..=x_max {
                if map.contains(&Pos::new(x, y)) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!();
        }

        println!();
    }
}
