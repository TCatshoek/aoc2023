use std::iter::zip;
use day6::*;

fn parse_games(input: &str) -> Vec<(Num, Num)> {
    let mut lines = input.lines();

    let times: Vec<Num> = lines.next().unwrap()
        .strip_prefix("Time:").unwrap()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let distances: Vec<Num> = lines.next().unwrap()
        .strip_prefix("Distance:").unwrap()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    zip(times, distances).collect::<Vec<(Num, Num)>>()
}

fn main() {
    let input = include_str!("../input1.txt");

    let games = parse_games(input);

    let score = games.iter().map(|game| {
        let (low, high) = search(game.0, game.1);
        high - low + 1
    }).product::<Num>();

    println!("Score: {}", score);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let input = "Time:      7  15   30
Distance:  9  40  200";

        let games = parse_games(input);

        let score = games.iter().map(|game| {
            let (low, high) = search(game.0, game.1);
            high - low + 1
        }).product::<Num>();

        assert_eq!(score, 288);
    }
}