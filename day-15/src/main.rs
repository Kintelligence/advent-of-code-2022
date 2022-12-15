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

fn parse(file: &str) -> Vec<Sensor> {
    std::fs::read_to_string(shared::io::expand_file_name(file))
        .unwrap()
        .lines()
        .map(|line| {
            let bytes = line.as_bytes();
            let mut positions: [usize; 8] = [bytes.len(); 8];
            let mut position_offset: usize = 0;

            for (i, byte) in bytes.iter().enumerate() {
                match byte {
                    61 => {
                        positions[position_offset] = i + 1;
                        position_offset += 1;
                    }
                    44 => {
                        positions[position_offset] = i;
                        position_offset += 1;
                    }
                    58 => {
                        positions[position_offset] = i;
                        position_offset += 1;
                    }
                    _ => (),
                }
            }

            let mut coords = positions
                .chunks(2)
                .map(|indexes| line[indexes[0]..indexes[1]].parse::<i32>().unwrap());

            Sensor::new(
                coords.next().unwrap(),
                coords.next().unwrap(),
                coords.next().unwrap(),
                coords.next().unwrap(),
            )
        })
        .collect()
}

const LINE_HEIGHT: i32 = 2000000;

fn part_1(data: &Vec<Sensor>) {
    let mut list: Vec<Range> = Vec::with_capacity(data.len() * 2);
    data.iter()
        .filter_map(|sensor| sensor.ranges_on_line(LINE_HEIGHT))
        .for_each(|line| {
            list.push(line.0);
            list.push(line.1);
        });

    list.sort_unstable();
    let mut depth: u8 = 0;
    let mut current: Option<i32> = None;
    let mut count: u32 = 0;

    for range in list {
        match range {
            Range::Start(start) => {
                depth += 1;
                if let None = current {
                    current = Some(start);
                }
            }
            Range::End(end) => {
                depth -= 1;

                if depth == 0 {
                    if let Some(start) = current {
                        current = None;
                        count += start.abs_diff(end);
                    }
                }
            }
        }
    }

    println!("{}", count);
}

const MIN: i32 = 0;
const MAX: i32 = 4000000;

fn part_2(data: &Vec<Sensor>) {
    let mut found: bool = false;

    for height in (MIN..=MAX).rev() {
        if found {
            break;
        }
        let mut list: Vec<Range> = Vec::with_capacity(data.len() * 2);
        data.iter()
            .filter_map(|sensor| sensor.ranges_on_line(height))
            .for_each(|line| {
                list.push(line.0);
                list.push(line.1);
            });

        list.sort_unstable();
        let mut depth: u8 = 0;
        let mut current: Option<i32> = None;

        for range in list {
            match range {
                Range::Start(start) => {
                    if depth == 0 {
                        if let Some(end) = current {
                            if start - end > 1 {
                                println!("{}", height as u64 + (start + 1) as u64 * MAX as u64);
                                found = true;
                                break;
                            }
                        }
                    }

                    depth += 1;
                }
                Range::End(end) => {
                    depth -= 1;

                    if depth == 0 {
                        current = Some(end);
                    }
                }
            }
        }
    }
}

struct Sensor {
    center: Point,
    dist: i32,
}

impl Sensor {
    fn ranges_on_line(&self, height: i32) -> Option<(Range, Range)> {
        let diff = self.dist - self.center.y.abs_diff(height) as i32;
        if diff >= 0 {
            return Some((
                Range::Start(self.center.x - diff),
                Range::End(self.center.x + diff),
            ));
        }

        return None;
    }

    fn new(center_x: i32, center_y: i32, closest_x: i32, closest_y: i32) -> Self {
        Self {
            center: Point {
                x: center_x,
                y: center_y,
            },
            dist: (i32::abs_diff(center_x, closest_x) + i32::abs_diff(center_y, closest_y)) as i32,
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum Range {
    Start(i32),
    End(i32),
}

impl Ord for Range {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Range::Start(a), Range::Start(b)) => a.cmp(b),
            (Range::Start(a), Range::End(b)) => a.cmp(b),
            (Range::End(a), Range::Start(b)) => a.cmp(b),
            (Range::End(a), Range::End(b)) => a.cmp(b),
        }
    }
}

impl PartialOrd for Range {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
struct Point {
    x: i32,
    y: i32,
}
