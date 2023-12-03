use std::collections::HashMap;

struct Game {
    num: u8,
    valid: bool,
    power: usize,
}
impl Game {
    fn new(input: &str, max_items: &HashMap<&str, u8>) -> Self {
        let gid: Vec<&str> = input.split(":").collect();
        let num = gid
            .first()
            .unwrap()
            .replace("Game", "")
            .trim()
            .parse::<u8>()
            .unwrap();
        let reveals = gid.last().unwrap().split(";");
        let mut min_items: HashMap<&str, u8> = [("red", 0), ("green", 0), ("blue", 0)].into();

        let mut valid = true;

        for reveal in reveals {
            let draws = reveal.split(",");
            for draw in draws {
                let item: Vec<&str> = draw.trim().split(" ").collect();
                let amount = item.first().unwrap().parse::<u8>().unwrap();

                if max_items.get(*item.last().unwrap()).unwrap() < &amount {
                    valid = false
                }
                match min_items.get_mut(*item.last().unwrap()) {
                    Some(total) => *total = std::cmp::max(*total, amount),
                    None => (),
                }
            }
        }
        let power = min_items.into_values().map(|i| i as usize).product();
        Self { num, valid, power }
    }
    fn is_valid(&self) -> bool {
        self.valid
    }
}

#[aoc2023::main(02)]
fn solution(input: &str) -> (usize, usize) {
    let max_items: HashMap<&str, u8> = [("red", 12), ("green", 13), ("blue", 14)].into();

    let mut total_p1: usize = 0;
    let mut total_p2: usize = 0;
    for line in input.lines() {
        let game = Game::new(line.trim(), &max_items);
        if game.is_valid() {
            total_p1 += game.num as usize;
        }
        total_p2 += game.power;
    }

    (total_p1, total_p2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution1() -> Result<(), String> {
        let test_input = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#;

        assert_eq!(solution(test_input.trim()).0, 8);
        Ok(())
    }

    #[test]
    fn test_solution2() -> Result<(), String> {
        let test_input = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#;

        assert_eq!(solution(test_input.trim()).1, 2286);
        Ok(())
    }
}
