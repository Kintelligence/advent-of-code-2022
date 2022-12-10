extern crate shared;

use shared::io::Reader;

fn main() {
    part_1_2(&mut Reader::open("input.txt").expect("expected reader"));
}

fn part_1_2(reader: &mut Reader) {
    let mut history: Vec<i32> = Vec::new();
    let mut current: i32 = 1;
    history.push(current);
    history.push(current);

    reader.for_each(|line| {
        let line = line.expect("should be line");
        let words: Vec<&str> = line.trim_end().split(' ').collect();
        match words[0] {
            "noop" => {
                history.push(current);
            }
            "addx" => {
                history.push(current);
                current += words[1].parse::<i32>().unwrap();
                history.push(current);
            }
            _ => panic!(),
        }
    });

    let indexes: Vec<usize> = vec![20, 60, 100, 140, 180, 220];

    let mut result = 0;

    for i in indexes {
        let val = i as i32 * history[i];
        dbg!(i, history[i], val);
        result += val;
    }

    println!("{}", result);

    for i in 0..history.len() - 2 {
        if i % 40 == 0 {
            println!();
        }

        if (history[i + 1] - (i as i32) % 40).abs() < 2 {
            print!("#");
        } else {
            print!(" ");
        }
    }
}
