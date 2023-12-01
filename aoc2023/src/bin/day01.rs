struct CalibrationNumbers(u8, u8);
impl CalibrationNumbers {
    fn as_number(&self) -> usize {
        self.0 as usize * 10 + self.1 as usize
    }
}

fn get_calibration(line: &str) -> CalibrationNumbers {
    let mut num: Option<CalibrationNumbers> = None;
    for char in line.chars() {
        match char.to_string().parse::<u8>() {
            Ok(item) => {
                num = match num {
                    Some(mut cal) => {
                        cal.1 = item;
                        Some(cal)
                    }
                    None => Some(CalibrationNumbers(item, item)),
                };
            }
            Err(_) => (),
        }
    }

    num.unwrap_or(CalibrationNumbers(0, 0))
}

// replace but keep part of the original to avoid breaking overlapping digits
const STR_NUMBERS: [(&str, &str); 9] = [
    ("one", "o1e"),
    ("two", "t2o"),
    ("three", "thr3e"),
    ("four", "fo4r"),
    ("five", "fi5e"),
    ("six", "s6x"),
    ("seven", "se7en"),
    ("eight", "ei8ht"),
    ("nine", "ni9ne"),
];
#[aoc2023::main(01)]
fn solution(input: &str) -> (usize, usize) {
    let mut total_p1: usize = 0;
    let mut total_p2: usize = 0;
    let input = input.trim();
    for line in input.split('\n') {
        let line = line.trim();
        // Convert the strings into numbers
        let mut p2line = line.to_string();
        for n in STR_NUMBERS {
            p2line = p2line.replace(n.0, n.1);
        }
        let num1 = get_calibration(line);
        let num2 = get_calibration(&p2line);

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

        assert_eq!(solution(test_input).0, 142);
        Ok(())
    }
    #[test]
    fn test_solution2() -> Result<(), String> {
        let test_2_input = r#"two1nine
                            eightwothree
                            abcone2threexyz
                            xtwone3four
                            4nineeightseven2
                            zoneight234
                            7pqrstsixteen"#;

        assert_eq!(solution(test_2_input).1, 281);
        Ok(())
    }
}
