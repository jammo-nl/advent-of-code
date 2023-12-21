fn reflection_index<T: PartialEq + std::fmt::Display>(items: &Vec<T>) -> (usize, usize) {
    let mut reflects_at = 0;
    let mut smudged_reflection = 0;

    for i in 0..(items.len() - 1) {
        let mut diffs = 0;

        // ignore this if we have a difference of 1 character
        let line1: Vec<char> = items[i].to_string().chars().collect();
        let line2: Vec<char> = items[i + 1].to_string().chars().collect();
        for idx in 0..line1.len() {
            if line1[idx] != line2[idx] {
                diffs += 1;
            }
        }

        if items[i] == items[i + 1] || diffs == 1 {
            let mut reflect = true;
            let mut smudge_reflect = true;
            for j in 1..i + 1 {
                if reflect && diffs == 0 && (j > i || i + j + 2 > items.len()) {
                    // edge detected, this case means we have a reflection
                    reflects_at = i + 1;
                } else if reflect && diffs == 0 && items[i - j] != items[i + j + 1] {
                    // if reflect was false ignore it, else set it to false when we detect we
                    // are not reflecting
                    reflect = false;
                }

                // do the same for smudge detection, ignore if the result is the same
                if smudge_reflect && i + 1 != reflects_at && (j > i || i + j + 2 > items.len()) {
                    smudged_reflection = i + 1;
                } else if smudge_reflect
                    && !(j > i || i + j + 2 > items.len())
                    && items[i - j] != items[i + j + 1]
                {
                    // ignore this if we have a difference of 1 character
                    let line1: Vec<char> = items[i - j].to_string().chars().collect();
                    let line2: Vec<char> = items[i + j + 1].to_string().chars().collect();
                    for idx in 0..line1.len() {
                        if line1[idx] != line2[idx] {
                            diffs += 1;
                        }
                    }

                    if diffs > 1 {
                        smudge_reflect = false;
                    }
                }
            }

            // loop ended if reflecct is still true we are reflecting
            if reflect && diffs == 0 {
                reflects_at = i + 1;
            }
            if smudge_reflect && i + 1 != reflects_at {
                smudged_reflection = i + 1;
            }
        }
    }

    (reflects_at, smudged_reflection)
}

fn get_reflection_indexes(block: &str) -> (usize, usize, usize, usize) {
    // horizontal
    let lines: Vec<&str> = block.lines().collect();
    let horizontal = reflection_index(&lines);

    let mut cols: Vec<String> = vec![];
    for line in block.lines() {
        for (idx, col) in line.chars().enumerate() {
            if cols.get(idx).is_none() {
                cols.push("".to_string());
            }

            cols[idx] = format!("{}{}", cols[idx], col);
        }
    }
    let vertical = reflection_index(&cols);

    (vertical.0, horizontal.0, vertical.1, horizontal.1)
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
