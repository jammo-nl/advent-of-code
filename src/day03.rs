use array_tool::vec::Intersect;

pub fn part1(input: String) -> String {
    let backpacks = parse_backpacks(input);
    format!(
        "{}",
        backpacks.get_score_for_items(backpacks.get_double_items())
    )
}

pub fn part2(input: String) -> String {
    let backpacks = parse_backpacks(input);

    format!(
        "{}",
        backpacks.get_score_for_items(backpacks.get_badge_items())
    )
}

struct BackPacks {
    items: Vec<char>,
    backpacks: Vec<Vec<Vec<char>>>,
}
impl BackPacks {
    fn new() -> Self {
        BackPacks {
            items: "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"
                .chars()
                .collect(),
            backpacks: vec![],
        }
    }

    fn add(&mut self, data: &str) {
        // all backpacks have 2 compartments
        let compartment_size = data.len() / 2;
        let mut compartments = vec![];
        let mut compartment = vec![];
        for (idx, item) in data.chars().enumerate() {
            if idx == compartment_size {
                // half way, add the next compartment
                compartments.push(compartment);
                compartment = vec![]
            }
            compartment.push(item);
        }
        compartments.push(compartment);
        self.backpacks.push(compartments);
    }

    fn get_double_items(&self) -> Vec<Vec<char>> {
        let mut result = vec![];
        for backpack in &self.backpacks {
            let mut my_backpack = backpack.clone();
            let mut items = my_backpack.pop().unwrap();

            for next in my_backpack {
                items = items.intersect(next);
            }
            result.push(items);
        }
        result
    }

    fn get_badge_items(&self) -> Vec<Vec<char>> {
        let mut items = vec![];
        let mut my_badge_items = vec![];
        for (idx, backpack) in self.backpacks.iter().enumerate() {
            let mut data = vec![];
            for items in backpack {
                data.append(&mut items.clone());
            }
            if idx % 3 == 0 {
                if idx != 0 {
                    items.push(my_badge_items.clone());
                }
                my_badge_items = data.clone();
            }
            my_badge_items = my_badge_items.intersect(data);
        }

        items.push(my_badge_items.clone());

        items
    }

    fn get_score_for_items(&self, backpacks: Vec<Vec<char>>) -> u64 {
        let mut total = 0u64;
        for backpack in backpacks {
            for item in backpack {
                let new: u64 = (self.items.iter().position(|&x| x == item).unwrap() + 1) as u64;
                total += new;
            }
        }
        total
    }
}

fn parse_backpacks(input: String) -> BackPacks {
    let mut backpacks = BackPacks::new();
    for line in input.lines() {
        backpacks.add(line);
    }
    backpacks
}
