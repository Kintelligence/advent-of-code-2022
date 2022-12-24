use std::collections::HashMap;

fn main() {
    let start = std::time::SystemTime::now();

    let data = parse("input.txt");
    println!("Parse: {:?}", start.elapsed());

    let time_1 = std::time::SystemTime::now();
    //_part_1(&data);

    println!("Part 1: {:?}", time_1.elapsed());

    let time_2 = std::time::SystemTime::now();
    part_2(&data);

    println!("Part 2: {:?}", time_2.elapsed());

    println!("Total: {:?}", start.elapsed());
}
const PRINT: bool = false;
const SIZE: u8 = 50;

fn _part_1(data: &Input) {
    let mut direction: u8 = 0;
    let mut position = Position::new(0, 0);

    for x in 0..u8::MAX {
        position.x = x;
        if let Some(Space::Empty) = data.map.get(&position) {
            break;
        }
    }

    for instruction in data.instructions.iter() {
        match instruction {
            Instruction::Move(value) => {
                for _ in 0..*value {
                    let mut after = position.push(direction);

                    loop {
                        if let Some(Space::Wall) = data.map.get(&after) {
                            break;
                        }

                        if let Some(Space::Empty) = data.map.get(&after) {
                            position = after;
                            break;
                        }

                        after = after.push(direction);
                    }
                }
            }
            Instruction::Left => {
                direction = (direction + 3) % 4;
            }
            Instruction::Right => {
                direction = (direction + 1) % 4;
            }
        }
    }

    dbg!(position.clone());

    println!(
        "{}",
        1000 * (position.y + 1) as u32 + 4 * (position.x + 1) as u32 + direction as u32
    );
}

fn part_2(data: &Input) {
    let mut direction: u8 = 0;
    let mut position = Position::new(0, 0);

    let mut canvas = vec![vec![' '; 200]; 200];
    data.map.iter().for_each(|(p, s)| {
        canvas[p.x as usize][p.y as usize] = match s {
            Space::Wall => '#',
            Space::Empty => '.',
        }
    });

    for x in 0..u8::MAX {
        position.x = x;
        if let Some(Space::Empty) = data.map.get(&position) {
            break;
        }
    }

    let mut c = 'a';

    canvas[position.x as usize][position.y as usize] = c;

    c = ((c as u8) + 1) as char;
    if c > 'z' {
        c = 'a';
    }

    for instruction in data.instructions.iter() {
        match instruction {
            Instruction::Move(value) => {
                for _ in 0..*value {
                    let mut after = position.push(direction);
                    let mut next_direction = direction;
                    if let None = data.map.get(&after) {
                        if position.y == 0
                            && position.x >= SIZE
                            && position.x < 2 * SIZE
                            && direction == 3
                        {
                            if PRINT {
                                println!("wrapping 1 -> 5");
                            }
                            next_direction = 0;
                            after.x = 0;
                            after.y = 3 * SIZE + (position.x - SIZE);
                        } else if position.x == 0 && position.y >= 3 * SIZE && direction == 2 {
                            if PRINT {
                                println!("wrapping 5 -> 1");
                            }
                            next_direction = 1;
                            after.x = SIZE + (position.y - 3 * SIZE);
                            after.y = 0;
                        } else if position.x == SIZE && position.y < SIZE && direction == 2 {
                            if PRINT {
                                println!("wrapping 1 -> 4");
                            }
                            next_direction = 0;
                            after.x = 0;
                            after.y = (3 * SIZE - 1) - (position.y);
                        } else if position.x == 0
                            && position.y < 3 * SIZE
                            && position.y >= 2 * SIZE
                            && direction == 2
                        {
                            if PRINT {
                                println!("wrapping 4 -> 1");
                            }
                            next_direction = 0;
                            after.x = SIZE;
                            after.y = SIZE - 1 - (position.y - 2 * SIZE);
                        } else if position.y == 0
                            && position.x < 3 * SIZE
                            && position.x >= 2 * SIZE
                            && direction == 3
                        {
                            if PRINT {
                                println!("wrapping 3 -> 5");
                            }
                            next_direction = 3;
                            after.x = position.x - 2 * SIZE;
                            after.y = 4 * SIZE - 1;
                        } else if position.y == 4 * SIZE - 1 && position.x < SIZE && direction == 1
                        {
                            if PRINT {
                                println!("wrapping 5 -> 3");
                            }
                            next_direction = 1;
                            after.x = position.x + 2 * SIZE;
                            after.y = 0;
                        } else if position.x == 3 * SIZE - 1 && position.y < SIZE && direction == 0
                        {
                            if PRINT {
                                println!("wrapping 3 -> 6");
                            }
                            next_direction = 2;
                            after.x = 2 * SIZE - 1;
                            after.y = 3 * SIZE - 1 - position.y;
                        } else if position.x == 2 * SIZE - 1
                            && position.y < SIZE * 3
                            && position.y >= SIZE * 2
                            && direction == 0
                        {
                            if PRINT {
                                println!("wrapping 6 -> 3");
                            }
                            next_direction = 2;
                            after.x = 3 * SIZE - 1;
                            after.y = SIZE - 1 - (position.y - SIZE * 2);
                        } else if position.y == SIZE - 1
                            && position.x < SIZE * 3
                            && position.x >= SIZE * 2
                            && direction == 1
                        {
                            if PRINT {
                                println!("wrapping 3 -> 2");
                            }
                            next_direction = 2;
                            after.x = 2 * SIZE - 1;
                            after.y = SIZE + (position.x - SIZE * 2);
                        } else if position.x == 2 * SIZE - 1
                            && position.y < SIZE * 2
                            && position.y >= SIZE
                            && direction == 0
                        {
                            if PRINT {
                                println!("wrapping 2 -> 3");
                            }
                            next_direction = 3;
                            after.x = 2 * SIZE + (position.y - SIZE);
                            after.y = SIZE - 1;
                        } else if position.x == SIZE
                            && position.y < SIZE * 2
                            && position.y >= SIZE
                            && direction == 2
                        {
                            if PRINT {
                                println!("wrapping 2 -> 4");
                            }
                            next_direction = 1;
                            after.x = position.y - SIZE;
                            after.y = 2 * SIZE;
                        } else if position.y == 2 * SIZE && position.x < SIZE && direction == 3 {
                            if PRINT {
                                println!("wrapping 4 -> 2");
                            }
                            next_direction = 0;
                            after.x = SIZE;
                            after.y = SIZE + position.x;
                        } else if position.y == 3 * SIZE - 1
                            && position.x < SIZE * 2
                            && position.x >= SIZE
                            && direction == 1
                        {
                            if PRINT {
                                println!("wrapping 6 -> 5");
                            }
                            next_direction = 2;
                            after.x = SIZE - 1;
                            after.y = 3 * SIZE + (position.x - SIZE);
                        } else if position.x == SIZE - 1
                            && position.y < 4 * SIZE
                            && position.y >= 3 * SIZE
                            && direction == 0
                        {
                            if PRINT {
                                println!("wrapping 5 -> 6");
                            }
                            next_direction = 3;
                            after.x = SIZE + (position.y - SIZE * 3);
                            after.y = SIZE * 3 - 1;
                        } else {
                            dbg!("What!");
                            dbg!(position, direction, next_direction, after);
                        }
                    }

                    if let None = data.map.get(&after) {
                        panic!("OH NO");
                    }

                    if let Some(Space::Wall) = data.map.get(&after) {
                        break;
                    }

                    if let Some(Space::Empty) = data.map.get(&after) {
                        position = after;
                        direction = next_direction;
                        canvas[position.x as usize][position.y as usize] = c;

                        c = ((c as u8) + 1) as char;
                        if c > 'z' {
                            c = 'a';
                        }
                    }
                }

                if PRINT {
                    println!("Move: {}", value);
                    for y in 0..SIZE * 4 {
                        for x in 0..SIZE * 3 {
                            print!("{}", canvas[x as usize][y as usize]);
                        }
                        println!();
                    }
                }
            }
            Instruction::Left => {
                println!("L");
                direction = (direction + 3) % 4;
            }
            Instruction::Right => {
                println!("R");
                direction = (direction + 1) % 4;
            }
        }
    }

    for y in 0..SIZE * 4 {
        for x in 0..SIZE * 3 {
            print!("{}", canvas[x as usize][y as usize]);
        }
        println!();
    }

    println!(
        "{}",
        1000 * (position.y + 1) as u32 + 4 * (position.x + 1) as u32 + direction as u32
    );
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Clone, Copy)]
struct Position {
    x: u8,
    y: u8,
}

