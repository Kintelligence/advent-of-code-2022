extern crate shared;

use std::{collections::BinaryHeap, time::SystemTime};

use shared::io::Reader;

fn main() {
    let start = SystemTime::now();

    let (map, x_start, y_start, x_end, y_end) =
        parse(&mut Reader::open("input.txt").expect("expected reader"));

    let result = part_1(&map, x_start, y_start);

    println!("{result}");

    let middle = SystemTime::now();
    println!(
        "Part 1: {}µs",
        middle.duration_since(start).unwrap().as_micros()
    );

    let result = part_2(&map, x_end, y_end);

    println!("{result}");

    let end = SystemTime::now();

    println!(
        "Part 2: {}µs",
        end.duration_since(middle).unwrap().as_micros()
    );

    println!(
        "Total time: {}µs",
        end.duration_since(start).unwrap().as_micros()
    );
}

fn part_1(map: &Vec<Vec<Node>>, x: usize, y: usize) -> u32 {
    dijkstra(&map, x, y)
}

fn part_2(map: &Vec<Vec<Node>>, x: usize, y: usize) -> u32 {
    dijkstra2(&map, x, y)
}

fn parse(reader: &mut Reader) -> (Vec<Vec<Node>>, usize, usize, usize, usize) {
    let mut y: usize = 0;

    let mut start_x: usize = 0;
    let mut start_y: usize = 0;
    let mut end_x: usize = 0;
    let mut end_y: usize = 0;

    let map = reader
        .map(|l| {
            let mut x: usize = 0;
            let row: Vec<Node> = l
                .expect("expect line")
                .trim_end()
                .chars()
                .map(|c| {
                    let node = Node {
                        x,
                        y,
                        letter: c,
                        height: (match c {
                            'S' => {
                                start_x = x;
                                start_y = y;
                                'a'
                            }
                            'E' => {
                                end_x = x;
                                end_y = y;
                                'z'
                            }
                            _ => c,
                        } as u32
                            - 'a' as u32) as u16,
                    };

                    x += 1;
                    node
                })
                .collect();

            y += 1;
            row
        })
        .collect();

    (map, start_x, start_y, end_x, end_y)
}

fn dijkstra(map: &Vec<Vec<Node>>, x: usize, y: usize) -> u32 {
    let mut cost_map = vec![vec![u32::MAX; map[0].len()]; map.len()];
    let mut heap = BinaryHeap::new();

    cost_map[y][x] = 0;
    heap.push(Priority { x, y, cost: 0 });

    while let Some(Priority { x, y, cost }) = heap.pop() {
        if map[y][x].is_goal() {
            return cost;
        }

        for (n_x, n_y) in map[y][x].neighbours(map) {
            if cost + 1 < cost_map[n_y][n_x] {
                cost_map[n_y][n_x] = cost + 1;
                heap.push(Priority {
                    x: n_x,
                    y: n_y,
                    cost: cost + 1,
                });
            }
        }
    }

    u32::MAX
}

fn dijkstra2(map: &Vec<Vec<Node>>, x: usize, y: usize) -> u32 {
    let mut cost_map = vec![vec![u32::MAX; map[0].len()]; map.len()];
    let mut heap = BinaryHeap::new();

    cost_map[y][x] = 0;
    heap.push(Priority { x, y, cost: 0 });

    while let Some(Priority { x, y, cost }) = heap.pop() {
        if map[y][x].is_goal2() {
            return cost;
        }

        for (n_x, n_y) in map[y][x].neighbours2(map) {
            if cost + 1 < cost_map[n_y][n_x] {
                cost_map[n_y][n_x] = cost + 1;
                heap.push(Priority {
                    x: n_x,
                    y: n_y,
                    cost: cost + 1,
                });
            }
        }
    }

    u32::MAX
}

#[derive(PartialEq, Eq)]
struct Priority {
    cost: u32,
    x: usize,
    y: usize,
}

impl Ord for Priority {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for Priority {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.cost.partial_cmp(&self.cost)
    }
}

#[derive(Debug, Clone, Copy)]
struct Node {
    x: usize,
    y: usize,
    letter: char,
    height: u16,
}

impl Node {
    fn is_goal(&self) -> bool {
        self.letter == 'E'
    }

    fn neighbours(&self, map: &Vec<Vec<Node>>) -> Vec<(usize, usize)> {
        let mut list = Vec::with_capacity(4);

        let adjacent = [
            (Some(self.x + 1), Some(self.y)),
            (Some(self.x), Some(self.y + 1)),
            (self.x.checked_sub(1), Some(self.y)),
            (Some(self.x), self.y.checked_sub(1)),
        ];

        let height = map.len();
        let width = map[0].len();

        for (x, y) in adjacent {
            if x.is_none() || y.is_none() {
                continue;
            }

            let x = x.unwrap();
            let y = y.unwrap();

            if x < width && y < height && map[y][x].height <= self.height + 1 {
                list.push((x, y));
            }
        }

        list
    }

    fn is_goal2(&self) -> bool {
        self.letter == 'a'
    }

    fn neighbours2(&self, map: &Vec<Vec<Node>>) -> Vec<(usize, usize)> {
        let mut list = Vec::with_capacity(4);

        let adjacent = [
            (Some(self.x + 1), Some(self.y)),
            (Some(self.x), Some(self.y + 1)),
            (self.x.checked_sub(1), Some(self.y)),
            (Some(self.x), self.y.checked_sub(1)),
        ];

        let height = map.len();
        let width = map[0].len();

        for (x, y) in adjacent {
            if x.is_none() || y.is_none() {
                continue;
            }

            let x = x.unwrap();
            let y = y.unwrap();

            if x < width && y < height && self.height <= map[y][x].height + 1 {
                list.push((x, y));
            }
        }

        list
    }
}
