use std::iter::zip;

#[aoc2023::main(06)]
fn solution(input: &str) -> (usize, usize) {
    let mut total_p1 = 1;
    let mut total_p2 = 1;
    let mut times = vec![];
    let mut distances = vec![];
    let mut total_time = 0;
    let mut total_distance = 0;

    for line in input.lines() {
        if line.starts_with("Time:") {
            times = line
                .replace("Time:", "")
                .split(' ')
                .filter_map(|i| i.parse::<usize>().ok())
                .collect();
            total_time = line
                .replace("Time:", "")
                .replace(" ", "")
                .parse::<usize>()
                .unwrap();
        } else {
            distances = line
                .replace("Distance:", "")
                .split(' ')
                .filter_map(|i| i.parse::<usize>().ok())
                .collect();
            total_distance = line
                .replace("Distance:", "")
                .replace(" ", "")
                .parse::<usize>()
                .unwrap();
        }
    }

    let timings = zip(times, distances);
    for timing in timings {
        let mut wins = 0;
        for i in 1..timing.0 {
            let time_left = timing.0 - i;
            let distance = i * time_left;
            if distance > timing.1 {
                wins += 1;
            }
        }
        total_p1 *= wins;
    }

    println!("time {} dist {}", total_time, total_distance);
    let mut wins = 0;
    for i in 1..total_time {
        let time_left = total_time - i;
        let distance = i * time_left;
        if distance > total_distance {
            wins += 1;
        }
    }
    total_p2 *= wins;

    (total_p1, total_p2)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r#"Time:      7  15   30
Distance:  9  40  200"#;

    #[test]
    fn test_solution1() -> Result<(), String> {
        assert_eq!(solution(TEST_INPUT.trim()).0, 288);
        Ok(())
    }

    #[test]
    fn test_solution2() -> Result<(), String> {
        assert_eq!(solution(TEST_INPUT.trim()).1, 71503);
        Ok(())
    }
}