impl Position {
    const fn new(x: u8, y: u8) -> Self {
        Position { x, y }
    }

    fn push(&self, direction: u8) -> Self {
        match direction {
            0 => Position::new(self.x + 1, self.y),
            1 => Position::new(self.x, self.y + 1),
            2 => Position::new(self.x - 1, self.y),
            3 => Position::new(self.x, self.y - 1),
            _ => panic!("unknown direction"),
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
enum Space {
    Wall,
    Empty,
}

#[derive(Debug)]
enum Instruction {
    Move(u8),
    Left,
    Right,
}

const HASHTAG: u8 = 35;
const DOT: u8 = 46;

#[derive(Debug)]
struct Input {
    map: HashMap<Position, Space>,
    instructions: Vec<Instruction>,
}

fn parse(file: &str) -> Input {
    let mut map: HashMap<Position, Space> = HashMap::new();
    let mut instructions: Vec<Instruction> = Vec::new();

    for (y, l) in std::fs::read_to_string(shared::io::expand_file_name(file))
        .unwrap()
        .lines()
        .enumerate()
    {
        let l = l.trim_end();

        if l.is_empty() {
            continue;
        }

        if l.starts_with([' ', '.', '#']) {
            for (x, byte) in l.as_bytes().iter().enumerate() {
                match byte {
                    &HASHTAG => {
                        map.insert(Position::new(x as u8, y as u8), Space::Wall);
                    }
                    &DOT => {
                        map.insert(Position::new(x as u8, y as u8), Space::Empty);
                    }
                    _ => {}
                }
            }

            continue;
        }

        let mut start = 0;
        for (i, b) in l.chars().enumerate() {
            if b.is_ascii_alphabetic() {
                if start != i {
                    instructions.push(Instruction::Move(l[start..i].parse().unwrap()));
                }
                instructions.push(match b {
                    'L' => Instruction::Left,
                    'R' => Instruction::Right,
                    _ => panic!(),
                });
                start = i + 1;
            }
        }

        instructions.push(Instruction::Move(l[start..].parse().unwrap()));
    }

    Input { map, instructions }
}
