use std::{
    collections::{hash_map::DefaultHasher, HashMap},
    hash::{Hash, Hasher},
};

fn main() {
    let start = std::time::SystemTime::now();

    let mut data = parse("input.txt");
    println!("Parse: {:?}", start.elapsed());

    let time_1 = std::time::SystemTime::now();
    part_1(&data);

    println!("Part 1: {:?}", time_1.elapsed());

    let time_2 = std::time::SystemTime::now();
    part_2(&mut data);

    println!("Part 2: {:?}", time_2.elapsed());

    println!("Total: {:?}", start.elapsed());
}

fn parse(file: &str) -> HashMap<u64, Monkey> {
    std::fs::read_to_string(shared::io::expand_file_name(file))
        .unwrap()
        .lines()
        .map(|l| {
            let segments: Vec<&str> = l.trim().split(' ').collect();
            let key = hash(&segments[0][0..4]);
            let monkey: Monkey;

            match segments.len() {
                4 => {
                    assert_eq!(segments[1].len(), 4);
                    assert_eq!(segments[1].parse::<i128>().is_err(), true);
                    assert_eq!(segments[3].len(), 4);
                    assert_eq!(segments[3].parse::<i128>().is_err(), true);

                    let left_key = hash(&segments[1]);
                    let right_key = hash(&segments[3]);

                    monkey = match segments[2] {
                        "+" => Monkey::Add(left_key, right_key),
                        "-" => Monkey::Sub(left_key, right_key),
                        "*" => Monkey::Mul(left_key, right_key),
                        "/" => Monkey::Div(left_key, right_key),
                        _ => panic!("Expected to match a math operation"),
                    };
                }
                2 => monkey = Monkey::Num(segments[1].parse().unwrap()),
                _ => panic!("Expected 4 or 2 segments "),
            };

            return (key, monkey);
        })
        .collect()
}

fn hash(input: &str) -> u64 {
    let mut hasher = DefaultHasher::new();
    input.hash(&mut hasher);
    hasher.finish()
}

enum Monkey {
    Num(i128),
    Add(u64, u64),
    Sub(u64, u64),
    Mul(u64, u64),
    Div(u64, u64),
}

fn keys(key: u64, map: &HashMap<u64, Monkey>) -> (u64, u64) {
    match map[&key] {
        Monkey::Num(_) => panic!("can not do keys on a num operation"),
        Monkey::Add(a, b) => (a, b),
        Monkey::Sub(a, b) => (a, b),
        Monkey::Mul(a, b) => (a, b),
        Monkey::Div(a, b) => (a, b),
    }
}

fn value(key: u64, map: &HashMap<u64, Monkey>, cache: &mut HashMap<u64, i128>) -> i128 {
    if let Some(v) = cache.get(&key) {
        return *v;
    }

    let result = match map[&key] {
        Monkey::Num(v) => v,
        Monkey::Add(a, b) => value(a, map, cache)
            .checked_add(value(b, map, cache))
            .unwrap(),
        Monkey::Sub(a, b) => value(a, map, cache)
            .checked_sub(value(b, map, cache))
            .unwrap(),
        Monkey::Mul(a, b) => value(a, map, cache)
            .checked_mul(value(b, map, cache))
            .unwrap(),
        Monkey::Div(a, b) => value(a, map, cache)
            .checked_div(value(b, map, cache))
            .unwrap(),
    };

    cache.insert(key, result);

    return result;
}

fn part_1(map: &HashMap<u64, Monkey>) {
    let mut cache: HashMap<u64, i128> = HashMap::new();
    let result = value(hash("root"), map, &mut cache);

    println!("{}", result);
}

fn part_2(map: &mut HashMap<u64, Monkey>) {
    dbg!(map.len());
    let (left, right) = keys(hash("root"), map);
    let you = hash("humn");

    let mut min: i64 = i64::MIN;
    let mut max: i64 = i64::MAX;

    loop {
        let guess = min.saturating_add(max) / 2;

        println!("testing: {} -> ", guess);

        map.insert(you, Monkey::Num(guess.into()));
        let mut cache: HashMap<u64, i128> = HashMap::new();
        let l = value(left, map, &mut cache);
        let r = value(right, map, &mut cache);

        println!("left: {}", l);
        println!("right: {}", r);

        match l.cmp(&r) {
            std::cmp::Ordering::Less => max = guess,
            std::cmp::Ordering::Equal => {
                println!("MATCH");
                break;
            }
            std::cmp::Ordering::Greater => min = guess,
        }
    }
}
/*
47221333683587
176750512663788

*/
