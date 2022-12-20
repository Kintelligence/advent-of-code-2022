fn main() {
    let start = std::time::SystemTime::now();

    let data = parse("input.txt");
    dbg!(data.iter().max());
    dbg!(data.iter().min());

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

struct Instruction {
    offset: i16,
    count: u8,
}

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
        let current = &mut list[i];
        if current.count == 0 {}

        i += 1;
    }
}
