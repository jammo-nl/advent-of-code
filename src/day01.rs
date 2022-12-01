pub fn part1(input: String) {
    let mut elves = parse_elves(input);
    // sort elves by the total_calories and print the result
    elves.sort_by(|a, b| b.total_calories.cmp(&a.total_calories));
    println!("result: {}", elves.first().unwrap().total_calories);
}

pub fn part2(input: String) {
    let mut elves = parse_elves(input);
    // sort elves by the total_calories and sum the result of the first 3
    elves.sort_by(|a, b| b.total_calories.cmp(&a.total_calories));
    elves.truncate(3);
    let total: u64 = elves.iter().map(|x| x.total_calories).sum();
    println!("result: {}", total);
}

struct Elf {
    total_calories: u64,
}
impl Elf {
    pub fn new() -> Self {
        Self { total_calories: 0 }
    }
    pub fn insert_food(&mut self, calories: u32) {
        self.total_calories += calories as u64;
    }
}

fn parse_elves(input: String) -> Vec<Elf> {
    let mut elves: Vec<Elf> = vec![];
    let mut current_elf = Elf::new();
    for line in input.lines() {
        if line.is_empty() {
            elves.push(current_elf);
            current_elf = Elf::new();
        } else {
            let calories = line.parse::<u32>().unwrap();
            current_elf.insert_food(calories);
        }
    }
    elves.push(current_elf);
    elves
}
