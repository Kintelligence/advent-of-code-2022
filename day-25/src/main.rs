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

    println!("Total: {:?}", start.elapsed());
     */
}

fn part_1(input: &Vec<isize>) {
    let result = input.iter().sum::<isize>();
    println!("{}", write_snafu(result));
}

fn parse(file: &str) -> Vec<isize> {
    std::fs::read_to_string(shared::io::expand_file_name(file))
        .unwrap()
        .lines()
        .map(|l| parse_snafu(l))
        .collect()
}

fn _test() {
    let dec: Vec<isize> = vec![
        1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 15, 20, 2022, 12345, 314159265,
    ];

    let snafu: Vec<&str> = vec![
        "1",
        "2",
        "1=",
        "1-",
        "10",
        "11",
        "12",
        "2=",
        "2-",
        "20",
        "1=0",
        "1-0",
        "1=11-2",
        "1-0---0",
        "1121-1110-1=0",
    ];

    for i in 0..dec.len() {
        println!("Testing: {} - {}", dec[i], snafu[i]);
        println!("Testing parse");
        assert_eq!(dec[i], parse_snafu(snafu[i]));
        println!("Testing write");
        assert_eq!(snafu[i], write_snafu(dec[i]));
        println!("Passed test for {} - {}", dec[i], snafu[i]);
    }
}

fn parse_snafu(input: &str) -> isize {
    let mut result: isize = 0;

    for (i, c) in input.trim_end().chars().rev().enumerate() {
        result += (5_isize.pow(i as u32))
            * match c {
                '=' => -2,
                '-' => -1,
                '0' => 0,
                '1' => 1,
                '2' => 2,
                _ => panic!("unexpected char in SNAFU string"),
            };
    }

    result
}

fn write_snafu(input: isize) -> String {
    let mut result = String::new();
    let mut input = input.clone();

    let mut i = 0;
    loop {
        let mut current = 2 * 5_isize.pow(i);

        for n in 0..i.saturating_sub(1) {
            current -= 2 * 5_isize.pow(n);
        }

        if input < current {
            break;
        }

        i += 1;
    }

    const NUMS: [(char, isize); 5] = [('2', 2), ('1', 1), ('0', 0), ('-', -1), ('=', -2)];

    loop {
        let mut potential = 0;

        for n in 0..i {
            potential += 2 * 5_isize.pow(n);
        }

        for (c, n) in NUMS {
            let current = n * 5_isize.pow(i);
            if input >= current - potential && input <= current + potential {
                input -= current;
                result.push(c);
                break;
            }
        }

        if let Some(val) = i.checked_sub(1) {
            i = val;
        } else {
            break;
        }
    }

    result = result.trim_start_matches('0').to_string();

    result
}
