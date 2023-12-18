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

struct Segment(IVec2, IVec2);

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
}

fn get_segments(commands: &Vec<Command>) -> (Vec<Segment>, Vec<Segment>) {
    let (b_min, b_max) = determine_bounds(commands);
    let mut vertical_segments = Vec::new();
    let mut horizontal_segments = Vec::new();

    let mut pos = b_min.abs();
    for Command { direction, n_steps, color } in commands {
        let new_pos = pos + direction.as_delta() * *n_steps;
        match direction {
            Direction::North | Direction::South => vertical_segments.push(Segment(pos, new_pos)),
            Direction::East | Direction::West => horizontal_segments.push(Segment(pos, new_pos))
        };
        pos = new_pos;
    }
    (vertical_segments, horizontal_segments)
}

fn solve(commands: &Vec<Command>) -> u64 {
    // Horizontal segments |
    let (vertical_segments, horizontal_segments) = get_segments(&commands);

    // y value of segment endpoints projected on the y axis
    let points = vertical_segments.iter()
        .flat_map(|s| [s.0.y, s.1.y])
        .unique()
        .sorted()
        .collect::<Vec<_>>();

    // Horizontal segments divided into parts
    let segment_parts = points.windows(2)
        .map(|window| {
            if let [a, b] = *window {
                Segment(IVec2::new(0, a), IVec2::new(0, b))
            } else { panic!() }
        })
        .collect::<Vec<_>>();

    let results = segment_parts.iter()
        .map(|seg_part| {

            let crossings = vertical_segments.iter()
                .filter(|seg| seg.crosses_y(seg_part.0.y))
                .map(|seg| seg.0.x)
                .sorted()
                .collect::<Vec<_>>();

            let height = seg_part.0.y.abs_diff(seg_part.1.y);
            let mut inside = true;

            let box_area = crossings.windows(2)
                .map(|window| {
                    if let [a, b] = *window {
                        let width = a.abs_diff(b) + 1;
                        return if inside {
                            inside = false;
                            width as u64 * height as u64
                        } else {
                            inside = true;
                            0
                        }
                    } else {panic!()}
                })
                .sum::<u64>();

            box_area

        })
        .collect::<Vec<_>>();

    results.iter().sum()
}

fn main() {
    // let input = include_str!("../input.txt");
    // let result = solve(input);
    // println!("Result: {}", result);
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
    fn test_part_1() {
        let input = include_str!("../input.txt");
        let commands = parse_pt1(input);
        let result = solve(&commands);
        assert_eq!(result, 35244)
    }
}
