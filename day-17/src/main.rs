use std::ops::BitOrAssign;

fn main() {
    let start = std::time::SystemTime::now();

    let data = parse("input.txt");

    println!("Parse: {:?}", start.elapsed());
    let time_1 = std::time::SystemTime::now();

    part_1(&data);

    println!("Part 1: {:?}", time_1.elapsed());

    /*
    let time_2 = std::time::SystemTime::now();

    part_2(&mut data);

    println!("Part 2: {:?}", time_2.elapsed());

    println!("Total: {:?}", start.elapsed());
    */
}

fn parse(file: &str) -> Vec<Direction> {
    std::fs::read_to_string(shared::io::expand_file_name(file))
        .unwrap()
        .as_bytes()
        .iter()
        .map(|c| match c {
            60 => Direction::Left,
            62 => Direction::Right,
            _ => panic!("Expected < or >"),
        })
        .collect()
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Direction {
    Left,
    Right,
}

const SIZE: usize = 5_000_000;
const PRINT: bool = false;

fn part_1(input: &Vec<Direction>) {
    const SHAPES: [Shape; 5] = [FLAT, PLUS, CORNER, LINE, BOX];
    //const SHAPES: [Shape; 5] = [FULL, FULL, FULL, FULL, FULL];
    let mut chamber = Chamber::new();
    let mut wind = 0;

    for i in 0..1000000000000 {
        if i % 10000000 == 0 {
            println!("{:06}/100000", i / 10000000);
        }

        let mut shape = SHAPES[i % 5].clone();
        chamber.make_space();
        let mut offset: usize = 0;

        loop {
            chamber._draw(&shape, offset);
            if let Some(moved) = shape.push(input[wind], chamber.get_segment(offset)) {
                shape = moved;
                chamber._draw(&shape, offset);
            }

            wind += 1;
            wind %= input.len();
            offset += 1;

            if !shape.check(chamber.get_segment(offset)) {
                chamber.add_shape(&shape, offset - 1);
                break;
            }
        }
    }

    chamber._draw_alone();
    println!("{}", chamber.height);
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct Line {
    bits: u8,
}

const EMPTY: Line = Line::new(0);
const WALL: Line = Line::new(0b00000001);
const FLOOR: Line = Line::new(0b11111111);
const _FILL: Line = Line::new(0b11111110);

impl Line {
    const fn new(bits: u8) -> Self {
        Self { bits }
    }

    fn check(&self, other: &Self) -> bool {
        (self.bits & other.bits) == 0
    }

    fn add(&mut self, other: &Self) {
        self.bits.bitor_assign(other.bits);
    }

    fn is_full(&self) -> bool {
        self.bits == 0b11111111
    }

    fn push(&self, direction: Direction, other: &Self) -> Option<Self> {
        match direction {
            Direction::Left => {
                let after = self.bits << 1;
                if after.count_ones() == self.bits.count_ones() {
                    let line = Line { bits: after };
                    if line.check(&other) {
                        return Some(line);
                    }
                }
            }
            Direction::Right => {
                let after = self.bits >> 1;
                if after.count_ones() == self.bits.count_ones() {
                    let line = Line { bits: after };
                    if line.check(&other) {
                        return Some(line);
                    }
                }
            }
        }

        None
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct Shape {
    lines: [Line; 4],
    height: u8,
}
const _FULL: Shape = Shape::new([_FILL, _FILL, _FILL, _FILL], 4);
const FLAT: Shape = Shape::new([Line::new(0b00111100), EMPTY, EMPTY, EMPTY], 1);
const PLUS: Shape = Shape::new(
    [
        Line::new(0b00010000),
        Line::new(0b00111000),
        Line::new(0b00010000),
        EMPTY,
    ],
    3,
);
const CORNER: Shape = Shape::new(
    [
        Line::new(0b00111000),
        Line::new(0b00001000),
        Line::new(0b00001000),
        EMPTY,
    ],
    3,
);
const LINE: Shape = Shape::new(
    [
        Line::new(0b00100000),
        Line::new(0b00100000),
        Line::new(0b00100000),
        Line::new(0b00100000),
    ],
    4,
);
const BOX: Shape = Shape::new(
    [Line::new(0b00110000), Line::new(0b00110000), EMPTY, EMPTY],
    2,
);

impl Shape {
    fn check(&self, background: &[Line]) -> bool {
        for i in 0..4 {
            if !self.lines[i].check(&background[i]) {
                return false;
            }
        }

        return true;
    }

    fn push(&self, direction: Direction, background: &[Line]) -> Option<Self> {
        let mut shape = Shape {
            lines: [EMPTY; 4],
            height: self.height,
        };

        for i in 0..4 {
            if let Some(line) = self.lines[i].push(direction, &background[i]) {
                shape.lines[i] = line;
            } else {
                return None;
            }
        }

        return Some(shape);
    }

    const fn new(lines: [Line; 4], height: u8) -> Self {
        Shape {
            lines: lines,
            height: height,
        }
    }
}

struct Chamber {
    lines: [Line; SIZE],
    head: usize,
    height: usize,
    tail: usize,
}

impl Chamber {
    const fn new() -> Self {
        let mut chamber = Chamber {
            lines: [EMPTY; SIZE],
            head: 1,
            height: 0,
            tail: 0,
        };
        chamber.lines[0] = FLOOR;
        chamber
    }

    fn make_space(&mut self) {
        if self.head + 6 >= SIZE {
            //println!("resetting offset");

            self.lines.copy_within(self.tail..self.head, 3);
            self.head -= self.tail - 3;
            self.tail = 3;
        }

        for i in 0..7 {
            self.lines[(self.head + i)] = WALL;
        }
    }

    fn get_segment(&self, offset: usize) -> &[Line] {
        let index = self.head + 3 - offset;
        return &self.lines[index..index + 4];
    }

    fn add_shape(&mut self, shape: &Shape, offset: usize) {
        let index = self.head + 3 - offset;
        for i in 0..shape.height as usize {
            let line = self.lines.get_mut(index + i).unwrap();
            line.add(&shape.lines[i]);

            if line.is_full() {
                self.tail = index + i;
            }
        }

        if let Some(diff) = (index + shape.height as usize).checked_sub(self.head) {
            self.head += diff;
            self.height += diff;
        }
    }

    fn _draw(&self, shape: &Shape, offset: usize) {
        if PRINT {
            let shape_bot = self.head + 3 - offset;
            let shape_top = self.head + 7 - offset;

            println!();
            for i in (self.tail..self.head + 7).rev() {
                if i >= shape_bot && i < shape_top {
                    println!(
                        "{:#010b}",
                        self.lines[i].bits | shape.lines[i - shape_bot].bits
                    );
                } else {
                    println!("{:#010b}", self.lines[i].bits);
                }
            }

            println!();
        }
    }

    fn _draw_alone(&self) {
        if PRINT {
            println!();
            for i in (self.tail..self.head + 3).rev() {
                println!("{:#010b}", self.lines[i].bits);
            }

            println!();
        }
    }
}
