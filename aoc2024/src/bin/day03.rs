use regex::Regex;

#[aoc2024::main(03)]
fn solution(input: &str) -> (usize, usize) {
    let mut total_1 = 0;
    let mut mul_enabled = true;
    let mut total_2 = 0;

    let muls = Regex::new(r#"((?P<do>do\(\))|(?P<dont>don't\(\))|(?P<cmd>mul\((?P<start>\d{1,3}),(?P<end>\d{1,3})\)))"#).unwrap();
    let items = muls.captures_iter(input);

    for item in items {
        if item.name("do").is_some() {
            mul_enabled = true;
        }
        if item.name("dont").is_some() {
            mul_enabled = false;
        }
        if item.name("cmd").is_some() {
            let start = item
                .name("start")
                .unwrap()
                .as_str()
                .parse::<usize>()
                .unwrap();
            let end = item.name("end").unwrap().as_str().parse::<usize>().unwrap();
            total_1 += start * end;

            if mul_enabled {
                total_2 += start * end;
            }
        }
    }

    (total_1, total_2)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r#"
xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"#;
    const TEST2_INPUT: &str = r#"
xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))
"#;

    #[test]
    fn test_solution1() -> Result<(), String> {
        assert_eq!(solution(TEST_INPUT.trim()).0, 161);
        Ok(())
    }

    #[test]
    fn test_solution2() -> Result<(), String> {
        assert_eq!(solution(TEST2_INPUT.trim()).1, 48);
        Ok(())
    }
}
