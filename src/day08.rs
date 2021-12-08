use crate::tools::read_lines;
use std::collections::HashMap;

pub fn run_all(input: &str) -> String {
    let part1 = part1(read_lines(input));
    let part2 = part2(read_lines(input));

    format!("
Result part 1: {}
Result part 2: {}
    ", part1, part2)
}

struct SegmentDisplay {
    num0: Vec<char>,
    num1: Vec<char>,
    num2: Vec<char>,
    num3: Vec<char>,
    num4: Vec<char>,
    num5: Vec<char>,
    num6: Vec<char>,
    num7: Vec<char>,
    num8: Vec<char>,
    num9: Vec<char>,
    map: HashMap<String,String>,
}
impl SegmentDisplay {
    fn new() -> Self {
        Self {
            num0: vec!('a','b','c','d','e','f','g'), // 6
            num1: vec!('a','b','c','d','e','f','g'), // 2
            num2: vec!('a','b','c','d','e','f','g'), // 5
            num3: vec!('a','b','c','d','e','f','g'), // 5
            num4: vec!('a','b','c','d','e','f','g'), // 4
            num5: vec!('a','b','c','d','e','f','g'), // 5
            num6: vec!('a','b','c','d','e','f','g'), // 6
            num7: vec!('a','b','c','d','e','f','g'), // 3
            num8: vec!('a','b','c','d','e','f','g'), // 7
            num9: vec!('a','b','c','d','e','f','g'), // 6
            map: HashMap::new()
        }
    }

    fn deduce(&mut self, input: Vec<&str>) {
        // get items by length
        let l_2: Vec<Vec<char>> = input.clone().into_iter()
                .filter(|i| i.len() == 2).collect::<Vec<&str>>().into_iter()
                .map(|item| item.chars().collect()).collect();
        let l_3: Vec<Vec<char>> = input.clone().into_iter()
                .filter(|i| i.len() == 3).collect::<Vec<&str>>().into_iter()
                .map(|item| item.chars().collect()).collect();
        let l_4: Vec<Vec<char>> = input.clone().into_iter()
                .filter(|i| i.len() == 4).collect::<Vec<&str>>().into_iter()
                .map(|item| item.chars().collect()).collect();
        let mut l_5: Vec<Vec<char>> = input.clone().into_iter()
                .filter(|i| i.len() == 5).collect::<Vec<&str>>().into_iter()
                .map(|item| item.chars().collect()).collect();
        let mut l_6: Vec<Vec<char>> = input.clone().into_iter()
                .filter(|i| i.len() == 6).collect::<Vec<&str>>().into_iter()
                .map(|item| item.chars().collect()).collect();

        // number 1
        self.num1.retain(|chr| l_2.first().unwrap().contains(chr));

        // number 7
        self.num7.retain(|chr| l_3.first().unwrap().contains(chr));

        // number 4
        self.num4.retain(|chr| l_4.first().unwrap().contains(chr));

        // number 3 has 5 segments and has all segments of number 1 (2 and 5 don't)
        let num_3_items = self.return_containing(&mut l_5, &self.num1);
        self.num3.retain(|chr| num_3_items.contains(chr));

        // number 9 has 6 segments and has all segments of number 4 (0 and 6 don't)
        let num_9_items = self.return_containing(&mut l_6, &self.num4);
        self.num9.retain(|chr| num_9_items.contains(chr));

        // number 0 has 6 segments and has all segments of number 1 (6 don't 9 is gone)
        let num_0_items = self.return_containing(&mut l_6, &self.num1);
        self.num0.retain(|chr| num_0_items.contains(chr));

        // 6 is left over int the 6 length
        self.num6.retain(|chr| l_6.first().unwrap().contains(chr));

        // number 5 has 5 segments and includes 3+4 - 1
        let mut temp = self.num3.clone();
        let item1 = self.num1.clone();
        temp.append(&mut self.num4.clone());
        temp.retain(|i| !item1.contains(i));
        let num_5_items = self.return_containing(&mut l_5, &temp);
        self.num5.retain(|chr| num_5_items.contains(chr));

        // number 2 is the only one left in the 5 length
        self.num2.retain(|chr| l_5.first().unwrap().contains(chr));

        // translation map
        self.map.insert(self.num1.iter().collect(), "1".into());
        self.map.insert(self.num2.iter().collect(), "2".into());
        self.map.insert(self.num3.iter().collect(), "3".into());
        self.map.insert(self.num4.iter().collect(), "4".into());
        self.map.insert(self.num5.iter().collect(), "5".into());
        self.map.insert(self.num6.iter().collect(), "6".into());
        self.map.insert(self.num7.iter().collect(), "7".into());
        self.map.insert(self.num8.iter().collect(), "8".into());
        self.map.insert(self.num9.iter().collect(), "9".into());
        self.map.insert(self.num0.iter().collect(), "0".into());
    }

    fn return_containing(&self, list: &mut Vec<Vec<char>>, test: &Vec<char>) -> Vec<char> {
        let mut ret: Vec<char> = vec!();
        let mut idx = 0;
        for (num, vec) in list.iter().enumerate() {
            let mut contains_all = true;
            for item in test {
                if !vec.contains(&item) {
                    contains_all = false;
                }
            }

            if contains_all {
                ret = vec.clone();
                idx = num;
            }
        }

        list.remove(idx);
        
        ret
    }

    fn decode(&self, input: Vec<&str>) -> u32 {
        let mut result: String = "".into();
        for num in input.iter() {
            let mut item = num.chars().collect::<Vec<char>>();
            item.sort();
            result += &self.map[&item.iter().collect::<String>()];
        }

        result.parse::<u32>().unwrap()
    }
}

pub fn part1(input: Vec<String>) -> u32 {
    let mut total:u32 = 0;
    for line in input {
        let (_, output) = line.split_once(" | ").unwrap();
        let digits = output.split(" ")
            .filter(|item| {item.len() == 2 || item.len() == 3 || item.len() == 4 || item.len() == 7})
            .collect::<Vec<&str>>().len();
        total += digits as u32;
    }

    total
}
pub fn part2(input: Vec<String>) -> u32 {
    let mut total:u32 = 0;
    
    for line in input {
        let (input, output) = line.split_once(" | ").unwrap();
        let input: Vec<&str> = input.split(" ").collect();
        let output: Vec<&str> = output.split(" ").collect();

        // process the display
        let mut display = SegmentDisplay::new();
        display.deduce(input);

        // add the decoded number to the total
        total += display.decode(output);
    }

    total
}

#[cfg(test)]
mod tests {
    static INPUT: &str = "./inputs/day08_test.txt";
    
    use super::*;
    #[test]
    fn test_part1() {
        let output = part1(read_lines(INPUT));
        assert_eq!(output, 26)
    }

    #[test]
    fn test_part2() {
        let output = part2(read_lines(INPUT));
        assert_eq!(output, 61229)
    }
}
