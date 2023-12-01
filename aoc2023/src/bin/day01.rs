struct CalibrationNumbers(u8, u8);
impl CalibrationNumbers {
    fn as_number(&self) -> usize {
        self.0 as usize * 10 + self.1 as usize
    }
}

const P1_NUM: [&str; 10] = ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"];
const P2_NUM: [&str; 19] = [
    "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "one", "two", "three", "four", "five", "six",
    "seven", "eight", "nine",
];

fn convert(input: &str) -> &str {
    match input {
        "one" => "1",
        "two" => "2",
        "three" => "3",
        "four" => "4",
        "five" => "5",
        "six" => "6",
        "seven" => "7",
        "eight" => "8",
        "nine" => "9",
        _ => input,
    }
}

fn get_calibration_set(line: &str, tokens: &[&str]) -> CalibrationNumbers {
    let mut line_items: Vec<(usize, &str)> = vec![];

    for item in tokens {
        line_items.extend(line.match_indices(item));
    }
    if line_items.is_empty() {
        line_items.push((0, "0")); // avoid empty set caused by combining part 1 and 2
    }

    // sort by location
    line_items.sort_by(|a, b| a.0.cmp(&b.0));
    let first = convert(line_items.first().unwrap().1);
    let last = convert(line_items.last().unwrap().1);

    CalibrationNumbers(first.parse().unwrap(), last.parse().unwrap())
}

#[aoc2023::main(01)]
fn solution(input: &str) -> (usize, usize) {
    let mut total_p1: usize = 0;
    let mut total_p2: usize = 0;
    for line in input.split('\n') {
        let num1 = get_calibration_set(line, &P1_NUM);
        let num2 = get_calibration_set(line, &P2_NUM);

        total_p1 += num1.as_number();
        total_p2 += num2.as_number();
    }
    (total_p1, total_p2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution1() -> Result<(), String> {
        let test_input = r#"1abc2
                            pqr3stu8vwx
                            a1b2c3d4e5f
                            treb7uchet"#;

        assert_eq!(solution(test_input.trim()).0, 142);
        Ok(())
    }
    #[test]
    fn test_solution2() -> Result<(), String> {
        let test_input = r#"two1nine
                            eightwothree
                            abcone2threexyz
                            xtwone3four
                            4nineeightseven2
                            zoneight234
                            7pqrstsixteen"#;

        assert_eq!(solution(test_input.trim()).1, 281);
        Ok(())
    }
}
