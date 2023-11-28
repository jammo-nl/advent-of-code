use crate::tools::read_lines;

pub fn run_all(input: &str) -> String {
    let part1 = part1(read_lines(input));
    let part2 = part2(read_lines(input));

    format!("
Result part 1: {}
Result part 2: {}
    ", part1, part2)
}

struct Postition {
    fw: i32,
    depth: i32,
    aimed_depth: i32
}
impl Postition {
    fn new() -> Self {
        Self{fw:0,depth:0,aimed_depth:0}
    }
    fn forward(&mut self, distance: i32) {
        self.fw += distance;
        self.aimed_depth += distance*self.depth;
    }
    fn up(&mut self, distance: i32) {
        self.depth -= distance;
    }
    fn down(&mut self, distance: i32) {
        self.depth += distance;
    }
    fn pos(&self) -> i32 {
        self.fw * self.depth
    }
    fn aimed_pos(&self) -> i32 {
        self.fw * self.aimed_depth
    }
}

pub fn part1(input: Vec<String>) -> i32 {
    let mut pos = Postition::new();

    for line in input {
        let line_parts = line.split(' ').collect::<Vec<&str>>();
        match line_parts[0] {
            "forward" => pos.forward(line_parts[1].parse::<i32>().unwrap()),
            "down" => pos.down(line_parts[1].parse::<i32>().unwrap()),
            "up" => pos.up(line_parts[1].parse::<i32>().unwrap()),
            _ => panic!("Wrong direction")
        }
    };

    pos.pos()
}
pub fn part2(input: Vec<String>) -> i32 {
    let mut pos = Postition::new();

    for line in input {
        let line_parts = line.split(' ').collect::<Vec<&str>>();
        match line_parts[0] {
            "forward" => pos.forward(line_parts[1].parse::<i32>().unwrap()),
            "down" => pos.down(line_parts[1].parse::<i32>().unwrap()),
            "up" => pos.up(line_parts[1].parse::<i32>().unwrap()),
            _ => panic!("Wrong direction")
        }
    };

    pos.aimed_pos()
}
