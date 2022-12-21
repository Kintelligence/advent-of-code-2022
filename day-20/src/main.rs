use std::ops::Index;

fn main() {
    let start = std::time::SystemTime::now();

    let data = parse("input.txt");

    println!("Parse: {:?}", start.elapsed());
    let time_1 = std::time::SystemTime::now();
    part_1(&data);

    println!("Part 1: {:?}", time_1.elapsed());

    /*
    let time_2 = std::time::SystemTime::now();
    part_2(&data);

    println!("Part 2: {:?}", time_2.elapsed());

    println!("Total: {:?}", start.elapsed());*/
}

#[derive(Clone, Copy, Debug)]
struct Instruction {
    offset: i16,
    count: u8,
}

const PRINT: bool = false;

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
            if PRINT {
                dbg!(list
                    .iter()
                    .map(|c| (c.offset, c.count))
                    .collect::<Vec<(i16, u8)>>());

                dbg!(i, current.offset);
            }

            current.count += 1;

            let mut offset = i as i16 + current.offset;

            while offset >= size as i16 {
                offset = offset + 1 - size as i16;
            }

            while offset < 0 as i16 {
                offset = offset - 1 + size as i16;
            }

            let mut index = offset as usize;

            if index == 0 && current.offset.signum() == -1 {
                index = size - 1;
            }

            if index == size - 1 && current.offset.signum() == 1 {
                index = 0;
            }

            if PRINT {
                dbg!(index);
            }

            match index.cmp(&i) {
                std::cmp::Ordering::Less => {
                    if PRINT {
                        dbg!("move left");
                    }
                    list.copy_within(index..i, index + 1);
                    i += 1;
                }
                std::cmp::Ordering::Equal => {
                    if PRINT {
                        dbg!("do nothing");
                    }
                }
                std::cmp::Ordering::Greater => {
                    if PRINT {
                        dbg!("move right");
                    }
                    list.copy_within(i + 1..=index, i);
                    i -= 1;
                }
            }

            list[index] = current;
        } else {
            if PRINT {
                dbg!("skip");
            }
        }

        i += 1;
    }

    let start = list.iter().position(|c| c.offset == 0).unwrap();

    if PRINT {
        dbg!(list
            .iter()
            .map(|c| (c.offset, c.count))
            .collect::<Vec<(i16, u8)>>());
    }
    dbg!(
        list[(start + 1000) % size].offset
            + list[(start + 2000) % size].offset
            + list[(start + 3000) % size].offset
    );
}
