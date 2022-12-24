use std::collections::{HashMap, VecDeque};

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
        bliz: map.init,
    };

    let (_, cost) = solve(&state, map, goal).unwrap();
    println!("Cost: {}", cost);
}

fn part_2(map: &Map) {
    let start = Position::new(map.start, 0);
    let goal = Position::new(map.goal, map.height - 1);

    let state = State {
        pos: start,
        bliz: map.init,
    };

    let mut total = 0;

    let (state, cost) = solve(&state, map, goal).unwrap();
    total += cost;
    println!("1: {}", cost);
    let (state, cost) = solve(&state, map, start).unwrap();
    total += cost;
    println!("2: {}", cost);
    let (_, cost) = solve(&state, map, goal).unwrap();
    total += cost;
    println!("3: {}", cost);
    println!("Total: {}", total);
}

fn solve(start: &State, map: &Map, goal: Position) -> Option<(State, u16)> {
    let mut cache: HashMap<State, u16> = HashMap::new();
    let mut queue: VecDeque<(State, u16)> = VecDeque::new();

    queue.push_back((*start, 0));
    cache.insert(*start, 0);

    while let Some((state, cost)) = queue.pop_front() {
        if PRINT {
            draw(map, &state);
        }
        let bliz = move_bliz(&state.bliz, &map);
        let mut options: Vec<State> = Vec::new();
        for (x_d, y_d) in DIRECTIONS {
            if let Some(next) = state.pos.checked_push(x_d, y_d, map) {
                let next_state = State {
                    pos: next,
                    bliz: bliz,
                };

                if next == goal {
                    return Some((next_state, cost + 1));
                }

                if let Some(cached) = cache.get(&next_state) {
                    if *cached > cost + 1 {
                        options.push(next_state);
                    }
                } else {
                    options.push(next_state);
                }
            }
        }

        for b in bliz {
            if b.x == u8::MAX {
                break;
            }

            if options.len() == 0 {
                break;
            }

            if let Some(found) = options.iter().position(|o| o.pos == b) {
                options.remove(found);
            }
        }

        for o in options {
            cache.insert(o, cost + 1);
            queue.push_back((o, cost + 1));
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
            } else if let Some(i) = state.bliz.iter().position(|c| c == &Position::new(x, y)) {
                match map.bliz[i] {
                    Dir::N => print!("^"),
                    Dir::S => print!("v"),
                    Dir::E => print!(">"),
                    Dir::W => print!("<"),
                    Dir::None => panic!("wtf"),
                }
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
}

fn move_bliz(blizzards: &[Position; SIZE], map: &Map) -> [Position; SIZE] {
    let mut result = [Position::new(u8::MAX, u8::MAX); SIZE];

    for i in 0..SIZE {
        result[i] = match map.bliz[i] {
            Dir::N => blizzards[i].wrapping_push(0, -1, map.width, map.height),
            Dir::S => blizzards[i].wrapping_push(0, 1, map.width, map.height),
            Dir::E => blizzards[i].wrapping_push(1, 0, map.width, map.height),
            Dir::W => blizzards[i].wrapping_push(-1, 0, map.width, map.height),
            Dir::None => break,
        }
    }

    result
}

#[derive(Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Debug)]
struct State {
    pos: Position,
    bliz: [Position; SIZE],
}

const SIZE: usize = 3000;

#[derive(Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Debug)]
struct Map {
    bliz: [Dir; SIZE],
    init: [Position; SIZE],
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
    None,
}

fn parse(file: &str) -> Map {
    let mut bliz = [Dir::None; SIZE];
    let mut init = [Position::new(u8::MAX, u8::MAX); SIZE];
    let mut i = 0;
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
                bliz[i] = Dir::S;
                init[i].x = x as u8;
                init[i].y = y as u8;

                i += 1;
            }

            if c == '>' {
                bliz[i] = Dir::E;
                init[i].x = x as u8;
                init[i].y = y as u8;

                i += 1;
            }

            if c == '<' {
                bliz[i] = Dir::W;
                init[i].x = x as u8;
                init[i].y = y as u8;

                i += 1;
            }

            if c == '^' {
                bliz[i] = Dir::N;
                init[i].x = x as u8;
                init[i].y = y as u8;

                i += 1;
            }
        }
    }

    Map {
        bliz,
        init,
        height,
        width,
        start,
        goal,
    }
}
