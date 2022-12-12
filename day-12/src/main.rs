extern crate shared;

use std::collections::BinaryHeap;

use shared::io::Reader;

fn main() {
    let result = part_1(&mut Reader::open("sample.txt").expect("expected reader"));

    println!("{result}");

    let result = part_2(&mut Reader::open("sample.txt").expect("expected reader"));

    println!("{result}");
}

fn part_1(reader: &mut Reader) -> u32 {
    let (map, x, y) = parse(reader);

    dijkstra(&map, x, y)
}

fn part_2(reader: &mut Reader) -> u32 {
    0
}

fn parse(reader: &mut Reader) -> (Vec<Vec<Node>>, usize, usize) {
    let mut y: usize = 0;

    let mut start_x: usize = 0;
    let mut start_y: usize = 0;

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
                            'E' => 'z',
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

    (map, start_y, start_x)
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

    0
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Priority {
    cost: u32,
    x: usize,
    y: usize,
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
}
