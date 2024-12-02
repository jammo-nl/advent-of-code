fn validate(d: Vec<isize>) -> (bool, usize) {
    let mut direction_incr: Option<bool> = None;
    for i in 0..d.len() - 1 {
        // if the diff it to big, skip to the next item, this is unsafe
        let diff = isize::abs(d[i] - d[i + 1]);

        let dir_incr = d[i] < d[i + 1];

        match direction_incr {
            Some(dir) => {
                if dir != dir_incr {
                    return (false, i);
                }
            }
            None => direction_incr = Some(dir_incr),
        }

        if !(1..=3).contains(&diff) {
            return (false, i);
        }
    }

    (true, 0)
}

#[aoc2024::main(02)]
fn solution(input: &str) -> (usize, usize) {
    let mut total_1 = 0;
    let mut total_2 = 0;

    for line in input.lines() {
        let d: Vec<isize> = line
            .split_whitespace()
            .map(|i| i.parse().unwrap())
            .collect();

        let (result, err_idx) = validate(d.clone());
        if result {
            total_1 += 1;
            total_2 += 1;
        } else {
            let mut new_d = d.clone();
            new_d.remove(err_idx);
            let (s_result, _) = validate(new_d);

            if s_result {
                total_2 += 1;
                continue;
            }
            if err_idx > 0 {
                let mut new_d = d.clone();
                new_d.remove(err_idx - 1);
                let (s_result, _) = validate(new_d);
                if s_result {
                    total_2 += 1;
                    continue;
                }
            }
            if err_idx < d.len() {
                let mut new_d = d.clone();
                new_d.remove(err_idx + 1);
                let (s_result, _) = validate(new_d);
                if s_result {
                    total_2 += 1;
                    continue;
                }
            }
        }
    }

    (total_1 as usize, total_2)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r#"
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"#;

    #[test]
    fn test_solution1() -> Result<(), String> {
        assert_eq!(solution(TEST_INPUT.trim()).0, 2);
        Ok(())
    }

    #[test]
    fn test_solution2() -> Result<(), String> {
        assert_eq!(solution(TEST_INPUT.trim()).1, 4);
        Ok(())
    }
}
