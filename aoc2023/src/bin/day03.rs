use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;

lazy_static! {
    static ref NUMBERS: Regex = Regex::new(r"(\d+)").unwrap();
}

fn overlaps(x1: usize, x2: usize, y1: usize, y2: usize) -> bool {
    std::cmp::max(x1, y1) <= std::cmp::min(x2, y2)
}

fn contains_part_indicator(input: &str) -> bool {
    let input = input
        .replace('.', "")
        .replace('0', "")
        .replace('1', "")
        .replace('2', "")
        .replace('3', "")
        .replace('4', "")
        .replace('5', "")
        .replace('6', "")
        .replace('7', "")
        .replace('8', "")
        .replace('9', "");
    input.len() > 0
}

fn get_adjecent_items(index: usize, items: &Vec<MachineItem>) -> Vec<usize> {
    let mut res_items = vec![];
    for item in items {
        let start = if index == 0 { 0 } else { index - 1 };
        let end = index + 1;
        let overlaps = overlaps(start, end, item.start, item.end - 1);

        if overlaps {
            res_items.push(item.value);
        }
    }
    res_items
}

struct MachineItem {
    start: usize,
    end: usize,
    value: usize,
}

#[aoc2023::main(03)]
fn solution(input: &str) -> (usize, usize) {
    let mut total_p1: usize = 0;
    let mut total_p2: usize = 0;
    let mut numbers: HashMap<usize, Vec<MachineItem>> = HashMap::new();
    let lines: Vec<&str> = input.split("\n").collect();
    for (n, line) in lines.iter().enumerate() {
        numbers.insert(n, vec![]);
        for item in NUMBERS.find_iter(line) {
            numbers.get_mut(&n).unwrap().push(MachineItem {
                start: item.start(),
                end: item.end(),
                value: item.as_str().parse().unwrap(),
            });

            //let substrings = vec![];
            let start = if item.start() > 0 {
                item.start() - 1
            } else {
                0
            };
            let end = std::cmp::min(item.end() + 1, line.len());

            if contains_part_indicator(&line[start..end]) {
                total_p1 += item.as_str().parse::<usize>().unwrap();
                continue;
            }
            if n > 0 {
                if contains_part_indicator(&lines[n - 1][start..end]) {
                    total_p1 += item.as_str().parse::<usize>().unwrap();
                    continue;
                }
            }
            if n + 1 < lines.len() {
                if contains_part_indicator(&lines[n + 1][start..end]) {
                    total_p1 += item.as_str().parse::<usize>().unwrap();
                    continue;
                }
            }
        }
    }
    for (n, line) in lines.iter().enumerate() {
        for item in line.match_indices('*') {
            let mut adjecent_items = vec![];
            adjecent_items.extend(get_adjecent_items(item.0, numbers.get(&n).unwrap()));
            if n > 0 {
                adjecent_items.extend(get_adjecent_items(item.0, numbers.get(&(n - 1)).unwrap()));
            }
            if n + 1 < lines.len() {
                adjecent_items.extend(get_adjecent_items(item.0, numbers.get(&(n + 1)).unwrap()));
            }

            if adjecent_items.len() == 2 {
                let score = adjecent_items.iter().product::<usize>();
                println!("score {} for {:?}", score, adjecent_items);
                total_p2 += score;
            }
        }
    }

    (total_p1, total_p2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution1() -> Result<(), String> {
        let test_input = r#"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."#;

        assert_eq!(solution(test_input.trim()).0, 4361);
        Ok(())
    }

    #[test]
    fn test_solution2() -> Result<(), String> {
        let test_input = r#"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."#;

        assert_eq!(solution(test_input.trim()).1, 467835);
        Ok(())
    }
}
