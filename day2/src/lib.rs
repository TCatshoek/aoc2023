use std::collections::HashMap;
use std::collections::hash_map::Iter;
use itertools::Itertools;
use regex::Regex;

#[derive(Debug)]
pub struct Grab<'a> {
    cubes: HashMap<&'a str, u32>,
}

impl<'a> Grab<'a> {
    pub fn from_str(input: &'a str) -> Self {
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
pub struct Game<'a> {
    pub id: u32,
    grabs: Vec<Grab<'a>>,
}

impl<'a> Game<'a> {
    pub fn from_str(input: &'a str) -> Self {
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

    pub fn get_max_counts(&self) -> HashMap<&'a str, u32> {
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

    pub fn is_possible(&self, bag_contains: &HashMap<&str, u32>) -> bool {
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