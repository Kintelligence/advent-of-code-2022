use std::{collections::VecDeque, time::SystemTime};

fn main() {
    let start = SystemTime::now();

    let mut map = parse("input.txt");
    let result = part_1(&mut map);
    //print_map(&map);
    println!("{}", result);

    let middle = SystemTime::now();
    println!(
        "Part 1: {}µs",
        middle.duration_since(start).unwrap().as_micros()
    );

    let start = SystemTime::now();

    let mut map = parse_2("input.txt");
    let result = part_2(&mut map);
    //print_map_2(&map);
    println!("{}", result);

    let end = SystemTime::now();
    println!(
        "Part 2: {}µs",
        end.duration_since(middle).unwrap().as_micros()
    );

    println!(
        "Total: {}µs",
        end.duration_since(start).unwrap().as_micros()
    );
}

fn parse(file: &str) -> [[u8; 200]; 200] {
    let mut map: [[u8; 200]; 200] = [[0; 200]; 200];

    let mut buffer = String::new();
    shared::io::Reader::open(file).unwrap().read(&mut buffer);
    buffer.lines().for_each(|line| {
        line.split("->")
            .map(|point| {
                let mut coords = point.trim().split(',').map(|c| c.parse::<u16>().unwrap());
                (coords.next().unwrap(), coords.next().unwrap())
            })
            .collect::<Vec<(u16, u16)>>()
            .windows(2)
            .for_each(|segment| {
                let a = segment[0];
                let b = segment[1];

                let x_range = match a.0.cmp(&b.0) {
                    std::cmp::Ordering::Less => (a.0, b.0),
                    std::cmp::Ordering::Equal => (a.0, b.0),
                    std::cmp::Ordering::Greater => (b.0, a.0),
                };

                let y_range = match a.1.cmp(&b.1) {
                    std::cmp::Ordering::Less => (a.1, b.1),
                    std::cmp::Ordering::Equal => (a.1, b.1),
                    std::cmp::Ordering::Greater => (b.1, a.1),
                };

                for x in x_range.0..=x_range.1 {
                    for y in y_range.0..=y_range.1 {
                        map[(x - 450) as usize][y as usize] = 1;
                    }
                }
            })
    });

    map
}

fn _print_map(map: &[[u8; 200]; 200]) {
    for y in 0..map[0].len() {
        for x in 0..map.len() {
            match map[x][y] {
                0 => print!("."),
                1 => print!("◼︎"),
                3 => print!("○"),
                9 => print!("▼"),
                _ => print!("?"),
            }
        }
        println!();
    }
}

fn part_1(map: &mut [[u8; 200]; 200]) -> u32 {
    let mut active_list: VecDeque<(u16, u16)> = VecDeque::new();
    let mut total = 0;
    loop {
        map[50][0] = 2;
        active_list.push_back((50, 0));

        let mut settled = 0;
        let mut end = false;

        for i in 0..active_list.len() {
            let mut sand = active_list.get_mut(i).unwrap();

            if map[sand.0 as usize][(sand.1 + 1) as usize] == 0 {
                sand.1 += 1;
                while map[sand.0 as usize][(sand.1 + 1) as usize] == 0 {
                    sand.1 += 1;
                    if sand.1 >= (map[0].len() - 1) as u16 {
                        end = true;
                        break;
                    }
                }
            } else if map[(sand.0 - 1) as usize][(sand.1 + 1) as usize] == 0 {
                sand.1 += 1;
                sand.0 -= 1;
            } else if map[(sand.0 + 1) as usize][(sand.1 + 1) as usize] == 0 {
                sand.1 += 1;
                sand.0 += 1;
            } else {
                map[sand.0 as usize][sand.1 as usize] = 3;
                settled += 1;
            }
        }

        total += settled;

        for _ in 0..settled {
            active_list.pop_front();
        }

        if end {
            return total;
        }
    }
}

fn parse_2(file: &str) -> [[u8; 200]; 1000] {
    let mut map: [[u8; 200]; 1000] = [[0; 200]; 1000];

    let mut buffer = String::new();
    shared::io::Reader::open(file).unwrap().read(&mut buffer);
    let mut y = 0;

    buffer.lines().for_each(|line| {
        line.split("->")
            .map(|point| {
                let mut coords = point.trim().split(',').map(|c| c.parse::<u16>().unwrap());
                (coords.next().unwrap(), coords.next().unwrap())
            })
            .collect::<Vec<(u16, u16)>>()
            .windows(2)
            .for_each(|segment| {
                let a = segment[0];
                let b = segment[1];

                let x_range = match a.0.cmp(&b.0) {
                    std::cmp::Ordering::Less => (a.0, b.0),
                    std::cmp::Ordering::Equal => (a.0, b.0),
                    std::cmp::Ordering::Greater => (b.0, a.0),
                };

                let y_range = match a.1.cmp(&b.1) {
                    std::cmp::Ordering::Less => (a.1, b.1),
                    std::cmp::Ordering::Equal => (a.1, b.1),
                    std::cmp::Ordering::Greater => (b.1, a.1),
                };

                for x in x_range.0..=x_range.1 {
                    for y in y_range.0..=y_range.1 {
                        map[(x) as usize][y as usize] = 1;
                    }
                }

                y = u16::max(y, y_range.1);
            })
    });

    for i in 0..1000 {
        map[i][(y + 2) as usize] = 1;
    }

    map
}

fn _print_map_2(map: &[[u8; 200]; 1000]) {
    for y in 0..map[0].len() {
        for x in 0..map.len() {
            match map[x][y] {
                0 => print!("."),
                1 => print!("◼︎"),
                3 => print!("○"),
                9 => print!("▼"),
                _ => print!("?"),
            }
        }
        println!();
    }
}

fn part_2(map: &mut [[u8; 200]; 1000]) -> u32 {
    let mut active_list: VecDeque<(u16, u16)> = VecDeque::new();
    let mut total = 0;
    loop {
        if map[500][0] == 3 {
            return total;
        }

        active_list.push_back((500, 0));

        let mut settled = 0;
        let mut end = false;

        for i in 0..active_list.len() {
            let mut sand = active_list.get_mut(i).unwrap();

            if map[sand.0 as usize][(sand.1 + 1) as usize] == 0 {
                sand.1 += 1;
                while map[sand.0 as usize][(sand.1 + 1) as usize] == 0 {
                    sand.1 += 1;
                    if sand.1 >= (map[0].len() - 1) as u16 {
                        end = true;
                        break;
                    }
                }
            } else if map[(sand.0 - 1) as usize][(sand.1 + 1) as usize] == 0 {
                sand.1 += 1;
                sand.0 -= 1;
            } else if map[(sand.0 + 1) as usize][(sand.1 + 1) as usize] == 0 {
                sand.1 += 1;
                sand.0 += 1;
            } else {
                map[sand.0 as usize][sand.1 as usize] = 3;
                settled += 1;
            }
        }

        total += settled;

        for _ in 0..settled {
            active_list.pop_front();
        }

        if end {
            return total;
        }
    }
}
