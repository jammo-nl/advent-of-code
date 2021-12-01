use std::fs;

// return the file content as a vector of u32
fn read_lines(filename: &str) -> Vec<u32> {
    let content = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let mut lines: Vec<u32> = vec!();
    for line in content.lines() {
        if let Ok(value) = line.parse::<u32>()  {
            lines.push(value);
        }
    }
    lines
}

pub fn run_all() -> String {
    let part1 = part1(read_lines("./inputs/day01.txt"));
    let part2 = part2(read_lines("./inputs/day01.txt"));

    format!("
Result part 1: {}
Result part 2: {}
    ", part1, part2)
}

pub fn part1(input: Vec<u32>) -> u32 {
    // keep track of the last depth, this starts as None
    let mut last_input: Option<u32> = None;
    // and count the number of times it increases
    let mut increases: u32 = 0;
 
    // loop over all lines and keep track of all changes / counts to solve the puzzle

    for num in 0..input.len() {
        let line = input.get(num).unwrap().clone();

        //println!("current {} next {},{},{}")
        match last_input {
            Some(last) => {
                if line > last {
                    increases += 1;
                }
            }
            None => {}
        }
        last_input = Some(line);
    }

    increases
}
pub fn part2(input: Vec<u32>) -> u32 {
    // keep track of all value sets (of size 3)
    let mut last_input_set: Option<u32> = None;
    // count the number of times the set depth increases
    let mut increases_sets: u32 = 0;
    
    // loop over all lines and keep track of all changes / counts to solve the puzzle
    for num in 2..input.len() {
        let line = input.get(num).unwrap().clone();

        let prev1 = input.get(num-1).unwrap().clone();
        let prev2 = input.get(num-2).unwrap().clone();
        let new_value = line+prev1+prev2;

        // check if the value is greater than the previous one (if there was one)
        // and update the pervious number.
        match last_input_set {
            Some(last) => {
                if new_value > last {
                    increases_sets += 1;
                }
            }
            None => {}
        }

        last_input_set = Some(new_value);
    }

    increases_sets
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_part1() {
        let input: Vec<u32> = vec!(199,200,208,210,200,207,240,269,260,263);
        let output = super::part1(input);
        assert_eq!(output, 7)
    }

    #[test]
    fn test_part2() {
        let input: Vec<u32> = vec!(199,200,208,210,200,207,240,269,260,263);
        let output = super::part2(input);
        assert_eq!(output, 5)
    }
}