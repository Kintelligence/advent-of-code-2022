extern crate shared;

use shared::io::Reader;

fn main() {
    println!("Day 1: {}", {
        part_1(&mut Reader::open("input.txt").expect("expected reader"))
    });
    println!("Day 2: {}", {
        part_2(&mut Reader::open("input.txt").expect("expected reader"))
    });
}

fn part_1(reader: &mut Reader) -> i32 {
    let mut data = String::new();
    reader.read(&mut data);

    let b = data
        .split("\n\n")
        .map(|x| {
            x.lines()
                .map(|l| l.parse().expect("Not an integer"))
                .collect::<Vec<i32>>()
        })
        .map(|elf| elf.iter().sum());

    b.max().expect("Not maxable")
}

fn part_2(reader: &mut Reader) -> i32 {
    let mut data = String::new();
    reader.read(&mut data);

    let mut a: Vec<i32> = data
        .split("\n\n")
        .map(|x| {
            x.lines()
                .map(|l| l.parse().expect("Not an integer"))
                .collect::<Vec<i32>>()
        })
        .map(|elf| elf.iter().sum())
        .collect();
    a.sort();
    a.reverse();
    a.iter().take(3).sum()
}
