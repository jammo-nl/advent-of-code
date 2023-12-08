use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;

lazy_static! {
    static ref ITEM: Regex = Regex::new(r#"(\w+) = \((\w+), (\w+)\)"#).unwrap();
}

// find the lowest common multiple, algorithm from
// https://github.com/TheAlgorithms/Rust/blob/master/src/math/lcm_of_n_numbers.rs
pub fn lcm(nums: &[usize]) -> usize {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lcm(&nums[1..]);
    a * b / gcd_of_two_numbers(a, b)
}

fn gcd_of_two_numbers(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }
    gcd_of_two_numbers(b, a % b)
}

#[aoc2023::main(08)]
fn solution(input: &str) -> (usize, usize) {
    let mut map = HashMap::new();
    let mut order: Vec<u8> = vec![];
    for (item, line) in input.lines().enumerate() {
        if item == 0 {
            order = line.chars().map(|i| if i == 'L' { 0 } else { 1 }).collect();
        } else if !line.is_empty() {
            let capt = ITEM.captures(line).unwrap();
            map.insert(
                capt.get(1).unwrap().as_str(),
                (capt.get(2).unwrap().as_str(), capt.get(3).unwrap().as_str()),
            );
        }
    }

    let mut current = "AAA";
    let mut currents: Vec<&str> = vec![];
    for i in map.keys() {
        if i.ends_with('A') {
            currents.push(*i)
        }
    }

    let mut total_p1 = 0;
    loop {
        // return ending to prevent unwrap error in test 2
        if order.get(total_p1 % order.len()).unwrap() == &0 {
            current = map.get(&current).unwrap_or(&("ZZZ", "ZZZ")).0;
        } else {
            current = map.get(&current).unwrap_or(&("ZZZ", "ZZZ")).1;
        }

        total_p1 += 1;
        if current == "ZZZ" {
            break;
        }
    }

    // for part 2 determine for each set when they end with a Z
    let mut z_locations = vec![];
    for cur in currents {
        let mut current = cur;
        let mut count = 0;
        loop {
            if order.get(count % order.len()).unwrap() == &0 {
                current = map.get(&current).unwrap_or(&("ZZZ", "ZZZ")).0;
            } else {
                current = map.get(&current).unwrap_or(&("ZZZ", "ZZZ")).1;
            }

            count += 1;
            if current.ends_with('Z') {
                z_locations.push(count);
                break;
            }
        }
    }
    // find the lowest multiple of all values
    let total_p2 = lcm(&z_locations);
    (total_p1, total_p2)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r#"RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)"#;

    const TEST_INPUT2: &str = r#"LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)"#;

    #[test]
    fn test_solution1() -> Result<(), String> {
        assert_eq!(solution(TEST_INPUT.trim()).0, 2);
        Ok(())
    }

    #[test]
    fn test_solution2() -> Result<(), String> {
        assert_eq!(solution(TEST_INPUT2.trim()).1, 6);
        Ok(())
    }
}
