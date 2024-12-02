#[aoc2024::main(01)]
fn solution(input: &str) -> (usize, usize) {
    let mut items1: Vec<isize> = vec![];
    let mut items2: Vec<isize> = vec![];
    let mut total_1 = 0;
    let mut total_2 = 0;
    for line in input.lines() {
        let d: Vec<_> = line.split_whitespace().collect();
        items1.push(d[0].parse().unwrap());
        items2.push(d[1].parse().unwrap());
    }

    items1.sort();
    items2.sort();

    for i in 0..items1.len() {
        total_1 += isize::abs(items1[i] - items2[i]);
        total_2 += items1[i] as usize
            * (items2
                .iter()
                .filter(|k| **k == items1[i])
                .collect::<Vec<_>>()
                .len())
    }

    (total_1 as usize, total_2)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r#"
3   4
4   3
2   5
1   3
3   9
3   3"#;

    #[test]
    fn test_solution1() -> Result<(), String> {
        assert_eq!(solution(TEST_INPUT.trim()).0, 11);
        Ok(())
    }

    #[test]
    fn test_solution2() -> Result<(), String> {
        assert_eq!(solution(TEST_INPUT.trim()).1, 31);
        Ok(())
    }
}
