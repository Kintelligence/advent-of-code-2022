extern crate shared;

use shared::io::Reader;

fn main() {
    let result = part_1(&mut Reader::open("input.txt").expect("expected reader"));

    println!("{result}");

    let result = part_2(&mut Reader::open("input.txt").expect("expected reader"));

    println!("{result}");
}

fn part_1(reader: &mut Reader) -> i32 {
    let map = parse(reader);

    let width = map[0].len();
    let height = map.len();

    let mut visibility_map = vec![vec![false; width]; height];

    let mut peak: i32;
    for x in 0..width {
        peak = i32::MIN;
        for y in 0..height {
            if map[y][x] > peak {
                peak = map[y][x];
                visibility_map[y][x] = true;
            }
        }

        peak = i32::MIN;
        for y in (0..height).rev() {
            if map[y][x] > peak {
                peak = map[y][x];
                visibility_map[y][x] = true;
            }
        }
    }

    for y in 0..height {
        peak = i32::MIN;
        for x in 0..width {
            if map[y][x] > peak {
                peak = map[y][x];
                visibility_map[y][x] = true;
            }
        }

        peak = i32::MIN;
        for x in (0..width).rev() {
            if map[y][x] > peak {
                peak = map[y][x];
                visibility_map[y][x] = true;
            }
        }
    }

    visibility_map
        .iter()
        .map(|row| row.iter().map(|visibility| *visibility as i32).sum::<i32>())
        .sum()
}

fn parse(reader: &mut Reader) -> Vec<Vec<i32>> {
    reader
        .map(|line| {
            line.expect("expected line")
                .trim_end()
                .chars()
                .map(|c| c.to_digit(10).expect("expect digit") as i32)
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>()
}

fn part_2(reader: &mut Reader) -> u32 {
    let map = parse(reader);

    let width = map[0].len();
    let height = map.len();

    let mut visibility_map = vec![vec![1; width]; height];

    for y in 0..height {
        for x in 0..width {
            if y == 0 || y == height - 1 || x == 0 || x == width - 1 {
                visibility_map[y][x] = 0;
            } else {
                let current = map[y][x];
                let mut counter = 0;

                for i in x + 1..width {
                    counter += 1;

                    if current <= map[y][i] {
                        break;
                    }
                }

                visibility_map[y][x] *= counter;
                counter = 0;

                for i in (0..x).rev() {
                    counter += 1;

                    if current <= map[y][i] {
                        break;
                    }
                }

                visibility_map[y][x] *= counter;
                counter = 0;

                for j in y + 1..height {
                    counter += 1;

                    if current <= map[j][x] {
                        break;
                    }
                }

                visibility_map[y][x] *= counter;
                counter = 0;

                for j in (0..y).rev() {
                    counter += 1;

                    if current <= map[j][x] {
                        break;
                    }
                }

                visibility_map[y][x] *= counter;
            }
        }
    }

    *visibility_map
        .iter()
        .map(|row| row.iter().max())
        .max()
        .unwrap()
        .unwrap()
}
