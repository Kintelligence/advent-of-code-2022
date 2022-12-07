extern crate shared;

use std::collections::HashMap;

use shared::io::Reader;

fn main() {
    let result = part_1(&mut Reader::open("input.txt").expect("expected reader"));

    println!("{result}");

    let result = part_2(&mut Reader::open("input.txt").expect("expected reader"));

    println!("{result}");
}

fn part_1(reader: &mut Reader) -> u32 {
    let mut cwd = String::new();
    let mut dict: HashMap<String, u32> = HashMap::new();

    reader.for_each(|line| {
        let line = line.expect("expected line");
        let words = line.trim_end().split_whitespace().collect::<Vec<&str>>();

        if words[0] == "$" {
            if words[1] == "cd" {
                if words[2] == ".." {
                    cwd.truncate(cwd.rfind('-').expect("expected delimiter match"));
                } else {
                    cwd.push('-');
                    cwd.push_str(words[2]);

                    if !dict.contains_key(&cwd) {
                        let key = cwd.clone();
                        dict.insert(key, 0);
                    }
                }
            }
        } else {
            if words[0] == "dir" {
                if !dict.contains_key(words[1]) {
                    let mut key = cwd.clone();
                    key.push('-');
                    key.push_str(words[1]);

                    dict.insert(key, 0);
                }
            } else {
                let depth = cwd.split('-').count();
                let mut key = cwd.clone();

                for i in 0..depth - 1 {
                    if i > 0 {
                        key.truncate(key.rfind('-').expect("expected delimiter match"));
                    }

                    *dict.get_mut(&key).expect("expected match") +=
                        words[0].parse::<u32>().expect("expected number");
                }
            }
        }
    });

    dict.iter()
        .map(|(_, value)| {
            if *value <= 100000 {
                return *value;
            }
            0
        })
        .sum()
}

fn part_2(reader: &mut Reader) -> u32 {
    let mut cwd = String::new();
    let mut dict: HashMap<String, u32> = HashMap::new();

    reader.for_each(|line| {
        let line = line.expect("expected line");
        let words = line.trim_end().split_whitespace().collect::<Vec<&str>>();

        if words[0] == "$" {
            if words[1] == "cd" {
                if words[2] == ".." {
                    cwd.truncate(cwd.rfind('-').expect("expected delimiter match"));
                } else {
                    cwd.push('-');
                    cwd.push_str(words[2]);

                    if !dict.contains_key(&cwd) {
                        let key = cwd.clone();
                        dict.insert(key, 0);
                    }
                }
            }
        } else {
            if words[0] == "dir" {
                if !dict.contains_key(words[1]) {
                    let mut key = cwd.clone();
                    key.push('-');
                    key.push_str(words[1]);

                    dict.insert(key, 0);
                }
            } else {
                let depth = cwd.split('-').count();
                let mut key = cwd.clone();
                for i in 0..depth - 1 {
                    if i > 0 {
                        key.truncate(key.rfind('-').expect("expected delimiter match"));
                    }

                    *dict.get_mut(&key).expect("expected match") +=
                        words[0].parse::<u32>().expect("expected number");
                }
            }
        }
    });

    let mut sizes = dict.values().collect::<Vec<&u32>>();
    sizes.sort();

    let target = 30000000 - (70000000 - *sizes.last().expect("expected last size"));
    println!("{}", *sizes.last().expect("expected last size"));
    println!("{}", target);

    **sizes
        .iter()
        .find(|size| {
            println!("{}", ***size);
            ***size >= target
        })
        .expect("expected match")
}
