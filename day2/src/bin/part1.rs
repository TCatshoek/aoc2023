use std::collections::HashMap;
use std::collections::hash_map::Iter;
use itertools::Itertools;
use regex::Regex;

#[derive(Debug)]
struct Grab<'a> {
    cubes: HashMap<&'a str, u32>,
}

impl<'a> Grab<'a> {
    fn from_str(input: &'a str) -> Self {
        let mut cubes = HashMap::new();

        for part in input.split(",") {
            if let Some((n, color)) = part.split_whitespace().collect_tuple() {
                let num: u32 = n.parse().unwrap();
                cubes.insert(color.into(), num);
            }
        };

        Grab {
            cubes
        }
    }

    fn items(&self) -> Iter<'_, &'a str, u32> {
        self.cubes.iter()
    }
}

#[derive(Debug)]
struct Game<'a> {
    id: u32,
    grabs: Vec<Grab<'a>>,
}

impl<'a> Game<'a> {
    fn from_str(input: &'a str) -> Self {
        let re = Regex::new(r"^Game (?<id>\d+):\s(?<rest>.*)$").unwrap();
        let Some(caps) = re.captures(input) else {
            panic!("Invalid input");
        };

        let id = caps["id"].parse::<u32>().expect("Couldn't parse game id to int");
        let rest = caps.name("rest").unwrap().as_str();

        let grabs: Vec<Grab> = rest.split(";").map(|grab_str| Grab::from_str(grab_str)).collect();

        Game {
            id,
            grabs,
        }
    }

    fn get_max_counts(&self) -> HashMap<&'a str, u32> {
        let mut max_counts = HashMap::new();

        for grab in &self.grabs {
            for (color, count) in grab.items() {
                if max_counts.contains_key(color) {
                    if max_counts[color] < *count {
                        max_counts.insert(*color, *count);
                    }
                } else {
                    max_counts.insert(*color, *count);
                }
            }
        }

        max_counts
    }

    fn is_possible(&self, bag_contains: &HashMap<&str, u32>) -> bool {
        let max_counts = self.get_max_counts();

        max_counts.iter()
            .map(|(color, draw_count)| {
                if let Some(max_count) = bag_contains.get(color) {
                    draw_count <= max_count
                } else {
                    false
                }
            })
            .all(|x| x)
    }
}

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