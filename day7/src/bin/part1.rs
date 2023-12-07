use std::cmp::Ordering;
use std::fmt::{Debug, Formatter};
use std::hash::{Hash, Hasher};
use std::str::FromStr;
use itertools::Itertools;
use miette::{miette, Report};
use unicode_segmentation::UnicodeSegmentation;
use crate::HandType::{FiveOfAKind, FourOfAKind, FullHouse, HighCard, OnePair, ThreeOfAKind, TwoPair};

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug, Hash, Copy, Clone)]
enum Card {
    A,
    K,
    Q,
    J,
    T,
    N9,
    N8,
    N7,
    N6,
    N5,
    N4,
    N3,
    N2,
}

impl FromStr for Card {
    type Err = Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Card::A),
            "K" => Ok(Card::K),
            "Q" => Ok(Card::Q),
            "J" => Ok(Card::J),
            "T" => Ok(Card::T),
            "9" => Ok(Card::N9),
            "8" => Ok(Card::N8),
            "7" => Ok(Card::N7),
            "6" => Ok(Card::N6),
            "5" => Ok(Card::N5),
            "4" => Ok(Card::N4),
            "3" => Ok(Card::N3),
            "2" => Ok(Card::N2),
            x => Err(miette!("Unknown card: {}", x))
        }
    }
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard
}

struct Hand {
    cards: [Card; 5],
    bid: usize
}

impl FromStr for Hand {
    type Err = Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (mut cards, bid) = s.split_whitespace().collect_tuple::<(&str, &str)>()
            .ok_or(miette!("Failed to split tuple: {}", s))
            .and_then(|(a, b)| Ok((
                a.graphemes(true)
                    .map(|x| x.parse::<Card>())
                    .collect::<Result<Vec<Card>, Self::Err>>()
                    .map_err(|_| miette!("Failed to parse cards: {}", b))?,
                b.parse::<usize>().map_err(|_| miette!("Failed to parse bid: {}", a))?
            )))?;

        Ok(Self {
            cards: cards.try_into().map_err(|_| miette!("Failed to convert to array: {}", s))?,
            bid
        })
    }
}

impl Debug for Hand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Hand {{ cards: {:?} bid: {}, type: {:?} }}", self.cards, self.bid, self.get_type())
    }
}

impl Hash for Hand {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.cards.hash(state);
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        let type_ord = self.get_type().cmp(&other.get_type());

        if type_ord == Ordering::Equal {
            for (s, o) in self.cards.iter().zip(&other.cards) {
                let card_ord = s.cmp(o);
                if card_ord != Ordering::Equal {
                    return card_ord
                }
            }
        }

        type_ord
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq<Self> for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards
    }
}

impl Eq for Hand { }

impl Hand {
    fn get_type(&self) -> HandType {

        let card_counts = self.cards.iter().copied().counts();
        let mut sorted_cards = self.cards;
        sorted_cards.sort_by(|a, b| {
            match card_counts[b].cmp(&card_counts[a]) {
                Ordering::Less => Ordering::Less,
                Ordering::Equal => a.cmp(b),
                Ordering::Greater => Ordering::Greater,
            }
        });

        match sorted_cards {
            [a,b,c,d,e] if [a,b,c,d,e].iter().unique().count() == 1 => FiveOfAKind,
            [a,b,c,d,_] if [a,b,c,d].iter().unique().count() == 1 => FourOfAKind,
            [a,b,c,d,e] if [a,b,c].iter().unique().count() == 1  && d == e => FullHouse,
            [a,b,c,_,_] if [a,b,c].iter().unique().count() == 1 => ThreeOfAKind,
            [a,b,c,d,_] if a == b && c == d => TwoPair,
            [a,b,_,_,_] if a == b => OnePair,
            [a,b,c,d,e] if [a,b,c,d,e].iter().unique().count() == 5 => HighCard,
            _ => panic!("Unknown hand: {:?}", self)
        }
    }
}


fn main() {
    let input = include_str!("../input1.txt");

    let mut hands = input.lines()
        .map(Hand::from_str)
        .collect::<Result<Vec<Hand>, _>>()
        .unwrap();

    hands.sort();

    for (idx, hand) in hands.iter().rev().enumerate() {
        println!("{:?} - {}", hand, idx + 1);
    }

    let result = hands.iter().rev().enumerate()
        .fold(0, |acc, (idx, el)| acc + ((idx + 1) * el.bid));

    println!("Result: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

        let mut hands = input.lines()
            .map(Hand::from_str)
            .collect::<Result<Vec<Hand>, _>>()
            .unwrap();

        hands.sort();

        let result = hands.iter().rev().enumerate()
            .fold(0, |acc, (idx, el)| acc + ((idx + 1) * el.bid));

        assert_eq!(result, 6440);
    }

    #[test]
    fn check_unique() {
        let input = include_str!("../input1.txt");

        let hands = input.lines()
            .map(Hand::from_str)
            .collect::<Result<Vec<Hand>, _>>()
            .unwrap();

        let n_hands = hands.iter().count();
        let n_unique = hands.iter().unique().count();

        println!("number of hands: {}", n_hands);
        println!("unique hands: {}", n_unique);

        assert_eq!(n_unique, n_hands);
    }
}