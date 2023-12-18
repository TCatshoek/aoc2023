use std::fmt::{Debug, Display, Formatter};
use glam::IVec2;
use regex::Regex;
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
    let re = Regex::new(r"^(?<direction>\w) (?<steps>\d) \(#(?<color>\w+)\)$").unwrap();

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

fn determine_bounds(commands: &Vec<Command>) -> IVec2 {
    let mut pos = IVec2::new(0, 0);
    let mut max_x = 0;
    let mut max_y = 0;

    for Command { direction, n_steps, .. } in commands {
        pos += direction.as_delta() * *n_steps as i32;
        max_x = max_x.max(pos.x);
        max_y = max_y.max(pos.y);
    }

    return IVec2::new(max_x, max_y);
}

fn dig(commands: &Vec<Command>) -> Map2D<Block> {
    let bounds = determine_bounds(commands);

    let mut world = Map2D::from_size(
        bounds.x as usize + 1,
        bounds.y as usize + 1,
        Block::Ground
    );

    let mut pos = IVec2::new(0, 0);
    for Command { direction, n_steps, color } in commands {
        for _ in 0..*n_steps {
            world.set_v(pos, Block::Hole(Some(*color)));
            pos += direction.as_delta();
        }
    }

    world
}

fn fill(world: &mut Map2D<Block>) {
    for y in 0..world.height {
        for x in 0..world.width {

            if world.get(x, y).unwrap() != Block::Ground {
                continue;
            }

            let mut in_wall = false;
            let mut n_crossings = 0;

            for b in world.iter_row(y).skip(x) {
                match (b, in_wall) {
                    (Block::Ground, true) => {in_wall = false;}
                    (Block::Ground, false) => {}
                    (Block::Hole(_), true) => {}
                    (Block::Hole(_), false) => {in_wall = true; n_crossings += 1}
                };
            };

            if n_crossings % 2 == 1 {
                world.set(x, y, Block::Hole(None));
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

fn main() {
    println!("Hello, world!");
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
