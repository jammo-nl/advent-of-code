use std::collections::HashSet;

fn trail_heads(grid: &Vec<u8>, width: usize, pos: usize, endpoints: &mut HashSet<usize>) -> usize {
    let curr_height = grid[pos];
    if curr_height == 9 {
        endpoints.insert(pos);
        return 1;
    }

    let mut score = 0;
    let next_height = curr_height + 1;
    let curr_x = pos % width;

    if curr_x + 1 < width && grid[pos + 1] == next_height {
        score += trail_heads(grid, width, pos + 1, endpoints);
    }
    if curr_x > 0 && grid[pos - 1] == next_height {
        score += trail_heads(grid, width, pos - 1, endpoints);
    }
    if pos + width < grid.len() && grid[pos + width] == next_height {
        score += trail_heads(grid, width, pos + width, endpoints);
    }
    if pos >= width && grid[pos - width] == next_height {
        score += trail_heads(grid, width, pos - width, endpoints);
    }

    score
}

#[aoc2024::main(10)]
fn solution(input: &str) -> (usize, usize) {
    let width = input.lines().next().unwrap().len();

    let grid: Vec<u8> = input
        .lines()
        .flat_map(|i| i.chars().map(|j| (j as u8) - 48).collect::<Vec<u8>>())
        .collect();

    let trail_head = grid
        .iter()
        .enumerate()
        .filter(|(_, i)| **i == 0)
        .map(|(pos, _)| {
            let mut endpoints = HashSet::new();
            let _ = trail_heads(&grid, width, pos, &mut endpoints);
            endpoints.len()
        })
        .sum();

    let trail_paths = grid
        .iter()
        .enumerate()
        .filter(|(_, i)| **i == 0)
        .map(|(pos, _)| {
            let mut endpoints = HashSet::new();
            let res = trail_heads(&grid, width, pos, &mut endpoints);
            res
        })
        .sum();

    (trail_head, trail_paths)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r#"89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732"#;

    #[test]
    fn test_solution1() -> Result<(), String> {
        assert_eq!(solution(TEST_INPUT.trim()).0, 36);
        Ok(())
    }

    #[test]
    fn test_solution2() -> Result<(), String> {
        assert_eq!(solution(TEST_INPUT.trim()).1, 81);
        Ok(())
    }
}
