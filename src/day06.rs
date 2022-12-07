use itertools::Itertools;

pub fn part1(input: String) {
    let mut start = 0;
    let input = input.as_bytes();
    loop {
        let mut curr = input[start..start + 4].to_vec();

        curr = curr.into_iter().unique().collect();
        if curr.len() == 4 {
            println!("Result: {:?}", start + 4);
            break;
        }

        start += 1;
    }
}

pub fn part2(input: String) {
    let mut start = 0;
    let input = input.as_bytes();
    loop {
        let mut curr = input[start..start + 14].to_vec();

        curr = curr.into_iter().unique().collect();
        if curr.len() == 14 {
            println!("Result: {:?}", start + 14);
            break;
        }

        start += 1;
    }
}
