extern crate shared;

use std::time::SystemTime;

use shared::io::Reader;

fn main() {
    let start = SystemTime::now();

    let mut history = [0; 241];
    let mut current: i32 = 1;
    let mut offset: usize = 0;
    history[offset] = current;

    Reader::open("input.txt")
        .expect("expected reader")
        .for_each(|line| {
            let line = line.expect("should be line");
            let words: Vec<&str> = line.trim_end().split(' ').collect();
            match words[0] {
                "noop" => {
                    offset += 1;
                    history[offset] = current;
                }
                "addx" => {
                    offset += 1;
                    history[offset] = current;
                    current += words[1].parse::<i32>().unwrap();
                    offset += 1;
                    history[offset] = current;
                }
                _ => panic!(),
            }
        });

    let indexes: [usize; 6] = [20, 60, 100, 140, 180, 220];
    let sum: i32 = indexes.map(|i| i as i32 * history[i - 1]).iter().sum();
    println!("{}", sum);

    for i in 0..history.len() {
        if i % 40 == 0 {
            println!();
        }

        if (history[i] - (i as i32) % 40).abs() < 2 {
            print!("#");
        } else {
            print!(" ");
        }
    }

    let end = SystemTime::now();
    println!(
        "Total time: {}µs",
        end.duration_since(start).unwrap().as_micros()
    );
}
