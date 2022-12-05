extern crate shared;
use shared::io::Reader;

fn main() {
    let result = part_1(&mut Reader::open("input.txt").expect("expected reader"));

    println!("{result}");

    let result = part_2(&mut Reader::open("input.txt").expect("expected reader"));

    println!("{result}");
}

fn part_1(reader: &mut Reader) -> u32 {
    let mut buffer = String::new();
    let line = reader.by_ref().read_line(&mut buffer);
    let width = line.len() / 4;

    let mut stacks: Vec<Vec<char>> = vec![vec![]; width];

    loop {
        let line = reader.by_ref().read_line(&mut buffer);
        if line.trim().is_empty() {
            break;
        }
    }

    0
}

fn part_2(reader: &mut Reader) -> u32 {
    0
}
