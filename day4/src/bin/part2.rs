use std::str::FromStr;
use day4::*;

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
        .map(|result| result.unwrap())
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
            .map(|result| result.unwrap())
            .collect::<Vec<Card>>();

        let score = calc_amount_scratch_cards(cards.as_slice());

        assert_eq!(score, 30);
    }
}