#[aoc2024::main(04)]
fn solution(input: &str) -> (usize, usize) {
    let mut total_1 = 0;
    let mut total_2 = 0;

    let width = input.lines().next().unwrap().len();
    let height = input.lines().count();
    let grid: Vec<char> = input
        .lines()
        .flat_map(|i| i.chars().collect::<Vec<char>>())
        .collect();

    // find a x, and from there search for xmas
    for (index, item) in grid.iter().enumerate() {
        if item == &'X' {
            total_1 += find_xmas(&grid, width, height, index);
        }
    }

    // find a x, and from there search for xmas
    for (index, item) in grid.iter().enumerate() {
        if item == &'A' {
            total_2 += find_x_mas(&grid, width, height, index);
        }
    }

    (total_1, total_2)
}

fn find_xmas(grid: &[char], width: usize, height: usize, start: usize) -> usize {
    // get x/y coordinate in grid
    let mut found = 0;
    let x = start % width;
    let y = start / width;

    let up = y >= 3;
    let left = x >= 3;
    let right = x + 3 < width;
    let down = y + 3 < height;

    // create string
    if up {
        let step = width;
        let range = (start - (step * 3)..=start).step_by(step);
        let mut data = String::new();
        for x in range {
            data = format!("{}{}", grid[x], data);
        }

        if data == "XMAS" {
            found += 1;
        }
    }
    if left {
        let step = 1;
        let range = (start - (step * 3)..=start).step_by(step);
        let mut data = String::new();
        for x in range {
            data = format!("{}{}", grid[x], data);
        }

        if data == "XMAS" {
            found += 1;
        }
    }
    if left && up {
        let step = width + 1;
        let range = (start - (step * 3)..=start).step_by(step);
        let mut data = String::new();
        for x in range {
            data = format!("{}{}", grid[x], data);
        }

        if data == "XMAS" {
            found += 1;
        }
    }
    if right && up {
        let step = width - 1;
        let range = (start - (step * 3)..=start).step_by(step);
        let mut data = String::new();
        for x in range {
            data = format!("{}{}", grid[x], data);
        }

        if data == "XMAS" {
            found += 1;
        }
    }
    if down {
        let step = width;
        let range = (start..=(start + (step * 3))).step_by(step);
        let mut data = String::new();
        for x in range {
            data = format!("{}{}", data, grid[x]);
        }

        if data == "XMAS" {
            found += 1;
        }
    }
    if right {
        let step = 1;
        let range = (start..=(start + (step * 3))).step_by(step);
        let mut data = String::new();
        for x in range {
            data = format!("{}{}", data, grid[x]);
        }

        if data == "XMAS" {
            found += 1;
        }
    }
    if down && left {
        let step = width - 1;
        let range = (start..=(start + (step * 3))).step_by(step);
        let mut data = String::new();
        for x in range {
            data = format!("{}{}", data, grid[x]);
        }

        if data == "XMAS" {
            found += 1;
        }
    }
    if down && right {
        let step = width + 1;
        let range = (start..=(start + (step * 3))).step_by(step);
        let mut data = String::new();
        for x in range {
            data = format!("{}{}", data, grid[x]);
        }

        if data == "XMAS" {
            found += 1;
        }
    }

    found
}

fn find_x_mas(grid: &[char], width: usize, height: usize, start: usize) -> usize {
    // get x/y coordinate in grid
    let found = 0;
    let x = start % width;
    let y = start / width;

    let up = y >= 1;
    let left = x >= 1;
    let right = x + 1 < width;
    let down = y + 1 < height;

    if !up || !down || !right || !left {
        return 0;
    }

    // from top left to bottom right
    let char1 = start - (width + 1);
    let char3 = start + (width + 1);
    let char4 = start - (width - 1);
    let char6 = start + (width - 1);

    let str1 = format!("{}{}{}", grid[char1], grid[start], grid[char3]);
    let str2 = format!("{}{}{}", grid[char4], grid[start], grid[char6]);

    if (&str1 == "MAS" || str1 == "SAM") && (&str2 == "MAS" || str2 == "SAM") {
        return 1;
    }

    found
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r#"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"#;

    #[test]
    fn test_solution1() -> Result<(), String> {
        assert_eq!(solution(TEST_INPUT.trim()).0, 18);
        Ok(())
    }

    #[test]
    fn test_solution2() -> Result<(), String> {
        assert_eq!(solution(TEST_INPUT.trim()).1, 9);
        Ok(())
    }
}
