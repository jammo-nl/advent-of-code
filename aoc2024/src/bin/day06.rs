use std::collections::HashSet;

#[derive(Clone, Debug, PartialEq, Eq)]
enum Item {
    Space(bool),
    Wall,
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum ReturnType {
    Working,
    OutOfBound,
    Loop,
}

fn step_trough(
    start_pos: isize,
    _prev_pos: isize,
    pos: &mut isize,
    current_dir: &mut u8,
    directions: &Vec<isize>,
    grid: &mut Vec<Item>,
    width: isize,
    visits: &mut HashSet<(isize, u8)>,
    tested_walls: &mut HashSet<isize>,
    loop_test: bool,
    loop_count: &mut usize,
) -> ReturnType {
    let start_x = *pos % width;
    let start_y = *pos / width;
    let prev_pos = *pos;
    let test_pos = *pos + directions[*current_dir as usize];

    let end_x = test_pos % width;
    let end_y = test_pos / width;

    if (end_x != start_x && end_y != start_y) || test_pos < 0 || test_pos >= grid.len() as isize {
        return ReturnType::OutOfBound;
    }

    // collect info for a test round
    match &mut grid[test_pos as usize] {
        Item::Space(res) => {
            // check if we are looping
            if visits.contains(&(test_pos, *current_dir)) {
                *loop_count += 1;
                return ReturnType::Loop;
            }

            *pos = test_pos;
            // keep track of visited locations for the main loop only
            if loop_test {
                *res = true;
            }
            visits.insert((*pos, *current_dir));
        }
        Item::Wall => *current_dir = (*current_dir + 1) % directions.len() as u8,
    }

    if !(loop_test) {
        return ReturnType::Working;
    }
    // check if the next value is blocking, a wall or our start location, if so we can not build an obstacle
    let obs_pos = prev_pos + directions[*current_dir as usize];
    let obs_x = obs_pos % width;
    let obs_y = obs_pos / width;

    if (obs_x != end_x && obs_y != end_y) || obs_pos < 0 || obs_pos > grid.len() as isize {
        //println!("Done, not placing an obstacle outside of the world");
        return ReturnType::Working;
    }
    if obs_pos == start_pos {
        return ReturnType::Working;
    }
    if let Item::Wall = grid[obs_pos as usize] {
        //println!("Done, not placing an obstacle on a wall");
        return ReturnType::Working;
    }
    if tested_walls.contains(&obs_pos) {
        return ReturnType::Working;
    }

    // clone the grid and add a wall for this loop
    let mut tst_grid = grid.clone();
    tst_grid[obs_pos as usize] = Item::Wall;
    tested_walls.insert(obs_pos);

    // track visits
    let mut sub_visits = visits.clone();
    let mut alt_pos = prev_pos;
    let mut alt_dir = (*current_dir + 1) % directions.len() as u8;

    loop {
        let result = step_trough(
            start_pos,
            prev_pos,
            &mut alt_pos,
            &mut alt_dir,
            &directions,
            &mut tst_grid,
            width,
            &mut sub_visits,
            tested_walls,
            false,
            loop_count,
        );

        if result == ReturnType::OutOfBound {
            break;
        } else if result == ReturnType::Loop {
            return ReturnType::Loop;
        } else {
        }
    }

    ReturnType::Working
}

#[aoc2024::main(06)]
fn solution(input: &str) -> (usize, usize) {
    let mut current_dir = 0;

    // populate the grid
    let width = input.lines().next().unwrap().len() as isize;
    let directions: Vec<isize> = vec![-1 * width, 1, 1 * width, -1];
    let mut visits = HashSet::new();
    let mut tested_walls = HashSet::new();

    let mut grid: Vec<Item> = input
        .lines()
        .flat_map(|i| {
            i.chars()
                .map(|j| match j {
                    '.' => Item::Space(false),
                    '^' => Item::Space(true),
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
        .filter(|(_, item)| **item == Item::Space(true))
        .map(|(idx, _)| idx)
        .collect::<Vec<usize>>()
        .first()
        .unwrap() as isize;

    let mut loop_count = 0;

    let start_pos = pos;
    while step_trough(
        start_pos,
        start_pos,
        &mut pos,
        &mut current_dir,
        &directions,
        &mut grid,
        width,
        &mut visits,
        &mut tested_walls,
        true,
        &mut loop_count,
    ) != ReturnType::OutOfBound
    {}

    let total_1 = grid
        .iter()
        .filter(|item| **item == Item::Space(true))
        .collect::<Vec<_>>()
        .len();

    (total_1, loop_count)
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
