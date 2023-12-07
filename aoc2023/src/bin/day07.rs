use std::cmp::Ordering;
use std::collections::HashMap;

const NORMAL_SORT: [char; 13] = [
    'A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2',
];

const JOKER_SORT: [char; 13] = [
    'A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2', 'J',
];

fn card_sort(
    char_score: &[char],
    a: &(usize, String, usize),
    b: &(usize, String, usize),
) -> Ordering {
    if a.0 == b.0 {
        let chars_a: Vec<_> =
            a.1.chars()
                .map(|i| char_score.iter().position(|j| j == &i).unwrap())
                .collect();
        let chars_b: Vec<_> =
            b.1.chars()
                .map(|i| char_score.iter().position(|j| j == &i).unwrap())
                .collect();
        for i in 0..chars_a.len() {
            if chars_a[i] == chars_b[i] {
                continue;
            } else if chars_a[i] > chars_b[i] {
                return Ordering::Less;
            } else {
                return Ordering::Greater;
            }
        }
        Ordering::Equal
    } else if a.0 > b.0 {
        Ordering::Less
    } else {
        Ordering::Greater
    }
}

pub fn rank_cards((cards, bid): (&str, &str)) -> ((usize, String, usize), (usize, String, usize)) {
    let jokers = cards.matches('J').count();
    let temp_cards = cards.replace("J", "");

    let mut card_occurences: Vec<usize> = temp_cards
        .chars()
        .fold(HashMap::new(), |mut acc, c| {
            *acc.entry(c).or_insert(0) += 1;
            acc
        })
        .values()
        .map(|i| *i)
        .collect();

    let mut no_joker = card_occurences.clone();
    no_joker.push(jokers);
    no_joker.sort_by(|a, b| b.cmp(a));

    card_occurences.sort_by(|a, b| b.cmp(a));

    // add joker score
    if card_occurences.len() > 0 {
        card_occurences[0] += jokers;
    } else {
        // if we only have jokers
        card_occurences.push(5);
    }

    (
        (rank(no_joker), cards.to_string(), bid.parse().unwrap()),
        (
            rank(card_occurences),
            cards.to_string(),
            bid.parse().unwrap(),
        ),
    )
}

pub fn rank(counts: Vec<usize>) -> usize {
    if counts[0] == 5 {
        1
    } else if counts[0] == 4 {
        2
    } else if counts[0] == 3 && counts[1] == 2 {
        3
    } else if counts[0] == 3 {
        4
    } else if counts[0] == 2 && counts[1] == 2 {
        5
    } else if counts[0] == 2 {
        6
    } else {
        7
    }
}

#[aoc2023::main(07)]
fn solution(input: &str) -> (usize, usize) {
    let mut hands = vec![];
    let mut joker_hands = vec![];
    for line in input.lines() {
        // get cards with the bid
        let (cards, bid) = line.split_once(' ').unwrap();

        // count the individual cards
        let ranked = rank_cards((cards, bid));
        hands.push(ranked.0);
        joker_hands.push(ranked.1);
    }

    hands.sort_by(|a, b| card_sort(&NORMAL_SORT, a, b));
    joker_hands.sort_by(|a, b| card_sort(&JOKER_SORT, a, b));
    let total_p1 = hands.iter().enumerate().map(|(i, x)| (i + 1) * x.2).sum();
    let total_p2 = joker_hands
        .iter()
        .enumerate()
        .map(|(i, x)| (i + 1) * x.2)
        .sum();

    (total_p1, total_p2)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r#"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"#;

    #[test]
    fn test_solution1() -> Result<(), String> {
        assert_eq!(solution(TEST_INPUT.trim()).0, 6440);
        Ok(())
    }

    #[test]
    fn test_solution2() -> Result<(), String> {
        assert_eq!(solution(TEST_INPUT.trim()).1, 5905);
        Ok(())
    }
}
