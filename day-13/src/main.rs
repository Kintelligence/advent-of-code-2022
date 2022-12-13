extern crate serde;
extern crate shared;

use serde::{Deserialize, Serialize};
use std::time::SystemTime;

use shared::io::Reader;

fn main() {
    let start = SystemTime::now();

    let mut list: Vec<Item> = parse("input.txt");

    let parse = SystemTime::now();
    println!(
        "Parsing: {}µs",
        parse.duration_since(start).unwrap().as_micros()
    );

    let result = part_1(&list);

    println!("{result}");

    let middle = SystemTime::now();
    println!(
        "Part 1: {}µs",
        middle.duration_since(parse).unwrap().as_micros()
    );

    let result = part_2(&mut list);

    println!("{result}");

    let end = SystemTime::now();

    println!(
        "Part 2: {}µs",
        end.duration_since(middle).unwrap().as_micros()
    );

    println!(
        "Total time: {}µs",
        end.duration_since(start).unwrap().as_micros()
    );
}

fn parse(input: &str) -> Vec<Item> {
    Reader::open(input)
        .expect("expected reader")
        .filter_map(|line| {
            let line = line.unwrap();
            if line.trim().is_empty() {
                return None;
            } else {
                return Some(serde_json::from_str::<Item>(&line).unwrap());
            }
        })
        .collect()
}

fn part_1(list: &Vec<Item>) -> u32 {
    let mut index = 0;

    list.chunks(2)
        .map(|lines| {
            index += 1;
            let left = &lines[0];
            let right = &lines[1];

            if left.cmp(&right).is_le() {
                return index;
            }
            0
        })
        .sum()
}

fn part_2(list: &mut Vec<Item>) -> usize {
    let divider_2 = Item::List(vec![Item::List(vec![Item::Number(2)])]);
    let divider_6 = Item::List(vec![Item::List(vec![Item::Number(6)])]);

    list.push(divider_2.clone());
    list.push(divider_6.clone());
    list.sort_unstable();

    let index_2 = 1 + list.iter().position(|item| *item == divider_2).unwrap();
    let index_6 = 1 + list.iter().position(|item| *item == divider_6).unwrap();

    index_2 * index_6
}

#[derive(Eq, PartialEq, Clone, Serialize, Deserialize, Debug)]
#[serde(untagged)]
enum Item {
    Number(u8),
    List(Vec<Item>),
}

impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(&other))
    }
}

impl Ord for Item {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self {
            Item::Number(left) => match other {
                Item::Number(right) => left.cmp(&right),
                Item::List(_) => Item::List(vec![Item::Number(*left)]).cmp(other),
            },
            Item::List(left) => match other {
                Item::Number(right) => self.cmp(&Item::List(vec![Item::Number(*right)])),
                Item::List(right) => {
                    let mut left_list = left.iter();
                    let mut right_list = right.iter();
                    loop {
                        let left_item = left_list.next();
                        let right_item = right_list.next();

                        if left_item.is_none() && right_item.is_none() {
                            return std::cmp::Ordering::Equal;
                        }

                        if left_item.is_none() && !right_item.is_none() {
                            return std::cmp::Ordering::Less;
                        }

                        if !left_item.is_none() && right_item.is_none() {
                            return std::cmp::Ordering::Greater;
                        }

                        let comparison = left_item.cmp(&right_item);
                        if comparison.is_ne() {
                            return comparison;
                        }
                    }
                }
            },
        }
    }
}
