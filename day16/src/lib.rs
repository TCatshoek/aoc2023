use std::collections::{HashSet, VecDeque};
use glam::IVec2;
use itertools::Itertools;
use rustc_hash::FxHashSet;
use aoc2023::direction::Direction;
use aoc2023::world::World;

pub fn walk(world: &World, start_pos: IVec2, start_direction: Direction) -> Vec<IVec2> {
    let mut visited = FxHashSet::default();
    let mut to_visit = VecDeque::new();
    to_visit.push_back((start_pos, start_direction));

    while let Some((pos, direction)) = to_visit.pop_front() {

        if visited.contains(&(pos, direction)) {
            continue;
        }

        if let Some(tile) = world.get(pos.x as usize, pos.y as usize) {
            visited.insert((pos, direction));

            match tile {
                '.' => to_visit.push_back((pos + direction.as_delta(), direction)),
                '/' => {
                    let new_direction = match direction {
                        Direction::North => Direction::East,
                        Direction::East => Direction::North,
                        Direction::South => Direction::West,
                        Direction::West => Direction::South
                    };
                    to_visit.push_back((pos + new_direction.as_delta(), new_direction));
                }
                '\\' => {
                    let new_direction = match direction {
                        Direction::North => Direction::West,
                        Direction::East => Direction::South,
                        Direction::South => Direction::East,
                        Direction::West => Direction::North
                    };
                    to_visit.push_back((pos + new_direction.as_delta(), new_direction));
                }
                '-' => match direction {
                    Direction::East | Direction::West => to_visit.push_back((pos + direction.as_delta(), direction)),
                    Direction::North | Direction::South => {
                        to_visit.push_back((pos + Direction::East.as_delta(), Direction::East));
                        to_visit.push_back((pos + Direction::West.as_delta(), Direction::West));
                    }
                },
                '|' => match direction {
                    Direction::North | Direction::South => to_visit.push_back((pos + direction.as_delta(), direction)),
                    Direction::East | Direction::West => {
                        to_visit.push_back((pos + Direction::North.as_delta(), Direction::North));
                        to_visit.push_back((pos + Direction::South.as_delta(), Direction::South));
                    }
                }
                c => panic!("Unknown tile: {}", c)
            }
        }
    }

    visited.iter().copied()
        .map(|(pos, _dir)| pos)
        .unique()
        .collect()
}