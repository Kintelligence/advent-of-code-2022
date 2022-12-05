extern crate shared;

use shared::io::Reader;

fn main() {
    let result = part_1(&mut Reader::open("input.txt").expect("expected reader"));

    println!("{result}");

    let result = part_2(&mut Reader::open("input.txt").expect("expected reader"));

    println!("{result}");
}

fn part_1(reader: &mut Reader) -> i32 {
    reader
        .map(|line| {
            line.expect("expect line")
                .trim_end()
                .chars()
                .collect::<Vec<char>>()
        })
        .map(|chars| {
            let a = (
                chars.first().expect("Missing first value").clone(),
                chars.last().expect("Missing last value").clone(),
            );
            a
        })
        .map(|line| match line {
            ('A', 'X') => 1 + 3,
            ('A', 'Y') => 2 + 6,
            ('A', 'Z') => 3 + 0,
            ('B', 'X') => 1 + 0,
            ('B', 'Y') => 2 + 3,
            ('B', 'Z') => 3 + 6,
            ('C', 'X') => 1 + 6,
            ('C', 'Y') => 2 + 0,
            ('C', 'Z') => 3 + 3,
            _ => {
                dbg!(line);
                0
            }
        })
        .sum()
}

fn part_2(reader: &mut Reader) -> i32 {
    reader
        .map(|line| {
            line.expect("expect line")
                .trim_end()
                .chars()
                .collect::<Vec<char>>()
        })
        .map(|chars| {
            (
                chars.first().expect("Missing first value").clone(),
                chars.last().expect("Missing last value").clone(),
            )
        })
        .map(|line| match line {
            ('A', 'X') => 3 + 0,
            ('A', 'Y') => 1 + 3,
            ('A', 'Z') => 2 + 6,
            ('B', 'X') => 1 + 0,
            ('B', 'Y') => 2 + 3,
            ('B', 'Z') => 3 + 6,
            ('C', 'X') => 2 + 0,
            ('C', 'Y') => 3 + 3,
            ('C', 'Z') => 1 + 6,
            _ => 0,
        })
        .sum()
}
