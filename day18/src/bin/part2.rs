#![feature(let_chains)]

use std::collections::VecDeque;
use std::fmt::{Debug, Display};
use glam::IVec2;
use itertools::Itertools;
use regex::Regex;
use rustc_hash::FxHashSet;
use aoc2023::direction::Direction;
use aoc2023::map2d::Map2D;


struct Command {
    direction: Direction,
    n_steps: i32,
    color: [u8; 3],
}

fn parse_pt1(input: &str) -> Vec<Command> {
    let re = Regex::new(r"^(?<direction>\w) (?<steps>\d+) \(#(?<color>\w+)\)$").unwrap();

    input.lines()
        .map(|line| {
            let caps = re.captures(line).unwrap();
            let direction = Direction::from_udlr(caps.name("direction").unwrap().as_str());
            let n_steps = caps.name("steps").unwrap().as_str()
                .parse().unwrap();
            let color = hex::decode(caps.name("color").unwrap().as_str()).unwrap()
                .try_into().unwrap();

            Command {
                direction,
                n_steps,
                color,
            }
        })
        .collect()
}

fn parse_pt2(input: &str) -> Vec<Command> {
    let re = Regex::new(r"^(?<direction>\w) (?<steps>\d+) \(#(?<color>\w+)\)$").unwrap();

    input.lines()
        .map(|line| {
            let caps = re.captures(line).unwrap();
            let color = hex::decode(caps.name("color").unwrap().as_str()).unwrap()
                .try_into().unwrap();

            let color_str = caps.name("color").unwrap().as_str();
            let dist_part = &color_str[0..5];
            let dir_part = &color_str[5..];

            let dist_vec = hex::decode(format!("000{}", dist_part)).unwrap();
            let dist = u32::from_be_bytes(dist_vec.try_into().expect("Couldn't convert"));

            let dir_byte: [u8; 1] = hex::decode(format!("0{}", dir_part)).unwrap().try_into().unwrap();
            let dir = match dir_byte[0] {
                0 => Direction::from_udlr("R"),
                1 => Direction::from_udlr("D"),
                2 => Direction::from_udlr("L"),
                3 => Direction::from_udlr("U"),
                _ => panic!()
            };

            Command {
                direction: dir,
                n_steps: dist as i32,
                color,
            }
        })
        .collect()
}

fn determine_bounds(commands: &Vec<Command>) -> (IVec2, IVec2) {
    let mut pos = IVec2::new(0, 0);
    let mut max_x = 0;
    let mut max_y = 0;
    let mut min_x = 0;
    let mut min_y = 0;

    for Command { direction, n_steps, .. } in commands {
        pos += direction.as_delta() * *n_steps as i32;
        max_x = max_x.max(pos.x);
        max_y = max_y.max(pos.y);
        min_x = min_x.min(pos.x);
        min_y = min_y.min(pos.y);
    }

    return (IVec2::new(min_x, min_y), IVec2::new(max_x, max_y));
}

struct Segment(IVec2, IVec2, Direction);

impl Segment {
    fn crosses_y(&self, y: i32) -> bool {
        let min = self.0.y.min(self.1.y);
        let max = self.0.y.max(self.1.y);
        min <= y && y < max
    }

    fn crosses_x(&self, x: i32) -> bool {
        let min = self.0.x.min(self.1.x);
        let max = self.0.x.max(self.1.x);
        min <= x && x < max
    }

    fn length(&self) -> IVec2 {
        (self.0 - self.1).abs()
    }
}

fn get_segments(commands: &Vec<Command>) -> Vec<Segment> {
    let (b_min, _) = determine_bounds(commands);
    let mut segments = Vec::new();

    let mut pos = b_min.abs();
    for Command { direction, n_steps, color } in commands {
        let new_pos = pos + direction.as_delta() * *n_steps;
        segments.push(Segment(pos, new_pos, *direction));
        pos = new_pos;
    }
    segments
}

fn solve(commands: &Vec<Command>) -> i64 {
    let segments = get_segments(&commands);

    let mut total: i64 = 0;

    for (idx, segment) in segments.iter().enumerate() {
        let prev_idx = match idx {
            0 => segments.len() - 1,
            x => x - 1
        };
        let next_idx = (idx + 1) % segments.len();

        let prev_dir = segments.get(prev_idx).unwrap().2;
        let next_dir = segments.get(next_idx).unwrap().2;

        match segment.2 {
            Direction::North => {
                let mut edges = 0;
                if prev_dir == Direction::East {
                    edges += 1;
                }
                if next_dir == Direction::West {
                    edges += 1;
                }

                let segment_length = segment.length().y + 1;

                let to_sub = (segment_length - edges) as i64 * segment.0.x as i64;

                total -= to_sub
            }
            Direction::South => {
                let mut edges = 0;
                if prev_dir == Direction::West {
                    edges += 1;
                }
                if next_dir == Direction::East {
                    edges += 1;
                }

                let segment_length = segment.length().y + 1;

                let to_add = (segment_length - edges) as i64 * (segment.0.x + 1) as i64;

                total += to_add;
            }

            _ => {}
        }
    }

    total
}

fn main() {
    let input = include_str!("../input.txt");
    let commands = parse_pt2(input);
    let result = solve(&commands);
    println!("Result: {}", result);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_input_1() {
        let input = "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";
        let commands = parse_pt1(input);
        let result = solve(&commands);
        assert_eq!(result, 62);
    }

    #[test]
    fn test_input_1_pt2() {
        let input = "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";
        let commands = parse_pt2(input);
        let result = solve(&commands);
        assert_eq!(result, 952408144115);
    }

    #[test]
    fn test_part_1() {
        let input = include_str!("../input.txt");
        let commands = parse_pt1(input);
        let result = solve(&commands);
        assert_eq!(result, 35244)
    }
}
