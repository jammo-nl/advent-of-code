use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;

lazy_static! {
    static ref SCRATCH_CARD: Regex =
        Regex::new(r"Card +(\d+): *([\d ]+)+ ?\| *([\d ]+)+ ?").unwrap();
}
#[derive(Debug, Clone)]
struct Card {
    number: usize,
    winning_numbers: usize,
}
impl Card {
    fn winning_cards(winning: Vec<usize>, numbers: Vec<usize>) -> usize {
        winning
            .iter()
            .filter(|i| numbers.contains(&i))
            .collect::<Vec<_>>()
            .len()
    }
    fn added_cards(&self, cards: &HashMap<usize, Card>) -> usize {
        let mut amount = self.winning_numbers;
        if amount > 0 {
            for num in self.number + 1..=self.number + amount {
                if let Some(sub_card) = cards.get(&num) {
                    amount += sub_card.added_cards(cards);
                }
            }
        }

        amount
    }
}

#[aoc2023::main(04)]
fn solution(input: &str) -> (usize, usize) {
    let mut total_p1 = 0;
    let mut total_p2 = 0;
    let mut cards: HashMap<usize, Card> = HashMap::new();
    for line in input.lines() {
        let data = SCRATCH_CARD.captures(line).unwrap();

        let winning = data
            .get(2)
            .unwrap()
            .as_str()
            .split(' ')
            .filter_map(|i| i.parse().ok())
            .collect();
        let numbers = data
            .get(3)
            .unwrap()
            .as_str()
            .split(' ')
            .filter_map(|i| i.parse().ok())
            .collect();

        let card = Card {
            winning_numbers: Card::winning_cards(winning, numbers),
            number: data.get(1).unwrap().as_str().parse().unwrap(),
        };

        // get price winning numbers
        let price = card.winning_numbers;
        // get score for this card
        let score = if price > 0 { 1 << price - 1 } else { 0 };
        total_p1 += score;

        cards.insert(card.number, card.clone());
    }

    for (_, card) in &cards {
        total_p2 += card.added_cards(&cards) + 1;
    }

    (total_p1, total_p2)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r#"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
"#;

    #[test]
    fn test_solution1() -> Result<(), String> {
        assert_eq!(solution(TEST_INPUT.trim()).0, 13);
        Ok(())
    }

    #[test]
    fn test_solution2() -> Result<(), String> {
        assert_eq!(solution(TEST_INPUT.trim()).1, 30);
        Ok(())
    }
}
