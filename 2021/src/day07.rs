use crate::tools::{read_lines, split_line};

pub fn run_all(input: &str) -> String {
    let data: Vec<String> = read_lines(input);
    let part1 = part1(split_line(&data[0], ","));
    let part2 = part2(split_line(&data[0], ","));

    format!("
Result part 1: {}
Result part 2: {}
    ", part1, part2)
}

pub fn part1(input: Vec<u32>) -> u32 {
    let min = input.iter().min().unwrap();
    let max = input.iter().max().unwrap();
    let mut smallest_path = u32::MAX;

    for p in *min..*max {
        let mut curr_len = 0;
        for item in &input {
            if *item > p {
                curr_len += item-p;
            }
            else {
                curr_len += p-item;
            }
        }

        if curr_len < smallest_path {
            smallest_path = curr_len;
        }
    }
    
    smallest_path
}
pub fn part2(input: Vec<u32>) -> u32 {
    let min = input.iter().min().unwrap();
    let max = input.iter().max().unwrap();
    let mut smallest_path = u32::MAX;

    for p in *min..*max {
        let mut curr_len = 0;
        for item in &input {
            let power;
            if *item > p {
                power = item-p;
            }
            else {
                power = p-item;
            }

            let power_for_step:u32 = (0..=power).sum();
            curr_len += power_for_step;
        }

        if curr_len < smallest_path {
            smallest_path = curr_len;
        }
    }
    
    smallest_path
}
