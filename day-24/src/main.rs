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

const PRINT: bool = false;
const DIRECTIONS: [(i16, i16); 5] = [(0, 1), (0, 0), (1, 0), (-1, 0), (0, -1)];

fn part_1(map: &Map) {
    let start = Position::new(map.start, 0);
    let goal = Position::new(map.goal, map.height - 1);

    let state = State {
        pos: start,
        cycle: 0,
    };

    let (_, turn) = solve(&state, map, goal).unwrap();
    println!("Cost: {}", turn);
}

fn part_2(map: &Map) {
    let start = Position::new(map.start, 0);
    let goal = Position::new(map.goal, map.height - 1);

    let state = State {
        pos: start,
        cycle: 0,
    };

    let mut total = 0;

    let (state, turn) = solve(&state, map, goal).unwrap();
    total += turn;
    println!("1: {}", turn);
    let (state, turn) = solve(&state, map, start).unwrap();
    total += turn;
    println!("2: {}", turn);
    let (_, turn) = solve(&state, map, goal).unwrap();
    total += turn;
    println!("3: {}", turn);
    println!("Total: {}", total);
}

fn solve(start: &State, map: &Map, goal: Position) -> Option<(State, u16)> {
    let mut cache: HashMap<State, u16> = HashMap::new();
    let mut queue: VecDeque<(State, u16)> = VecDeque::new();

    queue.push_back((*start, 0));
    cache.insert(*start, 0);

    while let Some((state, turn)) = queue.pop_front() {
        if PRINT {
            draw(map, &state);
        }
        let bliz = &map.bliz[((state.cycle + 1) % map.cycle_length) as usize];
        let mut options: Vec<State> = Vec::new();
        for (x_d, y_d) in DIRECTIONS {
            if let Some(next) = state.pos.checked_push(x_d, y_d, map) {
                let next_state = State {
                    pos: next,
                    cycle: (state.cycle + 1) % map.cycle_length,
                };

                if let Some(_) = bliz.get(&next) {
                    continue;
                }

                if next == goal {
                    return Some((next_state, turn + 1));
                }

                if let Some(cached) = cache.get(&next_state) {
                    if *cached > turn + 1 {
                        options.push(next_state);
                    }
                } else {
                    options.push(next_state);
                }
            }
        }

        for o in options {
            cache.insert(o, turn + 1);
            queue.push_back((o, turn + 1));
        }
    }

    None
}

fn draw(map: &Map, state: &State) {
    for _ in 1..map.start {
        print!("#");
    }
    if state.pos.x == map.start && state.pos.y == 0 {
        print!("E");
    } else {
        print!(".");
    }
    for _ in (map.start + 2)..map.width - 1 {
        print!("#");
    }
    println!();

    for y in 1..map.height - 1 {
        for x in 1..map.width - 1 {
            if state.pos.x == x && state.pos.y == y {
                print!("E");
            } else if let Some(_) = map.bliz[state.cycle as usize].get(&Position::new(x, y)) {
                print!("X");
            } else {
                print!(".");
            }
        }

        println!();
    }

    for _ in 1..map.goal {
        print!("#");
    }
    if state.pos.x == map.goal && state.pos.y == map.height - 1 {
        print!("E");
    } else {
        print!(".");
    }
    for _ in (map.goal + 2)..map.width - 1 {
        print!("#");
    }
    println!();
    println!()
}

fn move_bliz(bliz: &Vec<Position>, directions: &Vec<Dir>, width: u8, height: u8) -> Vec<Position> {
    let mut result = Vec::new();

    for i in 0..bliz.len() {
        result.push(match directions[i] {
            Dir::N => bliz[i].wrapping_push(0, -1, width, height),
            Dir::S => bliz[i].wrapping_push(0, 1, width, height),
            Dir::E => bliz[i].wrapping_push(1, 0, width, height),
            Dir::W => bliz[i].wrapping_push(-1, 0, width, height),
        });
    }

    result
}

#[derive(Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Debug)]
struct State {
    pos: Position,
    cycle: u16,
}

#[derive(Debug)]
struct Map {
    bliz: Vec<HashSet<Position>>,
    cycle_length: u16,
    height: u8,
    width: u8,
    start: u8,
    goal: u8,
}

#[derive(Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Debug)]
struct Position {
    x: u8,
    y: u8,
}

impl Position {
    const fn new(x: u8, y: u8) -> Self {
        Position { x, y }
    }

    fn checked_push(&self, x_d: i16, y_d: i16, map: &Map) -> Option<Self> {
        let x = self.x as i16 + x_d;
        let y = self.y as i16 + y_d;

        if x < 1 || x >= (map.width - 1) as i16 {
            return None;
        }

        if (y < 1 || y >= (map.height - 1) as i16)
            && !(x == map.goal as i16 && y == map.height as i16 - 1)
            && !(x == map.start as i16 && y == 0)
        {
            return None;
        }

        Some(Position {
            x: x as u8,
            y: y as u8,
        })
    }

    fn wrapping_push(&self, x_d: i16, y_d: i16, width: u8, height: u8) -> Self {
        let mut x = self.x as i16 + x_d;
        let mut y = self.y as i16 + y_d;

        if x < 1 {
            x = width as i16 - 2;
        } else if x >= (width - 1) as i16 {
            x = 1;
        } else if y < 1 {
            y = height as i16 - 2;
        } else if y >= (height - 1) as i16 {
            y = 1;
        }

        Position {
            x: x as u8,
            y: y as u8,
        }
    }
}

#[derive(Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Debug)]
enum Dir {
    N,
    S,
    E,
    W,
}

fn parse(file: &str) -> Map {
    let mut directions: Vec<Dir> = Vec::new();
    let mut init: Vec<Position> = Vec::new();
    let mut start: u8 = 0;
    let mut goal: u8 = 0;

    let input = std::fs::read_to_string(shared::io::expand_file_name(file)).unwrap();
    let lines: Vec<&str> = input.lines().collect();

    let height = lines.len() as u8;
    let width = lines.first().unwrap().len() as u8;

    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if y == 0 && c == '.' {
                start = x as u8;
            }

            if y as u8 == height - 1 && c == '.' {
                goal = x as u8;
            }

            if c == 'v' {
                directions.push(Dir::S);
                init.push(Position::new(x as u8, y as u8));
            }

            if c == '>' {
                directions.push(Dir::E);
                init.push(Position::new(x as u8, y as u8));
            }

            if c == '<' {
                directions.push(Dir::W);
                init.push(Position::new(x as u8, y as u8));
            }

            if c == '^' {
                directions.push(Dir::N);
                init.push(Position::new(x as u8, y as u8));
            }
        }
    }

    let cycle_length = lcd(height as u16 - 2, width as u16 - 2);
    let mut bliz: Vec<HashSet<Position>> = Vec::new();

    let mut c = init;
    bliz.push(HashSet::from_iter(c.iter().cloned()));
    for _ in 1..cycle_length {
        c = move_bliz(&c, &directions, width, height);
        bliz.push(HashSet::from_iter(c.iter().cloned()));
    }

    Map {
        bliz,
        cycle_length,
        height,
        width,
        start,
        goal,
    }
}

fn lcd(a: u16, b: u16) -> u16 {
    (a * b) / gcd(a, b)
}

fn gcd(a: u16, b: u16) -> u16 {
    let mut a = a;
    let mut b = b;
    if a > b {
        let c = b;
        b = a;
        a = c;
    }
    loop {
        let r = b % a;
        if r == 0 {
            return a;
        }
        b = a;
        a = r;
    }
}
