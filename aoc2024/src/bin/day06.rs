#[derive(Clone, Debug, PartialEq, Eq)]
enum Item {
    Space(Vec<isize>),
    Wall,
}

fn step_trough(
    pos: &mut isize,
    current_dir: &mut u8,
    directions: &Vec<isize>,
    grid: &mut Vec<Item>,
    width: isize,
) -> bool {
    let start_x = *pos % width;
    let start_y = *pos / width;
    let test_pos = *pos + directions[*current_dir as usize];

    let end_x = test_pos % width;
    let end_y = test_pos / width;

    if (end_x != start_x && end_y != start_y) || test_pos < 0 || test_pos >= grid.len() as isize {
        println!("out of bound, we are done..");
        return false;
    }

    let total_len = grid.len();
    match &mut grid[test_pos as usize] {
        Item::Space(count) => {
            // update our position
            *pos = test_pos;
            count.push(*pos);
        }
        Item::Wall => *current_dir = (*current_dir + 1) % directions.len() as u8,
    }
    true
}

#[aoc2024::main(06)]
fn solution(input: &str) -> (usize, usize) {
    let mut current_dir = 0;

    // populate the grid
    let width = input.lines().next().unwrap().len() as isize;
    let directions: Vec<isize> = vec![-1 * width, 1, 1 * width, -1];

    let mut grid: Vec<Item> = input
        .lines()
        .enumerate()
        .flat_map(|(c, i)| {
            i.chars()
                .enumerate()
                .map(|(c2, j)| match j {
                    '.' => Item::Space(vec![]),
                    '^' => Item::Space(vec![(c + c2) as isize]),
                    '#' => Item::Wall,
                    _ => panic!("no such item"),
                })
                .collect::<Vec<Item>>()
        })
        .collect();

    // start pos is the ony visited item
    let mut pos = *grid
        .iter()
        .enumerate()
        .filter(|(_, item)| {
            if let Item::Space(items) = item {
                items.len() > 0
            } else {
                false
            }
        })
        .map(|(idx, _)| idx)
        .collect::<Vec<usize>>()
        .first()
        .unwrap() as isize;

    while step_trough(&mut pos, &mut current_dir, &directions, &mut grid, width) {
        //
    }

    let total_1 = grid
        .iter()
        .filter(|item| {
            if let Item::Space(items) = item {
                items.len() > 0
            } else {
                false
            }
        })
        .collect::<Vec<_>>()
        .len();

    (total_1, 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r#"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."#;

    #[test]
    fn test_solution1() -> Result<(), String> {
        assert_eq!(solution(TEST_INPUT.trim()).0, 41);
        Ok(())
    }

    #[test]
    fn test_solution2() -> Result<(), String> {
        assert_eq!(solution(TEST_INPUT.trim()).1, 6);
        Ok(())
    }
}
