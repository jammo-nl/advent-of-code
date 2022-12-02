pub fn part1(input: String) {
    let result = parse_code(input, false);
    println!("result: {}", result.total_score);
}

pub fn part2(input: String) {
    let result = parse_code(input, true);
    println!("result: {}", result.total_score);
}

static SCORE_ROCK: u8 = 1;
static SCORE_PAPER: u8 = 2;
static SCORE_SCISSOR: u8 = 3;

static SCORE_LOSE: u8 = 0;
static SCORE_DRAW: u8 = 3;
static SCORE_WIN: u8 = 6;

#[derive(Clone, PartialEq)]
enum RpcAction {
    Win,
    Lose,
    Draw,
}

#[derive(Clone, PartialEq)]
enum RpcItem {
    Rock,
    Paper,
    Scissor,
}

struct RockPaperScissor {
    total_score: u64,
}
impl RockPaperScissor {
    fn new() -> Self {
        RockPaperScissor { total_score: 0 }
    }

    fn decrypt_item_a(opp: &str) -> RpcItem {
        match opp {
            "A" => RpcItem::Rock,
            "B" => RpcItem::Paper,
            "C" => RpcItem::Scissor,
            _ => panic!("no such item {}", opp),
        }
    }
    fn decrypt_item_b_as_item(opp: &str) -> RpcItem {
        match opp {
            "X" => RpcItem::Rock,
            "Y" => RpcItem::Paper,
            "Z" => RpcItem::Scissor,
            _ => panic!("no such item {}", opp),
        }
    }
    fn decrypt_item_b_as_result(opp: &str) -> RpcAction {
        match opp {
            "X" => RpcAction::Lose,
            "Y" => RpcAction::Draw,
            "Z" => RpcAction::Win,
            _ => panic!("no such item {}", opp),
        }
    }

    fn get_item_score(item: &RpcItem) -> u8 {
        match item {
            RpcItem::Rock => SCORE_ROCK,
            RpcItem::Paper => SCORE_PAPER,
            RpcItem::Scissor => SCORE_SCISSOR,
        }
    }

    fn do_draw(a: &RpcItem, b: &RpcItem) -> u8 {
        if a == b {
            SCORE_DRAW
        } else if (b == &RpcItem::Rock && a == &RpcItem::Scissor)
            || (b == &RpcItem::Paper && a == &RpcItem::Rock)
            || (b == &RpcItem::Scissor && a == &RpcItem::Paper)
        {
            SCORE_WIN
        } else {
            SCORE_LOSE
        }
    }

    fn add_draw(&mut self, a: &RpcItem, b: &RpcItem) {
        let mut score = Self::do_draw(a, b);
        score += Self::get_item_score(b);
        self.total_score += score as u64
    }

    fn get_item_for_action(a: &RpcItem, b: &RpcAction) -> RpcItem {
        match b {
            RpcAction::Win => match a {
                RpcItem::Rock => RpcItem::Paper,
                RpcItem::Paper => RpcItem::Scissor,
                RpcItem::Scissor => RpcItem::Rock,
            },
            RpcAction::Draw => a.clone(),
            RpcAction::Lose => match a {
                RpcItem::Rock => RpcItem::Scissor,
                RpcItem::Paper => RpcItem::Rock,
                RpcItem::Scissor => RpcItem::Paper,
            },
        }
    }
}

fn parse_code(input: String, as_result: bool) -> RockPaperScissor {
    let mut result = RockPaperScissor::new();
    for line in input.lines() {
        let mut item = line.splitn(2, ' ');
        let item1 = RockPaperScissor::decrypt_item_a(item.next().unwrap());
        let item2 = item.next().unwrap();

        if !as_result {
            result.add_draw(&item1, &RockPaperScissor::decrypt_item_b_as_item(item2));
        } else {
            result.add_draw(
                &item1,
                &RockPaperScissor::get_item_for_action(
                    &item1,
                    &RockPaperScissor::decrypt_item_b_as_result(item2),
                ),
            );
        }
    }
    result
}
