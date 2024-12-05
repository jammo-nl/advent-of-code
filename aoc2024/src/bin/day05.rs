fn windows_mut_each<T>(v: &mut [T], n: usize, mut f: impl FnMut(&mut [T])) {
    let mut start = 0;
    let mut end = n;
    while end <= v.len() {
        f(&mut v[start..end]);
        start += 1;
        end += 1;
    }
}

#[aoc2024::main(05)]
fn solution(input: &str) -> (usize, usize) {
    let mut total_1 = 0;
    let mut total_2 = 0;

    let mut rules: Vec<Vec<&str>> = vec![];
    let mut updates: Vec<Vec<&str>> = vec![];

    for line in input.lines() {
        if line.contains('|') {
            rules.push(line.split('|').collect());
        } else if !line.is_empty() {
            updates.push(line.split(',').collect());
        }
    }

    for mut update in updates {
        let mut result: bool = true;

        let mut subres = false;
        while subres == false {
            // maybe we need multiple runs to correct the line
            subres = true;
            windows_mut_each(&mut update, 2, |win| {
                let mut wrong = win.to_vec();
                wrong.reverse();

                if rules.contains(&wrong) {
                    win.reverse();

                    result = false;
                    subres = false;
                }
            });
        }

        if result {
            total_1 += update
                .get(update.len() / 2)
                .unwrap()
                .parse::<u32>()
                .unwrap();
        } else {
            total_2 += update
                .get(update.len() / 2)
                .unwrap()
                .parse::<u32>()
                .unwrap();
        }
    }

    (total_1 as usize, total_2 as usize)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r#"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47"#;

    #[test]
    fn test_solution1() -> Result<(), String> {
        assert_eq!(solution(TEST_INPUT.trim()).0, 143);
        Ok(())
    }

    #[test]
    fn test_solution2() -> Result<(), String> {
        assert_eq!(solution(TEST_INPUT.trim()).1, 123);
        Ok(())
    }
}
