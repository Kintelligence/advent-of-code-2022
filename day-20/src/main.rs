fn main() {
    let start = std::time::SystemTime::now();

    let data = parse("input.txt");

    println!("Parse: {:?}", start.elapsed());
    let time_1 = std::time::SystemTime::now();
    part_1(&data);

    println!("Part 1: {:?}", time_1.elapsed());

    let time_2 = std::time::SystemTime::now();
    part_2(&data);

    println!("Part 2: {:?}", time_2.elapsed());

    println!("Total: {:?}", start.elapsed());
}

#[derive(Clone, Copy, Debug)]
struct Instruction {
    offset: i16,
    count: u8,
}

const PRINT_1: bool = false;
const PRINT_2: bool = false;
const PRINT_END_1: bool = false;
const PRINT_END_2: bool = false;

fn parse(file: &str) -> Vec<i16> {
    std::fs::read_to_string(shared::io::expand_file_name(file))
        .unwrap()
        .lines()
        .map(|l| l.trim().parse().expect("expect integer"))
        .collect()
}

fn part_1(data: &Vec<i16>) {
    let mut list: Vec<Instruction> = data
        .iter()
        .map(|d| Instruction {
            offset: *d,
            count: 0,
        })
        .collect();

    let size = data.len();

    let mut i = 0;

    while i < size {
        let mut current = list[i].clone();
        if current.count == 0 {
            current.count += 1;

            let calculated_offset =
                ((current.offset % (size - 1) as i16) as usize + (size - 1)) % (size - 1);
            let mut index = calculated_offset + i;
            if index >= size {
                index %= size;
                index += 1;
            }

            if PRINT_1 {
                print(&list);
                println!(
                    "i: {:<3}, o: {:<3}, c: {:<3} t: {:<3}",
                    i, current.offset, calculated_offset, index
                );
            }

            match index.cmp(&i) {
                std::cmp::Ordering::Less => {
                    if PRINT_1 {
                        println!("move left");
                    }
                    list.copy_within(index..i, index + 1);
                }
                std::cmp::Ordering::Equal => {
                    if PRINT_1 {
                        println!("do nothing");
                    }
                }
                std::cmp::Ordering::Greater => {
                    if PRINT_1 {
                        println!("move right");
                    }
                    list.copy_within(i + 1..=index, i);
                    i -= 1;
                }
            }

            list[index] = current;
        } else {
            if PRINT_1 {
                println!("already visited {}", i);
            }
        }

        i += 1;
    }

    let start = list.iter().position(|c| c.offset == 0).unwrap();

    if PRINT_1 || PRINT_END_1 {
        print(&list);
    }

    println!(
        "Result: {}",
        list[(start + 1000) % size]
            .offset
            .checked_add(list[(start + 2000) % size].offset)
            .unwrap()
            .checked_add(list[(start + 3000) % size].offset)
            .unwrap()
    );
}

#[derive(Clone, Copy, Debug)]
struct BigInstruction {
    value: i64,
    offset: usize,
    order: usize,
}

fn part_2(data: &Vec<i16>) {
    const KEY: i64 = 811589153;
    let size = data.len();
    let mut order = 0;

    let mut list: Vec<BigInstruction> = data
        .iter()
        .map(|d| {
            let instruction = BigInstruction {
                value: (*d as i64).checked_mul(KEY).unwrap(),
                offset: (((*d as i64)
                    .checked_mul(KEY)
                    .unwrap()
                    .rem_euclid((size - 1) as i64)) as usize
                    + (size - 1))
                    % (size - 1),
                order: order,
            };
            order += 1;
            instruction
        })
        .collect();

    for _ in 0..10 {
        for m in 0..data.len() {
            let i = list.iter().position(|c| c.order == m).unwrap();

            let current = list.remove(i);
            let index = (current.offset + i) % (size - 1);

            if PRINT_2 {
                print_big(&list);
                println!("i: {:<3}, o: {:<3} t: {:<3}", i, current.offset, index);
            }

            list.insert(index, current);

            list[index] = current;
        }

        if PRINT_2 || PRINT_END_2 {
            print_big(&list);
        }
    }

    let start = list.iter().position(|c| c.value == 0).unwrap();

    println!(
        "Result: {},{},{} -> {}",
        list[(start + 1000) % size].value,
        list[(start + 2000) % size].value,
        list[(start + 3000) % size].value,
        list[(start + 1000) % size]
            .value
            .checked_add(list[(start + 2000) % size].value)
            .unwrap()
            .checked_add(list[(start + 3000) % size].value)
            .unwrap()
    );
}

fn print_big(data: &Vec<BigInstruction>) {
    for instruction in data {
        print!("{:<12} ", instruction.value);
    }
    println!();
}

fn print(data: &Vec<Instruction>) {
    for instruction in data {
        print!("{:<2} ", instruction.offset);
    }
    println!();

    for instruction in data {
        print!("{:<2} ", instruction.count);
    }
    println!();
}
