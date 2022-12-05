extern crate shared;
use shared::io::Reader;

fn main() {
    let result = part_1(&mut Reader::open("input.txt").expect("expected reader"));

    println!("{result}");

    let result = part_2(&mut Reader::open("input.txt").expect("expected reader"));

    println!("{result}");
}

fn part_1(reader: &mut Reader) -> String {
    let mut buffer = String::new();
    let mut line = reader.by_ref().read_line(&mut buffer);
    let width = line.len() / 4;

    let mut stacks: Vec<Vec<char>> = vec![vec![]; width];

    loop {
        let chars: Vec<char> = line.chars().collect();
        for i in 0..width {
            let c = chars[i * 4 + 1];
            if c.is_ascii_alphabetic() {
                stacks[i].push(c);
            }
        }

        line = reader.by_ref().read_line(&mut buffer);
        if line.trim().is_empty() {
            break;
        }
    }

    for i in 0..width {
        stacks[i].reverse();
    }

    reader.by_ref().for_each(|line| {
        let line = line.expect("expected line");
        let words = line.trim_end().split_whitespace().collect::<Vec<&str>>();

        let amount: usize = words[1].parse().expect("expect number");
        let source: usize = words[3].parse().expect("expect number");
        let target: usize = words[5].parse().expect("expect number");

        for _ in 0..amount {
            let c: char = stacks[source - 1].pop().expect("not empty");
            stacks[target - 1].push(c);
        }
    });

    let mut result = String::new();
    for i in 0..width {
        result.push(stacks[i].pop().expect("not empty"));
    }
    result
}

fn part_2(reader: &mut Reader) -> String {
    let mut buffer = String::new();
    let mut line = reader.by_ref().read_line(&mut buffer);
    let width = line.len() / 4;

    let mut stacks: Vec<Vec<char>> = vec![vec![]; width];

    loop {
        let chars: Vec<char> = line.chars().collect();
        for i in 0..width {
            let c = chars[i * 4 + 1];
            if c.is_ascii_alphabetic() {
                stacks[i].push(c);
            }
        }

        line = reader.by_ref().read_line(&mut buffer);
        if line.trim().is_empty() {
            break;
        }
    }

    for i in 0..width {
        stacks[i].reverse();
    }

    reader.by_ref().for_each(|line| {
        let line = line.expect("expected line");
        let words = line.trim_end().split_whitespace().collect::<Vec<&str>>();

        let amount: usize = words[1].parse().expect("expect number");
        let source: usize = words[3].parse::<usize>().expect("expect number") - 1;
        let target: usize = words[5].parse::<usize>().expect("expect number") - 1;

        let length = stacks[source].len().saturating_sub(amount);
        let mut tail = stacks[source].split_off(length);
        stacks[target].append(&mut tail);
    });

    let mut result = String::new();
    for i in 0..width {
        result.push(stacks[i].pop().expect("not empty"));
    }
    result
}
