extern crate shared;

use itertools::Itertools;
use std::collections::HashSet;

use shared::io::Reader;

fn main() {
    let result = part_1(&mut Reader::open("input.txt").expect("expected reader"));

    println!("{result}");

    let result = part_2(&mut Reader::open("input.txt").expect("expected reader"));

    println!("{result}");
}

fn part_1(reader: &mut Reader) -> u32 {
    reader
        .map(|line| {
            let line = line.expect("bad line");
            let mut chars = line.trim_end().chars();
            (
                chars.by_ref().take(line.len() / 2).collect::<HashSet<_>>(),
                chars.by_ref().take(line.len() / 2).collect::<HashSet<_>>(),
            )
        })
        .map(|backpack| {
            let c = backpack
                .0
                .intersection(&backpack.1)
                .next()
                .expect("expected overlap");
            let d = c.clone() as u32;
            if d > 97 {
                d - 96
            } else {
                d - 38
            }
        })
        .sum()
}

fn part_2(reader: &mut Reader) -> u32 {
    reader
        .map(|line| {
            line.expect("expected line")
                .trim_end()
                .chars()
                .collect::<HashSet<char>>()
        })
        .chunks(3)
        .into_iter()
        .map(|group| group.reduce(|a, b| a.intersection(&b).copied().collect::<HashSet<char>>()))
        .map(|intersection| {
            let c = intersection
                .expect("expected overlap")
                .into_iter()
                .next()
                .expect("expected overlap");
            let d = c.clone() as u32;
            if d > 97 {
                d - 96
            } else {
                d - 38
            }
        })
        .sum()
}
