use std::collections::HashMap;

#[derive(Clone, Debug)]
enum Operation {
    Add,
    Multiply,
    Concat,
}
impl Operation {
    fn run(&self, a: usize, b: usize) -> usize {
        match self {
            Operation::Add => a + b,
            Operation::Multiply => a * b,
            Operation::Concat => format!("{a}{b}").parse().unwrap(),
        }
    }
}

fn has_result<T>(total: usize, target: usize, items: &mut T, operations: Vec<Operation>) -> bool
where
    T: IntoIterator<Item = usize> + Iterator<Item = usize> + Clone,
{
    let new = items.next();
    match new {
        Some(new_num) => {
            let mut result = false;
            for op in &operations {
                let int_total = op.run(total, new_num);
                if has_result(int_total, target, &mut items.clone(), operations.clone()) {
                    result = true;
                }
            }
            return result;
        }
        None => return total == target,
    }
}

#[aoc2024::main(07)]
fn solution(input: &str) -> (usize, usize) {
    let mut total_1 = 0;
    let mut total_2 = 0;
    let mut data: HashMap<usize, Vec<usize>> = HashMap::new();
    for line in input.lines() {
        let mut d = line.split(":");
        let total = d.next().unwrap().parse::<usize>().unwrap();
        let numbers = d
            .next()
            .unwrap()
            .split_whitespace()
            .map(|i| i.parse().unwrap())
            .collect::<Vec<usize>>();

        data.insert(total, numbers);
    }

    for (item, numbers) in data.clone().into_iter() {
        let mut numbers = numbers.into_iter();
        let total = numbers.next().unwrap();
        if has_result(
            total,
            item,
            &mut numbers.clone(),
            vec![Operation::Add, Operation::Multiply],
        ) {
            total_1 += item;
        }
    }

    for (item, numbers) in data.clone().into_iter() {
        let mut numbers = numbers.into_iter();
        let total = numbers.next().unwrap();
        if has_result(
            total,
            item,
            &mut numbers.clone(),
            vec![Operation::Add, Operation::Multiply, Operation::Concat],
        ) {
            total_2 += item;
        }
    }

    (total_1, total_2)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r#"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20"#;

    #[test]
    fn test_solution1() -> Result<(), String> {
        assert_eq!(solution(TEST_INPUT.trim()).0, 3749);
        Ok(())
    }

    #[test]
    fn test_solution2() -> Result<(), String> {
        assert_eq!(solution(TEST_INPUT.trim()).1, 11387);
        Ok(())
    }
}
