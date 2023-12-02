use std::collections::HashMap;
use day2::*;

fn main() {
    let input = include_str!("../input1.txt");

    let bag_contents = HashMap::from([
        ("red", 12),
        ("green", 13),
        ("blue", 14)
    ]);

    let sum_ids = input.lines()
        .map(|line| Game::from_str(line))
        .filter(|game| game.is_possible(&bag_contents))
        .fold(0, |acc, el| acc + el.id);

    println!("Sum of possible Game IDs: {}", sum_ids);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        let bag_contents = HashMap::from([
            ("red", 12),
            ("green", 13),
            ("blue", 14)
        ]);

        let games: Vec<Game> = input.lines()
            .map(|line| Game::from_str(line))
            .collect();

        let possible_games: Vec<&Game> = games.iter()
            .filter_map(|game| if game.is_possible(&bag_contents) { Some(game) } else { None })
            .collect();

        for game in &possible_games {
            println!("{:?}", game);
        }

        let summed_ids = possible_games.iter()
            .fold(0, |acc, game| acc + game.id);

        assert_eq!(summed_ids, 8)
    }
}