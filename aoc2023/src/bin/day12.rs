fn calculate_arrangements(
    target_groups: usize,
    items: &Vec<usize>,
    replacement_items: &Vec<String>,
    depth: usize,
    line: &str,
    start: usize,
    currentdbg: &str,
) -> usize {
    let current = items[depth];
    let remaining = items.len() - (depth + 1);
    let space_needed: usize = items[depth..items.len()].iter().sum();

    let mut total = 0;
    for slice in start..=(line.len() - space_needed) {
        let part: &str = &line[slice..slice + current];
        // our previous item can not be a #, the next item can not be a # and the current
        // substring can not contain a .
        if part.contains('.') {
            continue; // skip this item, we have a dot so it is unusable
        } else if slice > 0 && &line[slice - 1..slice] == "#" {
            continue;
        } else if slice + current < line.len() && &line[slice + current..slice + current + 1] == "#"
        {
            continue;
        }

        let mut currentdbg = currentdbg.to_string();
        currentdbg.replace_range(slice..slice + current, &replacement_items[depth]);

        // dive in, start 1 behind the current item
        if remaining > 0 {
            total += calculate_arrangements(
                target_groups,
                items,
                replacement_items,
                depth + 1,
                line,
                slice + current + 1,
                &currentdbg,
            );
            continue;
        }

        if remaining == 0 {
            // Check if the number of groups we have match the starting amount, if this is the case
            // that we have a result.
            let groups = currentdbg.split('.').filter(|i| !i.is_empty()).count();
            if groups == target_groups {
                total += 1;
            }
        }
    }
    return total;
}

#[aoc2023::main(12)]
fn solution(input: &str) -> (usize, usize) {
    let mut total_p1 = 0;
    let mut total_p2 = 0;
    for i in input.lines() {
        let data = i.split(' ').collect::<Vec<&str>>();
        let items = data[1]
            .split(',')
            .map(|i| i.parse().unwrap())
            .collect::<Vec<usize>>();

        total_p1 += calculate_arrangements(
            items.len(),
            &items,
            &items.iter().map(|i| "#".repeat(*i)).collect(),
            0,
            data[0],
            0,
            &data[0].replace('?', "."),
        );

        // let mut expanded_items = items.clone();
        // expanded_items.extend(items.clone());
        // expanded_items.extend(items.clone());
        // expanded_items.extend(items.clone());
        // expanded_items.extend(items.clone());
        // let expanded_line = format!("{0}?{0}?{0}?{0}?{0}", data[0]);

        // println!("process {expanded_line} {:?}", expanded_items);
        // total_p2 += calculate_arrangements(
        //     expanded_items.len(),
        //     &expanded_items,
        //     &expanded_items.iter().map(|i| "#".repeat(*i)).collect(),
        //     0,
        //     &expanded_line,
        //     0,
        //     &expanded_line.replace('?', "."),
        // );
    }
    (total_p1, total_p2)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r#"???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1"#;

    #[test]
    fn test_solution1() -> Result<(), String> {
        assert_eq!(solution(TEST_INPUT.trim()).0, 21);
        Ok(())
    }

    #[test]
    fn test_solution2() -> Result<(), String> {
        assert_eq!(solution(TEST_INPUT.trim()).1, 525152);
        Ok(())
    }
}
