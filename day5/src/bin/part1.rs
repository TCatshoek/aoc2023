use miette::{miette, Report};
use std::str::FromStr;
use itertools::Itertools;
use regex::Regex;

#[derive(Debug)]
struct Range {
    src: u32,
    dst: u32,
    len: u32
}

impl Range {
    fn contains(&self, input: u32) -> bool {
        input < self.src + self.len && input >= self.src
    }

    fn map(&self, input: u32) -> u32 {
        input + (self.dst - self.src)
    }
}

#[derive(Debug)]
struct Mapping {
    ranges: Vec<Range>
}

impl Mapping {
    fn map(&self, input: u32) -> u32 {
        for range in &self.ranges {
            if range.contains(input) {
                return range.map(input)
            }
        }
        input
    }
}

impl FromStr for Mapping {
    type Err = Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut ranges = Vec::new();

        for line in s.lines() {

            let (dst, src, len) = line.split_whitespace()
                .map(|part| part.parse::<u32>().map_err(|_| miette!("Failed to parse number: {}", part)))
                .collect::<Result<Vec<u32>, Self::Err>>()?.iter().copied()// Can this collect be avoided?
                .collect_tuple()
                .ok_or(miette!("Tuple destructuring failed: {}", line))?;

            ranges.push(Range{
                src, dst, len
            })
        }

        Ok(Mapping{
            ranges
        })
    }
}

fn parse_game(input: &str) -> (Vec<u32>, Vec<Mapping>) {
    let mut lines = input.lines();
    let line = lines.next().unwrap();

    //Parse seeds
    let seeds_re = Regex::new(r"^seeds: (?<seednums>.*)$").unwrap();

    let Some(seeds_caps) = seeds_re.captures(line) else {
        panic!("Couldn't parse seeds");
    };

    let seed_nums: Vec<u32> = seeds_caps.name("seednums")
        .unwrap().as_str()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    // Parse mappings
    let map_re = Regex::new(r"(?<maptype>[\w-]+) map:\n(?<mapnumbers>(\d+ \d+ \d+\s?)+)").unwrap();

    let Some(maps_caps) = map_re.captures(input) else {
        panic!("Couldn't parse mappings");
    };

    let mut mappings = Vec::new();

    for captures in map_re.captures_iter(input) {
        let name = captures.name("maptype").unwrap().as_str();
        let numbers = captures.name("mapnumbers").unwrap().as_str();

        println!("name: {}", name);
        println!("{}", numbers);

        mappings.push(Mapping::from_str(numbers).unwrap());
    }

    (seed_nums, mappings)
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parse() {
        let input ="seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
        let (seeds, mappings) = parse_game(input);
        println!("{:?}", seeds);
        println!("{:?}", mappings);
    }
}