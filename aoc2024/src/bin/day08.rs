use itertools::Itertools;
use std::collections::HashMap;
use std::ops::{Add, Sub};

#[derive(Debug, Clone)]
struct Vec2 {
    x: isize,
    y: isize,
}
impl Vec2 {
    fn from_location(width: usize, location: usize) -> Self {
        let x = (location % width) as isize;
        let y = (location / width) as isize;

        Self { x, y }
    }
    fn as_location(self, width: usize, height: usize) -> Option<usize> {
        if self.x < 0 || self.x as usize >= width || self.y < 0 || self.y as usize >= height {
            None
        } else {
            Some((self.y as usize * width) + self.x as usize)
        }
    }
}
impl Sub for Vec2 {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}
impl Add for Vec2 {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

// item can be an antenna or emtpty
// each antenna or empty field can be an antinode
#[derive(Clone, Debug, PartialEq, Eq)]
enum Item {
    Antenna(char, bool),
    None(bool),
}

#[aoc2024::main(08)]
fn solution(input: &str) -> (usize, usize) {
    // populate the grid
    let width = input.lines().next().unwrap().len();
    let height = input.lines().collect::<Vec<_>>().len();

    let mut grid: Vec<Item> = input
        .lines()
        .flat_map(|i| {
            i.chars()
                .map(|j| match j {
                    '.' => Item::None(false),
                    _ => Item::Antenna(j, false),
                })
                .collect::<Vec<Item>>()
        })
        .collect();

    // get all item field and their locations
    let mut antenna_locations: HashMap<char, Vec<usize>> = HashMap::new();
    grid.iter()
        .enumerate()
        .filter(|(_, item)| {
            if let Item::Antenna(_, _) = item {
                true
            } else {
                false
            }
        })
        .for_each(|(loc, item)| {
            if let Item::Antenna(frequency, _) = item {
                let locations = antenna_locations.entry(*frequency).or_insert(vec![]);
                locations.push(loc);
            }
        });

    // for each frequency look at all distances and get the x/y for the anti node
    let mut grid_copy = grid.clone();
    for (_, locations) in &antenna_locations {
        locations.iter().combinations(2).for_each(|data| {
            let start = Vec2::from_location(width, *data[0]);
            let next = Vec2::from_location(width, *data[1]);
            let diff = next.clone() - start.clone();
            let start_loc = (start.clone() - diff.clone()).as_location(width, height);
            let next_loc = (next.clone() + diff.clone()).as_location(width, height);

            if let Some(loc) = start_loc {
                match &mut grid[loc] {
                    Item::Antenna(_, res) => *res = true,
                    Item::None(res) => *res = true,
                }
            };
            if let Some(loc) = next_loc {
                match &mut grid[loc] {
                    Item::Antenna(_, res) => *res = true,
                    Item::None(res) => *res = true,
                }
            };
        });
    }

    for (_, locations) in &antenna_locations {
        locations.iter().combinations(2).for_each(|data| {
            let mut start = Vec2::from_location(width, *data[0]);
            let mut next = Vec2::from_location(width, *data[1]);
            let diff = next.clone() - start.clone();

            // 2 inline so the antennas are also anti
            match &mut grid_copy[*data[0]] {
                Item::Antenna(_, res) => *res = true,
                Item::None(res) => *res = true,
            }
            match &mut grid_copy[*data[1]] {
                Item::Antenna(_, res) => *res = true,
                Item::None(res) => *res = true,
            }

            // get all nodes for start
            loop {
                let start_loc = (start.clone() - diff.clone()).as_location(width, height);
                match start_loc {
                    Some(start_loc) => {
                        start = start - diff.clone();
                        match &mut grid_copy[start_loc] {
                            Item::Antenna(_, res) => *res = true,
                            Item::None(res) => *res = true,
                        }
                    }
                    None => break,
                }
            }
            loop {
                let next_loc = (next.clone() + diff.clone()).as_location(width, height);
                match next_loc {
                    Some(next_loc) => {
                        next = next + diff.clone();
                        match &mut grid_copy[next_loc] {
                            Item::Antenna(_, res) => *res = true,
                            Item::None(res) => *res = true,
                        }
                    }
                    None => break,
                }
            }
        });
    }

    let total_1 = grid
        .iter()
        .filter(|i| match i {
            Item::Antenna(_, anti) => *anti,
            Item::None(anti) => *anti,
        })
        .collect::<Vec<_>>()
        .len();

    let total_2 = grid_copy
        .iter()
        .filter(|i| match i {
            Item::Antenna(_, anti) => *anti,
            Item::None(anti) => *anti,
        })
        .collect::<Vec<_>>()
        .len();

    for (num, item) in grid_copy.iter().enumerate() {
        if num % width == 0 {
            println!();
        }

        match item {
            Item::Antenna(freq, _) => print!("{}", freq),
            Item::None(ant) => {
                if *ant {
                    print!("#");
                } else {
                    print!(".")
                }
            }
        }
    }
    println!();
    (total_1, total_2)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r#"............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............"#;

    #[test]
    fn test_solution1() -> Result<(), String> {
        assert_eq!(solution(TEST_INPUT.trim()).0, 14);
        Ok(())
    }

    #[test]
    fn test_solution2() -> Result<(), String> {
        assert_eq!(solution(TEST_INPUT.trim()).1, 34);
        Ok(())
    }
}
