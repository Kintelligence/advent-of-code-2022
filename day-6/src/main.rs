extern crate shared;
use std::collections::{HashMap, HashSet};

use shared::io::Reader;

fn main() {
    let result = part_1(&mut Reader::open("input.txt").expect("expected reader"));

    println!("{result}");

    let result = part_2(&mut Reader::open("input.txt").expect("expected reader"));

    println!("{result}");
}

fn part_1(reader: &mut Reader) -> u32 {
    let mut buffer = String::new();
    let input = reader.read_line(&mut buffer);

    input
        .chars()
        .collect::<Vec<char>>()
        .windows(4)
        .position(|slice| slice.iter().collect::<HashSet<&char>>().len() == 4)
        .expect("expect match") as u32
        + 4
}

fn part_2(reader: &mut Reader) -> u32 {
    let mut buffer = String::new();
    let input = reader.read_line(&mut buffer);

    input
        .chars()
        .collect::<Vec<char>>()
        .windows(14)
        .position(|slice| slice.iter().collect::<HashSet<&char>>().len() == 14)
        .expect("expect match") as u32
        + 14
}
