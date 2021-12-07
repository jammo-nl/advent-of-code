use crate::tools::read_lines;
use std::collections::HashMap;


pub fn run_all(input: &str) -> String {
    let part1 = part1(read_lines(input));
    let part2 = part2(read_lines(input));

    format!("
Result part 1: {}
Result part 2: {}
    ", part1, part2)
}

struct Grid {
    points: HashMap<String,u32>
}
impl Grid {
    fn new() -> Self {
        Self { points: HashMap::new() }
    }

    fn get_line_coordinates(&self, line: &str) -> (u32,u32,u32,u32) {
        let (xy1, xy2) = line.split_once(" -> ").unwrap();
        let (x1,y1) = xy1.split_once(",").unwrap();
        let (x2,y2) = xy2.split_once(",").unwrap();
        
        // return the lines always from top to bottom, left to right
        (x1.parse::<u32>().unwrap(), x2.parse::<u32>().unwrap(),
        y1.parse::<u32>().unwrap(), y2.parse::<u32>().unwrap())
    }

    fn iterator(&self, first: u32, last: u32) -> Vec<u32> {
        if first <= last {
            (first..=last).collect::<Vec<u32>>()
        }
        else {
            (last..=first).rev().collect::<Vec<u32>>()
        }
    }

    fn add_line_points(&mut self, line: &str) {
        // update all points in the given line, only if the line is straight!
        let (x1, x2, y1, y2) = self.get_line_coordinates(line);

        if x1 == x2 {
            for c in self.iterator(y1,y2) {
                let coordinate = format!("{}.{}", x1, c);
                let mut val = *self.points.get(&coordinate).unwrap_or(&0);
                val += 1;
                self.points.insert(coordinate, val);
            }
        }
        if y1 == y2 {
            for c in self.iterator(x1,x2) {
                let coordinate = format!("{}.{}", c, y1);
                let mut val = *self.points.get(&coordinate).unwrap_or(&0);
                val += 1;
                self.points.insert(coordinate, val);
            }
        }
    }

    fn c(&self, counter: u32, c1: u32, c2: u32) -> u32 {
        //println!("{}-{}-{}", c1,c2,counter);
        if c1 > c2 {
            c1-counter
        } else {
            c1+counter
        }
    }

    fn add_diagonal_points(&mut self, line: &str) {
        // update all points in the given line, only if the line is diagonal
        // mind the fact that we only allow 45 degree lines!
        let (x1, x2, y1, y2) = self.get_line_coordinates(line);

        if i32::abs(y2 as i32 - y1 as i32) == i32::abs(x2  as i32 - x1  as i32) {
            for c in 0..=i32::abs(y2 as i32 - y1 as i32) as u32 {
                let coordinate = format!("{}.{}", self.c(c, x1, x2), self.c(c, y1, y2));
                let mut val = *self.points.get(&coordinate).unwrap_or(&0);
                val += 1;
                self.points.insert(coordinate, val);
            }
        }
    }
}

pub fn part1(input: Vec<String>) -> u32 {
    let mut vents = Grid::new();
    for line in input {
        vents.add_line_points(&line);
    }

    let mut result = vents.points.clone();
    result.retain(|_, value| *value >= 2);
    result.len() as u32
}
pub fn part2(input: Vec<String>) -> u32 {
    let mut vents = Grid::new();
    for line in input {
        vents.add_line_points(&line);
        vents.add_diagonal_points(&line);
    }

    let mut result = vents.points.clone();
    result.retain(|_, value| *value >= 2);

    result.len() as u32
}

#[cfg(test)]
mod tests {
    static INPUT: &str = "./inputs/day05_test.txt";
    
    use super::*;
    #[test]
    fn test_part1() {
        let output = part1(read_lines(INPUT));
        assert_eq!(output, 5)
    }

    #[test]
    fn test_part2() {
        let output = part2(read_lines(INPUT));
        assert_eq!(output, 12)
    }
}
