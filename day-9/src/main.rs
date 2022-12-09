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
        let direction = match operation[0] {
            "U" => Direction::Up,
            "R" => Direction::Right,
            "D" => Direction::Down,
            "L" => Direction::Left,
            _ => panic!("Unknown direction"),
        };
        let amount: u32 = operation[1].parse().expect("Expected number");

        for _ in 0..amount {
            execute_move(direction, &mut head);
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

#[derive(Clone, Copy, Debug)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

fn execute_move(direction: Direction, head: &mut Point) {
    match direction {
        Direction::Up => head.y += 1,
        Direction::Right => head.x += 1,
        Direction::Down => head.y -= 1,
        Direction::Left => head.x -= 1,
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

fn _print(map: HashSet<Point>) {
    let xs: Vec<i32> = map.iter().map(|p| p.x).collect();
    let ys: Vec<i32> = map.iter().map(|p| p.y).collect();

    let x_min = xs.iter().min().unwrap();
    let y_min = ys.iter().min().unwrap();

    dbg!(x_min, y_min);

    let x_range = xs.iter().max().unwrap() - x_min;
    let y_range = ys.iter().max().unwrap() - y_min;

    dbg!(x_range, y_range);

    let mut output: Vec<Vec<char>> =
        vec![vec!['.'; (x_range + 1).try_into().unwrap()]; (y_range + 1).try_into().unwrap()];

    let list: Vec<&Point> = map.iter().collect();

    list.iter()
        .for_each(|p| output[(p.y - y_min) as usize][(p.x - x_min) as usize] = '#');

    output.iter().rev().for_each(|line| {
        line.iter().for_each(|c| print!("{}", c));
        println!();
    })
}

fn part_2(reader: &mut Reader) -> usize {
    let mut visited: HashSet<Point> = HashSet::new();
    let mut rope: Vec<Point> = vec![Point { x: 0, y: 0 }; 10];

    reader.for_each(|line| {
        let line = line.expect("expected line");
        let operation: Vec<&str> = line.trim_end().split(' ').collect();
        let direction = match operation[0] {
            "U" => Direction::Up,
            "R" => Direction::Right,
            "D" => Direction::Down,
            "L" => Direction::Left,
            _ => panic!("Unknown direction"),
        };
        let amount: u32 = operation[1].parse().expect("Expected number");

        for _ in 0..amount {
            execute_move(direction, &mut rope.get_mut(0).unwrap());

            for i in 1..rope.len() {
                execute_drag(*rope.get(i - 1).unwrap(), &mut rope.get_mut(i).unwrap())
            }
            visited.insert(*rope.last().unwrap());
        }
    });
    //print(visited.clone());
    visited.len()
}
