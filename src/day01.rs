use crate::tools::read_lines;

pub fn run_all(input: &str) -> String {
    let part1 = part1(read_lines(input));
    let part2 = part2(read_lines(input));

    format!("
Result part 1: {}
Result part 2: {}
    ", part1, part2)
}

fn part1(input: Vec<u32>) -> u32 {
    // keep track of the last depth, this starts as None
    let mut last_input: Option<u32> = None;
    // and count the number of times it increases
    let mut increases: u32 = 0;
 
    // loop over all lines and keep track of all changes / counts to solve the puzzle

    for num in 0..input.len() {
        let line = *input.get(num).unwrap();

        if let Some(last) = last_input {
            if line > last {
                increases += 1;
            }
        }
        
        last_input = Some(line);
    }

    increases
}
fn part2(input: Vec<u32>) -> u32 {
    // keep track of all value sets (of size 3)
    let mut last_input_set: Option<u32> = None;
    // count the number of times the set depth increases
    let mut increases_sets: u32 = 0;
    
    // loop over all lines and keep track of all changes / counts to solve the puzzle
    for num in 2..input.len() {
        let line = *input.get(num).unwrap();

        let prev1 = *input.get(num-1).unwrap();
        let prev2 = *input.get(num-2).unwrap();
        let new_value = line+prev1+prev2;

        // check if the value is greater than the previous one (if there was one)
        // and update the pervious number.
        if let Some(last) = last_input_set {
            if new_value > last {
                increases_sets += 1;
            }
        }

        last_input_set = Some(new_value);
    }

    increases_sets
}
