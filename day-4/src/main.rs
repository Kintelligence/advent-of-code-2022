extern crate shared;
use shared::io::Reader;

fn main() {
    let result = part_1(&mut Reader::open("input.txt").expect("expected reader"));

    println!("{result}");

    let result = part_2(&mut Reader::open("input.txt").expect("expected reader"));

    println!("{result}");
}

fn part_1(reader: &mut Reader) -> u32 {
    reader
        .map(|line| {
            let line = line.expect("expected line");
            let nums = line
                .trim_end()
                .split(&['-', ','][..])
                .map(|num| num.parse::<u32>().expect("expected number"))
                .collect::<Vec<u32>>();

            ((nums[0] >= nums[2]) == (nums[1] <= nums[3])
                || (nums[0] <= nums[2]) == (nums[1] >= nums[3])) as u32
        })
        .sum()
}

fn part_2(reader: &mut Reader) -> u32 {
    reader
        .map(|line| {
            let line = line.expect("expected line");
            let nums = line
                .trim_end()
                .split(&['-', ','][..])
                .map(|num| num.parse::<u32>().expect("expected number"))
                .collect::<Vec<u32>>();

            ((nums[0] >= nums[2] && nums[0] <= nums[3])
                || (nums[1] >= nums[2] && nums[1] <= nums[3])
                || (nums[2] >= nums[0] && nums[2] <= nums[1])
                || (nums[3] >= nums[0] && nums[3] <= nums[1])) as u32
        })
        .sum()
}
