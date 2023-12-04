use std::cell::Cell;
use std::collections::BTreeSet;
use itertools::Itertools;

struct Card {
    card_id: u32,
    winning_numbers: BTreeSet<u32>,
    card_numbers: BTreeSet<u32>,
    num_matching: Cell<Option<usize>>,
}

impl Card {
    fn from_str(input: &str) -> Self {
        let (card_str, numbers_str) = input.split(':').collect_tuple().unwrap();

        let card_id = card_str.split_whitespace()
            .nth(1).unwrap()
            .parse().unwrap();

        let (winning_numbers_str, card_numbers_str) = numbers_str.split(" | ").collect_tuple().unwrap();

        let winning_numbers = BTreeSet::from_iter(
            winning_numbers_str
                .split_whitespace()
                .map(|n| n.parse().unwrap())
        );
        let card_numbers = BTreeSet::from_iter(
            card_numbers_str
                .split_whitespace()
                .map(|n| n.parse().unwrap())
        );

        Self {
            card_id,
            winning_numbers,
            card_numbers,
            num_matching: None.into()
        }
    }

    fn calc_score(&self) -> u32 {
        self.winning_numbers
            .intersection(&self.card_numbers)
            .enumerate()
            .last()
            .map(|(idx, _)| (2_u32).pow(idx as u32))
            .unwrap_or(0)
    }

    fn get_num_matching(&self) -> usize {
        match self.num_matching.get() {
            None => {
                let n = self.winning_numbers
                    .intersection(&self.card_numbers)
                    .count();
                self.num_matching.replace(Some(n));
                n
            }
            Some(n) => n
        }
    }
}

fn calc_amount_scratch_cards(cards: &[Card]) -> usize {
    let mut to_visit = Vec::from_iter(cards);
    let mut visited = Vec::new();

    while let Some(cur_card) = to_visit.pop() {
        let num_matches = cur_card.get_num_matching();
        let idx = cur_card.card_id as usize;

        visited.push(cur_card);

        if num_matches > 0 {
            let to_add = &cards[idx..idx + num_matches];
            to_visit.extend(to_add);
        }
    }

    visited.len()
}

fn main() {
    let input = include_str!("../input1.txt");

    let cards = input.lines()
        .map(Card::from_str)
        .collect::<Vec<Card>>();

    let score = calc_amount_scratch_cards(cards.as_slice());

    println!("Result: {}", score);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_number() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

        let cards = input.lines()
            .map(Card::from_str)
            .collect::<Vec<Card>>();

        let score = calc_amount_scratch_cards(cards.as_slice());

        assert_eq!(score, 30);
    }
}