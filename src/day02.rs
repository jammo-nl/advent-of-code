use crate::tools::read_lines;

static INPUT: &str = "./inputs/day02.txt";

pub fn run_all() -> String {
    let part1 = part1(read_lines(INPUT));
    let part2 = part2(read_lines(INPUT));

    format!("
Result part 1: {}
Result part 2: {}
    ", part1, part2)
}

struct Postition {
    fw: i32,
    depth: i32
}
impl Postition {
    fn new() -> Self {
        Self{fw:0,depth:0}
    }
    fn forward(&mut self, distance: i32) {
        self.fw += distance;
    }
    fn up(&mut self, distance: i32) {
        self.depth -= distance;
    }
    fn down(&mut self, distance: i32) {
        self.depth += distance;
    }
    fn mutiplied(&self) -> i32 {
        self.fw * self.depth
    }
}

struct AimPostition {
    fw: i32,
    aim: i32,
    depth: i32
}
impl AimPostition {
    fn new() -> Self {
        Self{fw:0,aim:0,depth:0}
    }
    fn forward(&mut self, distance: i32) {
        self.fw += distance;
        self.depth += distance*self.aim;
    }
    fn up(&mut self, distance: i32) {
        self.aim -= distance;
    }
    fn down(&mut self, distance: i32) {
        self.aim += distance;
    }
    fn mutiplied(&self) -> i32 {
        self.fw * self.depth
    }
}

pub fn part1(input: Vec<String>) -> i32 {
    let mut pos = Postition::new();

    for line in input {
        let line_parts = line.split(" ").collect::<Vec<&str>>();
        match line_parts[0] {
            "forward" => pos.forward(line_parts[1].parse::<i32>().unwrap()),
            "down" => pos.down(line_parts[1].parse::<i32>().unwrap()),
            "up" => pos.up(line_parts[1].parse::<i32>().unwrap()),
            _ => panic!("Wrong direction")
        }
    };

    pos.mutiplied()
}
pub fn part2(input: Vec<String>) -> i32 {
    let mut pos = AimPostition::new();

    for line in input {
        let line_parts = line.split(" ").collect::<Vec<&str>>();
        match line_parts[0] {
            "forward" => pos.forward(line_parts[1].parse::<i32>().unwrap()),
            "down" => pos.down(line_parts[1].parse::<i32>().unwrap()),
            "up" => pos.up(line_parts[1].parse::<i32>().unwrap()),
            _ => panic!("Wrong direction")
        }
    };

    pos.mutiplied()
}

#[cfg(test)]
mod tests {
    static INPUT: &str = "./inputs/day02_test.txt";
    
    use super::*;
    #[test]
    fn test_part1() {
        let output = part1(read_lines(INPUT));
        assert_eq!(output, 150)
    }

    #[test]
    fn test_part2() {
        let output = part2(read_lines(INPUT));
        assert_eq!(output, 900)
    }
}
