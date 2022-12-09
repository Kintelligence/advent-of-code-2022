extern crate shared;

use std::collections::HashSet;

use shared::io::Reader;

fn main() {
    let result = part_1(&mut Reader::open("input.txt").expect("expected reader"));

    println!("{result}");

    let result = part_2(&mut Reader::open("input.txt").expect("expected reader"));

    println!("{result}");
}

fn part_1(reader: &mut Reader) -> usize {
    let mut visited: HashSet<Point> = HashSet::new();
    let mut head = Point { x: 0, y: 0 };
    let mut tail = Point { x: 0, y: 0 };

    reader.for_each(|line| {
        let line = line.expect("expected line");
        let operation: Vec<&str> = line.trim_end().split(' ').collect();
        let amount: u32 = operation[1].parse().expect("Expected number");

        for _ in 0..amount {
            execute_move(operation[0], &mut head);
            execute_drag(head, &mut tail);
            visited.insert(tail.clone());
        }
    });
    //print(visited.clone());
    visited.len()
}

#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy, PartialOrd, Ord)]
struct Point {
    x: i32,
    y: i32,
}

fn execute_move(direction: &str, head: &mut Point) {
    match direction {
        "U" => head.y += 1,
        "R" => head.x += 1,
        "D" => head.y -= 1,
        "L" => head.x -= 1,
        _ => panic!("unexpected direction"),
    }
}

fn execute_drag(head: Point, tail: &mut Point) {
    let diff_x = head.x - tail.x;
    let diff_y = head.y - tail.y;

    if diff_x.abs() > 1 {
        tail.x += diff_x.signum();
        if diff_y != 0 {
            tail.y += diff_y.signum();
        }
    } else if diff_y.abs() > 1 {
        tail.y += diff_y.signum();
        if diff_x != 0 {
            tail.x += diff_x.signum();
        }
    }
}

fn part_2(reader: &mut Reader) -> usize {
    let mut visited: HashSet<Point> = HashSet::new();
    let mut rope: Vec<Point> = vec![Point { x: 0, y: 0 }; 10];

    reader.for_each(|line| {
        let line = line.expect("expected line");
        let operation: Vec<&str> = line.trim_end().split(' ').collect();
        let amount: u32 = operation[1].parse().expect("Expected number");

        for _ in 0..amount {
            execute_move(operation[0], &mut rope.get_mut(0).unwrap());

            for i in 1..rope.len() {
                execute_drag(*rope.get(i - 1).unwrap(), &mut rope.get_mut(i).unwrap())
            }
            visited.insert(*rope.last().unwrap());
        }
    });
    //print(visited.clone());
    visited.len()
}
