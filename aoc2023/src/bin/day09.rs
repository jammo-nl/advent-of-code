fn extrapolate_future(nums: &Vec<isize>) -> isize {
    // if the line total is 0 we are done..
    if nums.iter().sum::<isize>() == 0 {
        return 0;
    }

    let mut new_list = vec![];
    for i in 0..nums.len() - 1 {
        new_list.push(nums[i + 1] - nums[i]);
    }

    // return the last returned value + the last item of the list as new item
    let ret = extrapolate_future(&new_list);
    ret + nums.last().unwrap()
}

fn extrapolate_history(nums: &Vec<isize>) -> isize {
    // if the line total is 0 we are done..
    if nums.iter().sum::<isize>() == 0 {
        return 0;
    }

    let mut new_list = vec![];
    for i in 0..nums.len() - 1 {
        new_list.push(nums[i + 1] - nums[i]);
    }

    // return the last returned value + the last item of the list as new item
    let ret = extrapolate_history(&new_list);
    nums.first().unwrap() - ret
}

#[aoc2023::main(09)]
fn solution(input: &str) -> (isize, isize) {
    let mut total_p1 = 0;
    let mut total_p2 = 0;
    for line in input.lines() {
        let nums: Vec<isize> = line
            .split(' ')
            .filter_map(|i| i.parse::<isize>().ok())
            .collect();

        total_p1 += extrapolate_future(&nums);
        total_p2 += extrapolate_history(&nums);
    }

    (total_p1, total_p2)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r#"0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45"#;

    #[test]
    fn test_solution1() -> Result<(), String> {
        assert_eq!(solution(TEST_INPUT.trim()).0, 114);
        Ok(())
    }

    #[test]
    fn test_solution2() -> Result<(), String> {
        assert_eq!(solution(TEST_INPUT.trim()).1, 2);
        Ok(())
    }
}
