extern crate shared;

use std::{collections::VecDeque, time::SystemTime};

fn main() {
    let start = SystemTime::now();

    let mut list = parse("input.txt");

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

fn parse(file: &str) -> Vec<Item> {
    let mut buffer = String::new();
    shared::io::Reader::open(file).unwrap().read(&mut buffer);

    let mut s = Vec::<Item>::new();

    for line in buffer.lines() {
        if line.trim().is_empty() {
            continue;
        }

        let mut st = VecDeque::<Item>::new();
        split_keep(line.trim())
            .iter()
            .filter(|&&x| x != ",")
            .map(|x| *x)
            .for_each(|x| match x {
                "[" => {
                    let new_list = Item::List(VecDeque::<Item>::new());
                    st.push_back(new_list);
                }
                "]" => {
                    if st.len() > 1 {
                        let last = st.pop_back().unwrap();
                        match st.iter_mut().last() {
                            Some(Item::List(list)) => list.push_back(last),
                            _ => st.push_back(last),
                        }
                    }
                }
                a => {
                    let n = a.parse::<u8>().unwrap();
                    match st.iter_mut().last() {
                        Some(Item::List(list)) => list.push_back(Item::Number(n)),
                        _ => st.push_back(Item::Number(n)),
                    }
                }
            });

        s.push(Item::List(st));
    }
    s
}

fn split_keep<'a>(text: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();
    let mut last = 0;
    for (index, matched) in text.match_indices(|c: char| !(c.is_alphanumeric() || c == '\'')) {
        if last != index {
            result.push(&text[last..index]);
        }
        result.push(matched);
        last = index + matched.len();
    }
    if last < text.len() {
        result.push(&text[last..]);
    }
    result
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
    let divider_2 = Item::List(VecDeque::from(vec![Item::List(VecDeque::from(vec![
        Item::Number(2),
    ]))]));
    let divider_6 = Item::List(VecDeque::from(vec![Item::List(VecDeque::from(vec![
        Item::Number(6),
    ]))]));

    list.push(divider_2.clone());
    list.push(divider_6.clone());
    list.sort_unstable();

    let index_2 = 1 + list.iter().position(|item| *item == divider_2).unwrap();
    let index_6 = 1 + list.iter().position(|item| *item == divider_6).unwrap();

    index_2 * index_6
}

#[derive(Eq, PartialEq, Clone, Debug)]
enum Item {
    Number(u8),
    List(VecDeque<Item>),
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
                Item::List(_) => Item::List(VecDeque::from(vec![Item::Number(*left)])).cmp(other),
            },
            Item::List(left) => match other {
                Item::Number(right) => {
                    self.cmp(&Item::List(VecDeque::from(vec![Item::Number(*right)])))
                }
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
