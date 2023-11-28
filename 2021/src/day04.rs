use crate::tools::read_lines;

pub fn run_all(input: &str) -> String {
    let part1 = part1(read_lines(input));
    let part2 = part2(read_lines(input));

    format!("
Result part 1: {}
Result part 2: {}
    ", part1, part2)
}

#[derive(Debug)]
struct BingoCard {
    numbers: Vec<u32>,
    set_numbers: Vec<u32>,
    has_won: bool,
}
impl BingoCard {
    fn new() -> Self {
        Self { numbers: vec!(), set_numbers: vec!(), has_won: false }
    }
    fn add_number(&mut self, number: u32) -> bool {
        // add a number, return true if this is a winning number for this card
        self.set_numbers.push(number);

        let mut winner = false;

        for loc in 0..self.numbers.len() {
            if self.numbers[loc] == number {
                // added number, check from here..
                //get the column and row for this item
                let col = loc % 5;
                let row = loc / 5;

                // check a row
                let mut won_row = true;
                for c1 in 0..5 {
                    if !self.set_numbers.contains(&self.numbers[(row*5)+c1]) {
                        won_row = false;
                        break;
                    }
                }
                // check a column
                let mut won_col = true;
                for c1 in 0..5 {
                    if !self.set_numbers.contains(&self.numbers[col+(c1*5)]) {
                        won_col = false;
                        break;
                    }
                }

                winner = won_row || won_col;
            }
        }

        self.has_won = winner;
        winner
    }
}

fn build_cards(input: &[String], all_numbers: &mut Vec<u32>, cards: &mut Vec<BingoCard>) {
    let mut numbers: Vec<u32> = input[0].split(',').map(|i| i.parse::<u32>().unwrap()).collect();
    all_numbers.append(&mut numbers);
    all_numbers.reverse();

    // loop over the file, filling in all cards
    let mut card: BingoCard = BingoCard::new();
    for line in input.iter().skip(2) {
        if line.is_empty() {
            cards.push(card);
            card = BingoCard::new();
            continue;
        }

        // parse the line and add all numbers.. to this bingocard..
        let mut line_numbers: Vec<u32> = line.trim().replace("  ", " ").split(' ').map(|i| i.parse::<u32>().unwrap()).collect();
        card.numbers.append(&mut line_numbers);
    }
    cards.push(card);
}

pub fn part1(input: Vec<String>) -> u32 {
    let mut all_numbers: Vec<u32> = vec!();
    let mut cards: Vec<BingoCard> = vec!();
    let mut winning_idx: usize = 0;

    // create all boards from the file
    build_cards(&input, &mut all_numbers, &mut cards);

    // play the game and return the first winning board
    let mut winner = false;
    while !winner {
        let num = all_numbers.pop().unwrap();
        for (card_idx, card) in cards.iter_mut().enumerate() {
            winner = card.add_number(num);
            if winner {
                winning_idx = card_idx;
                break;
            }
        }
    }
    let sum: u32 = cards[winning_idx].numbers.iter().map(|item| {
        if cards[winning_idx].set_numbers.contains(item) {
            return 0;
        }
        *item
    }).sum();
    
    sum * cards[winning_idx].set_numbers.last().unwrap()
}
pub fn part2(input: Vec<String>) -> u32 {
    let mut all_numbers: Vec<u32> = vec!();
    let mut cards: Vec<BingoCard> = vec!();

    // create all boards from the file
    build_cards(&input, &mut all_numbers, &mut cards);

    // play the game until we have 1 board left
    'outer: loop {
        let num = all_numbers.pop().unwrap();
        let mut card_idx = 0;
        while card_idx < cards.len() {
            if cards[card_idx].add_number(num) {
                if cards.len() > 1 {
                    cards.remove(card_idx);
                    if card_idx > 0{
                        card_idx -= 1;
                    }
                    continue;
                }
                if cards.len() == 1 {
                    break 'outer;
                }
            }
            card_idx += 1;
        }
    }

    let sum: u32 = cards[0].numbers.iter().map(|item| {
        if cards[0].set_numbers.contains(item) {
            return 0;
        }
        *item
    }).sum();
    
    sum * cards[0].set_numbers.last().unwrap()
}
