use itertools::Itertools;

pub fn part1(input: String) -> String {
    let mut start = 0;
    let input = input.as_bytes();
    loop {
        let mut curr = input[start..start + 4].to_vec();

        curr = curr.into_iter().unique().collect();
        if curr.len() == 4 {
            return format!("{:?}", start + 4);
        }

        start += 1;
    }
}

pub fn part2(input: String) -> String {
    let mut start = 0;
    let input = input.as_bytes();
    loop {
        let mut curr = input[start..start + 14].to_vec();

        curr = curr.into_iter().unique().collect();
        if curr.len() == 14 {
            return format!("{:?}", start + 14);
        }

        start += 1;
    }
}
