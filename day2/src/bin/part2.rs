use day2::*;

fn main() {
    let input = include_str!("../input1.txt");

    let sum_power: u32 = input.lines()
        .map(|line| Game::from_str(line))
        .map(|game| game.get_max_counts()
            .iter()
            .fold(1, |acc, (color, count)| acc * count)
        )
        .sum();

    println!("Sum of Game powers: {}", sum_power);
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


        let games: Vec<Game> = input.lines()
            .map(|line| Game::from_str(line))
            .collect();

        let game_powers = games.iter()
            .map(|game| game.get_max_counts()
                .iter()
                .fold(1, |acc, (color, count)| acc * count)
            )
            .collect::<Vec<u32>>();

        assert_eq!(game_powers.iter().sum::<u32>(), 2286);

    }
}