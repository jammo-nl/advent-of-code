#[derive(Debug, Clone)]
struct Vec2 {
    x: usize,
    y: usize,
}
#[derive(Debug, Clone)]
struct Pipe {
    contained: bool,
    distance: Option<usize>,
    location: Vec2,
    exits: u8,
}
impl Pipe {
    fn new(input: &char, x: usize, y: usize) -> Self {
        Self {
            contained: true,
            distance: None,
            location: Vec2 { x, y },
            exits: match input {
                // bits top-right-bottom-left
                '|' => 0b1010u8,
                '-' => 0b0101u8,
                'L' => 0b1100u8,
                'J' => 0b1001u8,
                '7' => 0b0011u8,
                'F' => 0b0110u8,
                'S' => 0b1111u8,
                _ => 0b0000u8,
            },
        }
    }
    fn get_directions(&self) -> Vec<Direction> {
        let mut directions = vec![];
        if self.exits & 0b1000u8 != 0 {
            directions.push(Direction::Up)
        }
        if self.exits & 0b0100u8 != 0 {
            directions.push(Direction::Right)
        }
        if self.exits & 0b0010u8 != 0 {
            directions.push(Direction::Down)
        }
        if self.exits & 0b0001u8 != 0 {
            directions.push(Direction::Left)
        }

        directions
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Direction {
    Up = 0,
    Right,
    Down,
    Left,
    None,
}

struct Grid {
    start_item: Vec2,
    items: Vec<Vec<Pipe>>,
}
impl Grid {
    fn new() -> Self {
        Self {
            start_item: Vec2 { x: 0, y: 0 },
            items: vec![],
        }
    }
    fn add(&mut self, input: &char, x: usize, y: usize) {
        let mut p = Pipe::new(input, x, y);
        if *input == 'S' {
            p.distance = Some(0);
            self.start_item = Vec2 { x, y };
        }
        while self.items.len() < y + 1 {
            self.items.push(vec![]);
        }
        while self.items[y].len() < x + 1 {
            self.items[y].push(Pipe::new(&'.', x, y));
        }

        self.items[y][x] = p;
    }
    fn get_item_mut(&mut self, location: Vec2, from: &Direction) -> Option<&mut Pipe> {
        if let Some(item) = self.items[location.y].get_mut(location.x) {
            match from {
                Direction::Up => {
                    if item.exits & 0b0010 != 0 {
                        Some(item)
                    } else {
                        None
                    }
                }
                Direction::Right => {
                    if item.exits & 0b0001 != 0 {
                        Some(item)
                    } else {
                        None
                    }
                }
                Direction::Down => {
                    if item.exits & 0b1000 != 0 {
                        Some(item)
                    } else {
                        None
                    }
                }
                Direction::Left => {
                    if item.exits & 0b0100 != 0 {
                        Some(item)
                    } else {
                        None
                    }
                }
                Direction::None => Some(item),
            }
        } else {
            None
        }
    }
    fn get_item(&self, location: Vec2, from: &Direction) -> Option<&Pipe> {
        if let Some(item) = self.items[location.y].get(location.x) {
            match from {
                Direction::Up => {
                    if item.exits & 0b0010 != 0 {
                        Some(item)
                    } else {
                        None
                    }
                }
                Direction::Right => {
                    if item.exits & 0b0001 != 0 {
                        Some(item)
                    } else {
                        None
                    }
                }
                Direction::Down => {
                    if item.exits & 0b1000 != 0 {
                        Some(item)
                    } else {
                        None
                    }
                }
                Direction::Left => {
                    if item.exits & 0b0100 != 0 {
                        Some(item)
                    } else {
                        None
                    }
                }
                Direction::None => Some(item),
            }
        } else {
            None
        }
    }
    fn get_coordinate(&self, start: &Vec2, direction: &Direction) -> Option<Vec2> {
        match direction {
            Direction::Up => {
                if start.y == 0 {
                    None
                } else {
                    Some(Vec2 {
                        x: start.x,
                        y: start.y - 1,
                    })
                }
            }
            Direction::Right => {
                if start.x == self.items[0].len() - 1 {
                    None
                } else {
                    Some(Vec2 {
                        x: start.x + 1,
                        y: start.y,
                    })
                }
            }
            Direction::Down => {
                if start.y == self.items.len() - 1 {
                    None
                } else {
                    Some(Vec2 {
                        x: start.x,
                        y: start.y + 1,
                    })
                }
            }
            Direction::Left => {
                if start.x == 0 {
                    None
                } else {
                    Some(Vec2 {
                        x: start.x - 1,
                        y: start.y,
                    })
                }
            }
            Direction::None => None,
        }
    }
    fn max_distance(&self) -> usize {
        let mut max_distance = 0;
        for i in &self.items {
            for p in i {
                if let Some(dist) = p.distance {
                    if dist > max_distance {
                        max_distance = dist;
                    }
                }
            }
        }
        max_distance
    }
    fn get_contained(&self) -> usize {
        let mut num_contained = 0;
        for i in &self.items {
            for p in i {
                if p.contained && p.distance.is_none() {
                    num_contained += 1;
                }
            }
        }
        num_contained
    }
    fn update_contained(&mut self) {
        let width = self.items[0].len() - 1;
        let height = self.items.len() - 1;
        let mut contained = vec![];
        for line in self.items.iter() {
            for pipe in line.iter() {
                if pipe.distance.is_none() {
                    // sides are never contained
                    if pipe.location.x == 0
                        || pipe.location.y == 0
                        || pipe.location.x == width
                        || pipe.location.y == height
                    {
                        contained.push(pipe.location.clone());
                    } else {
                        // count blokking edges, even is outside, uneven is contained
                        let mut left_edges = 0;
                        let current_y = pipe.location.y;
                        let current_x = pipe.location.x;
                        for i in 0..current_x {
                            let p = self.get_item(Vec2 { x: i, y: current_y }, &Direction::None);
                            if let Some(item) = p {
                                if item.distance.is_some() {
                                    if item.exits == 0b1010u8
                                        || item.exits == 0b1001u8
                                        || item.exits == 0b1100u8
                                    {
                                        left_edges += 1;
                                    }
                                }
                            }
                        }
                        if left_edges == 0 || left_edges % 2 == 0 {
                            contained.push(pipe.location.clone());
                        }
                    }
                }
            }
        }

        for loc in contained {
            let item = self.get_item_mut(loc, &Direction::None);
            if let Some(item) = item {
                item.contained = false;
            }
        }
    }
    fn navigate(&mut self, location: Option<Vec2>, from: Direction, step: usize) {
        // update item distance
        if let Some(item) =
            self.get_item_mut(location.clone().unwrap_or(self.start_item.clone()), &from)
        {
            item.distance = Some(step);
        }

        let new_step = step + 1;
        let mut next_locations = vec![];
        {
            let item = self.get_item(location.unwrap_or(self.start_item.clone()), &from);
            if let Some(item) = item {
                for direction in item.get_directions() {
                    if let Some(coordinate) = self.get_coordinate(&item.location, &direction) {
                        if let Some(new_item) = self.get_item(coordinate.clone(), &direction) {
                            if let Some(dist) = new_item.distance {
                                if new_step >= dist {
                                    continue;
                                }
                            }

                            next_locations.push((coordinate.clone(), direction.clone()));
                        }
                    }
                }
            }
        }

        for (location, direction) in next_locations {
            self.navigate(Some(location), direction, new_step);
        }
    }
}

#[aoc2023::main(10)]
fn solution(input: &str) -> (usize, usize) {
    let mut grid = Grid::new();

    for (count, line) in input.lines().enumerate() {
        for (idx, item) in line.chars().enumerate() {
            grid.add(&item, idx, count);
        }
    }

    grid.navigate(None, Direction::None, 0);
    grid.update_contained();

    let total_p1 = grid.max_distance();
    let total_p2 = grid.get_contained();
    (total_p1, total_p2)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r#"..F7.
.FJ|.
SJ.L7
|F--J
LJ...
"#;

    const TEST_INPUT2: &str = r#"FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJIF7FJ-
L---JF-JLJIIIIFJLJJ7
|F|F-JF---7IIIL7L|7|
|FFJF7L7F-JF7IIL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L
"#;

    const TEST_INPUT3: &str = r#".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...
"#;

    #[test]
    fn test_solution1() -> Result<(), String> {
        assert_eq!(solution(TEST_INPUT.trim()).0, 8);
        Ok(())
    }

    #[test]
    fn test_solution2() -> Result<(), String> {
        assert_eq!(solution(TEST_INPUT2.trim()).1, 10);
        Ok(())
    }
    #[test]
    fn test_solution3() -> Result<(), String> {
        assert_eq!(solution(TEST_INPUT3.trim()).1, 8);
        Ok(())
    }
}
