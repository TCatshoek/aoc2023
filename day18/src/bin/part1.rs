#![feature(let_chains)]

use std::collections::VecDeque;
use std::fmt::{Debug, Display, Formatter};
use glam::IVec2;
use regex::Regex;
use rustc_hash::FxHashSet;
use aoc2023::direction::Direction;
use aoc2023::map2d::Map2D;


#[derive(Copy, Clone, Eq, PartialEq)]
enum Block {
    Ground,
    Hole(Option<[u8; 3]>),
}

impl Debug for Block {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Block::Ground => write!(f, "{}", '.'),
            Block::Hole(_) => write!(f, "{}", '#')
        }
    }
}

impl Display for Block {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self::Debug::fmt(self, f)
    }
}

struct Command {
    direction: Direction,
    n_steps: u32,
    color: [u8; 3],
}

fn parse(input: &str) -> Vec<Command> {
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

    return (IVec2::new(min_x, min_y), IVec2::new(max_x, max_y))
}

fn dig(commands: &Vec<Command>) -> Map2D<Block> {
    let (b_min, b_max) = determine_bounds(commands);

    let width = b_min.x.abs_diff(b_max.x) as usize + 1;
    let height = b_min.y.abs_diff(b_max.y) as usize + 1;

    let mut world = Map2D::from_size(
        width,
        height,
        Block::Ground
    );

    let mut pos = b_min.abs();
    for Command { direction, n_steps, color } in commands {
        for _ in 0..*n_steps {
            world.set_v(pos, Block::Hole(Some(*color)));
            pos += direction.as_delta();
        }
    }

    world
}

fn get_edge_ground(world: &Map2D<Block>) -> FxHashSet<IVec2> {
    let mut goals = FxHashSet::default();

    goals.extend(world.iter_row(0).enumerate().filter_map(|(idx, &x)| match x {
        Block::Ground => Some(IVec2::new(idx as i32, 0)),
        Block::Hole(_) => None
    }));

    goals.extend(world.iter_row(world.width - 1).enumerate().filter_map(|(idx, &x)| match x {
        Block::Ground => Some(IVec2::new(idx as i32, world.height as i32 - 1)),
        Block::Hole(_) => None
    }));

    let first_col = world.iter_cols().next().unwrap();
    goals.extend(first_col.enumerate().filter_map(|(idx, x)| match x {
        Block::Ground => Some(IVec2::new(0, idx as i32)),
        Block::Hole(_) => None
    }));

    let last_col = world.iter_cols().last().unwrap();
    goals.extend(last_col.enumerate().filter_map(|(idx, x)| match x {
        Block::Ground => Some(IVec2::new(world.width as i32 - 1, idx as i32)),
        Block::Hole(_) => None
    }));

    goals
}

fn fill(world: &mut Map2D<Block>) {
    // Time to floodfill and see if we can reach the side of the map
    let goals = get_edge_ground(world);
    let mut outside = FxHashSet::default();

    for y in 0..world.height {
        for x in 0..world.width {
            // y = 6 x = 5
            if world.get(x, y).unwrap() != Block::Ground
                || outside.contains(&IVec2::new(x as i32, y as i32)){
                continue;
            }

            let mut visited = FxHashSet::default();
            let mut to_visit = VecDeque::new();

            to_visit.push_back(IVec2::new(x as i32, y as i32));

            while let Some(pos) = to_visit.pop_front() {
                for direction in Direction::all() {
                    let next_pos = pos + direction.as_delta();

                    if visited.contains(&next_pos) {
                        continue
                    }

                    if let Some(block) = world.get_v(next_pos) && block == Block::Ground {
                        to_visit.push_back(next_pos);
                        visited.insert(next_pos);
                    }
                }
            }

            if visited.intersection(&goals).count() == 0 {
                for pos in visited {
                    world.set_v(pos, Block::Hole(None))
                }
            } else {
                for pos in visited {
                    outside.insert(pos);
                }
            }
        }
    }
}

fn count(world: &Map2D<Block>) -> i32 {
    let mut count = 0;
    for y in 0..world.height {
        for x in 0..world.width {
            match world.get(x, y).unwrap() {
                Block::Ground => {}
                Block::Hole(_) => {count += 1}
            }
        }
    }
    count
}

fn solve(input: &str) -> i32 {
    let commands = parse(input);
    let mut world = dig(&commands);
    println!("World: \n{:?}", world);
    fill(&mut world);
    println!("Filled: \n{:?}", world);
    count(&world)
}

fn main() {
    let input = include_str!("../input.txt");
    let result = solve(input);
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

        let commands = parse(input);
        let mut world = dig(&commands);
        println!("World: \n{:?}", world);
        fill(&mut world);
        println!("Filled: \n{:?}", world);
        let result = count(&world);

        assert_eq!(result, 62);
    }
}
