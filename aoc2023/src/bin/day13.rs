fn reflection_index<T: PartialEq + std::fmt::Display>(
    items: &Vec<T>,
    skip: Option<usize>,
) -> usize {
    let mut reflects_at = 0;

    for i in 0..(items.len() - 1) {
        // Avoid returning the same item if we want to skip it
        if let Some(skip) = skip {
            if i == skip - 1 {
                continue;
            }
        }

        if items[i] == items[i + 1] {
            let mut reflect = true;
            for j in 1..i + 1 {
                if reflect && (j > i || i + j + 2 > items.len()) {
                    // edge detected, this case means we have a reflection
                    reflects_at = i + 1;
                } else if reflect && items[i - j] != items[i + j + 1] {
                    // if reflect was false ignore it, else set it to false when we detect we
                    // are not reflecting
                    reflect = false;
                }
            }

            // loop ended if reflecct is still true we are reflecting
            if reflect {
                reflects_at = i + 1;
            }
        }
    }

    reflects_at
}
fn line_smudges<T: PartialEq + std::fmt::Display + std::fmt::Debug>(
    items: &Vec<T>,
) -> Vec<(usize, usize, usize)> {
    let mut locations = vec![];
    for i in 0..(items.len() - 1) {
        let line1: Vec<char> = items[i].to_string().chars().collect();
        let line2: Vec<char> = items[i + 1].to_string().chars().collect();

        for idx in 0..line1.len() {
            if line1[idx] != line2[idx] {
                locations.push((idx, i, i + 1));
            }
        }

        if items[i] == items[i + 1] {
            for j in 1..i + 1 {
                if j > i || i + j + 2 > items.len() {
                    // edge detected,
                    break;
                }

                let line1: Vec<char> = items[i - j].to_string().chars().collect();
                let line2: Vec<char> = items[i + j + 1].to_string().chars().collect();

                for idx in 0..line1.len() {
                    if line1[idx] != line2[idx] {
                        locations.push((idx, i - j, i + j + 1));
                    }
                }
            }
        }
    }
    locations
}

fn get_reflection_indexes(block: &str) -> (usize, usize, usize, usize) {
    // horizontal
    let lines: Vec<&str> = block.lines().collect();
    let horizontal = reflection_index(&lines, None);

    let mut cols: Vec<String> = vec![];
    for line in block.lines() {
        for (idx, col) in line.chars().enumerate() {
            if cols.get(idx).is_none() {
                cols.push("".to_string());
            }

            cols[idx] = format!("{}{}", cols[idx], col);
        }
    }
    let vertical = reflection_index(&cols, None);

    (vertical, horizontal, 0, 0)
}

#[aoc2023::main(13)]
fn solution(input: &str) -> (usize, usize) {
    let blocks = input.split("\n\n").collect::<Vec<&str>>();
    let mut total_p1 = 0;
    let mut total_p2 = 0;

    for block in blocks {
        let (vertical, horizontal, vertical2, horizontal2) = get_reflection_indexes(block);
        if vertical2 == 0 && horizontal2 == 0 {
            println!("{block} -- ({vertical},{horizontal}) :: ({vertical2},{horizontal2})\n");
        }
        total_p1 += vertical + (horizontal * 100);
        total_p2 += vertical2 + (horizontal2 * 100);
    }

    (total_p1, total_p2)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r#"#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#
"#;

    #[test]
    fn test_solution1() -> Result<(), String> {
        assert_eq!(solution(TEST_INPUT.trim()).0, 405);
        Ok(())
    }

    #[test]
    fn test_solution2() -> Result<(), String> {
        assert_eq!(solution(TEST_INPUT.trim()).1, 400);
        Ok(())
    }
}
