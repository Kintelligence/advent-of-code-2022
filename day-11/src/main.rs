extern crate shared;

use std::time::SystemTime;

use shared::io::Reader;

fn main() {
    part_1();
    part_2();
}

fn part_1() {
    let start = SystemTime::now();

    let mut input = String::new();

    Reader::open("input.txt")
        .expect("expected reader")
        .read(&mut input);

    let mut monkeys: Vec<Monkey> = input
        .split("Monkey")
        .skip(1)
        .map(|m| {
            let lines: Vec<&str> = m.lines().collect();

            let operation_words: Vec<&str> = lines[2].split_ascii_whitespace().collect();

            let operand = match operation_words[5] {
                "old" => Operand::Old,
                _ => Operand::Number,
            };

            Monkey {
                items: lines[1]
                    .split(':')
                    .last()
                    .unwrap()
                    .split(',')
                    .map(|n| n.trim().parse().unwrap())
                    .collect(),
                operator: match operation_words[4] {
                    "*" => Operator::Multiply,
                    "+" => Operator::Plus,
                    _ => {
                        panic!("unexpected operator");
                    }
                },
                operand: operand,
                number: match operand {
                    Operand::Old => 0,
                    Operand::Number => operation_words[5].parse().unwrap(),
                },
                test: lines[3].split(' ').last().unwrap().parse().unwrap(),
                if_true: lines[4].split(' ').last().unwrap().parse().unwrap(),
                if_false: lines[5].split(' ').last().unwrap().parse().unwrap(),
            }
        })
        .collect();

    let parse = SystemTime::now();
    println!(
        "Parsing time: {}µs",
        parse.duration_since(start).unwrap().as_micros()
    );

    // Part 1

    let mut counter: Vec<usize> = vec![0; monkeys.len()];

    for _ in 0..20 {
        for monkey_index in 0..monkeys.len() {
            counter[monkey_index] += monkeys[monkey_index].items.len();

            for _ in 0..monkeys[monkey_index].items.len() {
                let mut item = monkeys[monkey_index].items[0];
                match monkeys[monkey_index].operator {
                    Operator::Plus => match monkeys[monkey_index].operand {
                        Operand::Old => item += item,
                        Operand::Number => item += monkeys[monkey_index].number,
                    },
                    Operator::Multiply => match monkeys[monkey_index].operand {
                        Operand::Old => item *= item,
                        Operand::Number => item *= monkeys[monkey_index].number,
                    },
                }

                item /= 3;

                if item % monkeys[monkey_index].test == 0 {
                    let index = monkeys[monkey_index].if_true;
                    monkeys[index].items.push(item);
                } else {
                    let index = monkeys[monkey_index].if_false;
                    monkeys[index].items.push(item);
                }

                monkeys[monkey_index].items.remove(0);
            }
        }
    }

    counter.sort();
    counter.reverse();
    dbg!(counter.iter().take(2).product::<usize>());

    let part_1 = SystemTime::now();
    println!(
        "Part 1 time: {}µs",
        part_1.duration_since(parse).unwrap().as_micros()
    );

    let end = SystemTime::now();
    println!(
        "Total time: {}µs",
        end.duration_since(start).unwrap().as_micros()
    );
}

fn part_2() {
    let start = SystemTime::now();

    let mut input = String::new();

    Reader::open("input.txt")
        .expect("expected reader")
        .read(&mut input);

    let mut monkeys: Vec<Monkey> = input
        .split("Monkey")
        .skip(1)
        .map(|m| {
            let lines: Vec<&str> = m.lines().collect();

            let operation_words: Vec<&str> = lines[2].split_ascii_whitespace().collect();

            let operand = match operation_words[5] {
                "old" => Operand::Old,
                _ => Operand::Number,
            };

            Monkey {
                items: lines[1]
                    .split(':')
                    .last()
                    .unwrap()
                    .split(',')
                    .map(|n| n.trim().parse().unwrap())
                    .collect(),
                operator: match operation_words[4] {
                    "*" => Operator::Multiply,
                    "+" => Operator::Plus,
                    _ => {
                        panic!("unexpected operator");
                    }
                },
                operand: operand,
                number: match operand {
                    Operand::Old => 0,
                    Operand::Number => operation_words[5].parse().unwrap(),
                },
                test: lines[3].split(' ').last().unwrap().parse().unwrap(),
                if_true: lines[4].split(' ').last().unwrap().parse().unwrap(),
                if_false: lines[5].split(' ').last().unwrap().parse().unwrap(),
            }
        })
        .collect();

    let parse = SystemTime::now();
    println!(
        "Parsing time: {}µs",
        parse.duration_since(start).unwrap().as_micros()
    );

    // Part 2

    let mut counter: Vec<u64> = vec![0; monkeys.len()];

    let mut magic_number: u64 = 1;

    for i in 0..monkeys.len() {
        magic_number *= monkeys[i].test;
    }

    for _ in 0..10000 {
        for monkey_index in 0..monkeys.len() {
            counter[monkey_index] += monkeys[monkey_index].items.len() as u64;

            for _ in 0..monkeys[monkey_index].items.len() {
                let mut item = monkeys[monkey_index].items[0];

                item = (item % magic_number) + magic_number;

                match monkeys[monkey_index].operator {
                    Operator::Plus => match monkeys[monkey_index].operand {
                        Operand::Old => item = item.checked_add(item).expect("failed to add self"),
                        Operand::Number => {
                            item = item
                                .checked_add(monkeys[monkey_index].number)
                                .expect("failed to add num")
                        }
                    },
                    Operator::Multiply => match monkeys[monkey_index].operand {
                        Operand::Old => item = item.checked_mul(item).expect("failed to mult num"),
                        Operand::Number => {
                            item = item
                                .checked_mul(monkeys[monkey_index].number)
                                .expect("failed to mult num")
                        }
                    },
                }

                if item % monkeys[monkey_index].test == 0 {
                    let index = monkeys[monkey_index].if_true;
                    monkeys[index].items.push(item);
                } else {
                    let index = monkeys[monkey_index].if_false;
                    monkeys[index].items.push(item);
                }

                monkeys[monkey_index].items.remove(0);
            }
        }
    }

    counter.sort();
    counter.reverse();
    dbg!(counter.iter().take(2).product::<u64>());

    let part_2 = SystemTime::now();
    println!(
        "Part 2 time: {}µs",
        part_2.duration_since(parse).unwrap().as_micros()
    );

    let end = SystemTime::now();
    println!(
        "Total time: {}µs",
        end.duration_since(start).unwrap().as_micros()
    );
}

#[derive(Debug)]
struct Monkey {
    items: Vec<u64>,
    operator: Operator,
    operand: Operand,
    number: u64,
    test: u64,
    if_true: usize,
    if_false: usize,
}

#[derive(Debug)]
enum Operator {
    Plus,
    Multiply,
}

#[derive(Clone, Copy, Debug)]
enum Operand {
    Old,
    Number,
}
