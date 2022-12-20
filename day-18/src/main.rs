use std::collections::VecDeque;

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

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
struct Cube {
    x: i8,
    y: i8,
    z: i8,
}

fn parse(file: &str) -> Vec<Cube> {
    std::fs::read_to_string(shared::io::expand_file_name(file))
        .unwrap()
        .lines()
        .map(|l| {
            let nums = l
                .trim()
                .split(',')
                .map(|n| n.parse::<i8>().unwrap())
                .collect::<Vec<i8>>();

            Cube {
                x: nums[0],
                y: nums[1],
                z: nums[2],
            }
        })
        .collect()
}

fn part_1(data: &Vec<Cube>) {
    let mut count = data.len() * 6;

    for i in 0..data.len() {
        for n in i..data.len() {
            let a = data[i];
            let b = data[n];

            let x = a.x.abs_diff(b.x);
            let y = a.y.abs_diff(b.y);
            let z = a.z.abs_diff(b.z);

            if (x == 0 && y == 0 && z == 1)
                || (x == 0 && y == 1 && z == 0)
                || (x == 1 && y == 0 && z == 0)
            {
                count -= 2;
            }
        }
    }
    println!("{}", count);
}

fn part_2(data: &Vec<Cube>) {
    let ranges = get_ranges(data);

    let x_width = (ranges.x.max - ranges.x.min + 3) as usize;
    let y_width = (ranges.y.max - ranges.y.min + 3) as usize;
    let z_width = (ranges.z.max - ranges.z.min + 3) as usize;

    let mut map: Vec<Vec<Vec<u8>>> = vec![vec![vec![0; z_width]; y_width]; x_width];

    for cube in data {
        map[(cube.x - ranges.x.min + 1) as usize][(cube.y - ranges.y.min + 1) as usize]
            [(cube.z - ranges.z.min + 1) as usize] = 1;
    }

    let mut i = 0;

    let mut queue: VecDeque<(usize, usize, usize)> = VecDeque::new();
    queue.push_back((0, 0, 0));

    const DIRECTIONS: [(i8, i8, i8); 6] = [
        (0, 0, 1),
        (0, 1, 0),
        (1, 0, 0),
        (-1, 0, 0),
        (0, -1, 0),
        (0, 0, -1),
    ];

    while let Some((cx, cy, cz)) = queue.pop_back() {
        for (dx, dy, dz) in DIRECTIONS {
            let x = cx as i8 + dx;
            let z = cz as i8 + dz;
            let y = cy as i8 + dy;

            if x > x_width as i8 - 1 || x < 0 {
                continue;
            }

            if y > y_width as i8 - 1 || y < 0 {
                continue;
            }

            if z > z_width as i8 - 1 || z < 0 {
                continue;
            }

            match map[x as usize][y as usize][z as usize] {
                0 => {
                    queue.push_back((x as usize, y as usize, z as usize));
                    map[x as usize][y as usize][z as usize] = 2;
                }
                1 => i += 1,
                _ => {}
            }
        }
    }

    println!("{}", i);
}

#[derive(Debug, Clone, Copy)]
struct Ranges {
    x: Range,
    y: Range,
    z: Range,
}

#[derive(Debug, Clone, Copy)]
struct Range {
    min: i8,
    max: i8,
}

fn get_ranges(data: &Vec<Cube>) -> Ranges {
    let mut ranges = Ranges {
        x: Range {
            min: i8::MAX,
            max: i8::MIN,
        },
        y: Range {
            min: i8::MAX,
            max: i8::MIN,
        },
        z: Range {
            min: i8::MAX,
            max: i8::MIN,
        },
    };

    for cube in data {
        if cube.x > ranges.x.max {
            ranges.x.max = cube.x;
        }

        if cube.x < ranges.x.min {
            ranges.x.min = cube.x;
        }

        if cube.y > ranges.y.max {
            ranges.y.max = cube.y;
        }

        if cube.y < ranges.y.min {
            ranges.y.min = cube.y;
        }

        if cube.z > ranges.z.max {
            ranges.z.max = cube.z;
        }

        if cube.z < ranges.z.min {
            ranges.z.min = cube.z;
        }
    }

    ranges
}
