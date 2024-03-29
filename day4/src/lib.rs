use std::cell::Cell;
use std::collections::BTreeSet;
use std::str::FromStr;
use itertools::Itertools;
use miette::{miette, Report};


pub struct Card {
    pub card_id: u32,
    winning_numbers: BTreeSet<u32>,
    card_numbers: BTreeSet<u32>,
    num_matching: Cell<Option<usize>>,
}

impl FromStr for Card {
    type Err = Report;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let (card_str, numbers_str) = input.split(':').collect_tuple()
            .ok_or_else(|| miette!("Invalid input format"))?;

        let card_id = card_str.split_whitespace()
            .nth(1)
            .ok_or_else(|| miette!("Missing card ID"))?
            .parse()
            .map_err(|_| miette!("Invalid card ID"))?;


        let (winning_numbers_str, card_numbers_str) = numbers_str.split(" | ").collect_tuple()
            .ok_or_else(|| miette!("Invalid numbers format"))?;

        let winning_numbers = winning_numbers_str.split_whitespace()
            .map(|n| n.parse().map_err(|_| miette!("Invalid winning number: {}", n)))
            .collect::<Result<BTreeSet<u32>, _>>()?;

        let card_numbers = card_numbers_str.split_whitespace()
            .map(|n| n.parse().map_err(|_| miette!("Invalid card number: {}", n)))
            .collect::<Result<BTreeSet<u32>, _>>()?;

        Ok(Self {
            card_id,
            winning_numbers,
            card_numbers,
            num_matching: None.into()
        })
    }
}

impl Card {
    pub fn calc_score(&self) -> u32 {
        self.winning_numbers
            .intersection(&self.card_numbers)
            .enumerate()
            .last()
            .map(|(idx, _)| (2_u32).pow(idx as u32))
            .unwrap_or(0)
    }

    pub fn get_num_matching(&self) -> usize {
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