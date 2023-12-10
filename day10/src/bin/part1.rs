#![feature(let_chains)]

use std::collections::{BTreeMap, BTreeSet, VecDeque};
use crate::Tile::SW;

#[derive(Copy, Clone, Eq, PartialEq)]
enum Tile {
    NS,
    EW,
    NE,
    NW,
    SW,
    SE,
    GROUND,
    START,
}

impl Tile {
    fn new(c: char) -> Self {
        match c {
            '|' => Self::NS,
            '-' => Self::EW,
            'L' => Self::NE,
            'J' => Self::NW,
            '7' => Self::SW,
            'F' => Self::SE,
            '.' => Self::GROUND,
            'S' => Self::START,
            x => panic!("Unknown tile type: {}", x)
        }
    }

    fn next_positions(&self, cur_x: usize, cur_y: usize) -> Vec<(usize, usize)> {
        let x = cur_x as i32;
        let y = cur_y as i32;

        let possible_positions = match self {
            Tile::NS => vec![(x, y - 1), (x, y + 1)],
            Tile::EW => vec![(x - 1, y), (x + 1, y)],
            Tile::NE => vec![(x, y - 1), (x + 1, y)],
            Tile::NW => vec![(x, y - 1), (x - 1, y)],
            Tile::SW => vec![(x - 1, y), (x, y + 1)],
            Tile::SE => vec![(x, y + 1), (x + 1, y)],
            Tile::GROUND => vec![],
            Tile::START => vec![(x, y - 1), (x, y + 1), (x + 1, y), (x - 1, y)]
        };

        possible_positions.iter().copied().filter_map(|el| match el {
            (x, y) if x >= 0 && y >= 0 => Some((x as usize, y as usize)),
            _ => None
        }).collect()
    }
}

struct World {
    tiles: Vec<Tile>,
    width: usize,
    height: usize,
    start_pos: (usize, usize),
}

impl World {
    fn new(input: &str) -> Self {
        let mut tiles = Vec::new();
        let mut width = 0;
        let mut height = 0;
        let mut start_pos = (0, 0);

        for (y, line) in input.lines().enumerate() {
            height = y + 1;
            for (x, c) in line.chars().enumerate() {
                width = x + 1;
                let new_tile = Tile::new(c);
                tiles.push(new_tile);
                if new_tile == Tile::START {
                    start_pos = (x, y);
                }
            }
        }

        Self {
            tiles,
            width,
            height,
            start_pos,
        }
    }

    fn is_in_bounds(&self, x: usize, y: usize) -> bool {
        x < self.width && y < self.height
    }

    fn get(&self, x: usize, y: usize) -> Option<Tile> {
        match (x, y) {
            (x, y) if self.is_in_bounds(x, y) => {
                self.tiles.get(x + y * self.width).copied()
            }
            _ => None
        }
    }
}

fn walk_loop(world: &World, start_pos: (usize, usize)) -> BTreeMap<(usize, usize), usize> {
    let mut visited = BTreeMap::new();
    let mut to_visit = VecDeque::new();

    visited.insert(world.start_pos, 0);
    to_visit.push_back((start_pos, 1));

    while let Some(((x, y), cur_distance)) = to_visit.pop_front() {
        // println!("Cur pos: {}, {}", x, y);
        visited.insert((x, y), cur_distance);

        let cur_tile = world.get(x, y).unwrap();

        if cur_tile == Tile::GROUND {
            panic!();
        }

        for (new_x, new_y) in cur_tile.next_positions(x, y) {

            // Don't revisit on this walk
            if visited.contains_key(&(new_x, new_y)) {
                continue;
            }

            // Only visit within bounds of map & valid tile types
            match world.get(new_x, new_y) {
                None | Some(Tile::GROUND) => continue,
                Some(_) => ()
            }

            to_visit.push_back(((new_x, new_y), cur_distance + 1))
        }
    }

    let max_dist = visited.values().max().unwrap();
    println!("Loop len: {}", max_dist);

    visited
}

fn find_largest_distance(world: &World) -> ((usize, usize), usize) {
    let (start_x, start_y) = world.start_pos;

    let mut loop_start_positions: Vec<(usize, usize)> = Vec::new(); //world.get(start_x, start_y)

    if let Some(tile) = world.get(start_x + 1, start_y) {
        if [Tile::EW, Tile::NW, Tile::SW].contains(&tile) {
            loop_start_positions.push((start_x + 1, start_y));
        }
    }

    if start_x > 0 {
        if let Some(tile) = world.get(start_x - 1, start_y) {
            if [Tile::EW, Tile::NE, Tile::SE].contains(&tile) {
                loop_start_positions.push((start_x - 1, start_y));
            }
        }
    }

    if let Some(tile) = world.get(start_x, start_y + 1) {
        if [Tile::NS, Tile::NW, Tile::NE].contains(&tile) {
            loop_start_positions.push((start_x, start_y + 1));
        }
    }

    if start_y > 0 {
        if let Some(tile) = world.get(start_x, start_y - 1) {
            if [Tile::NS, Tile::SW, Tile::SE].contains(&tile) {
                loop_start_positions.push((start_x, start_y - 1));
            }
        }
    }

    let walk_distances: Vec<BTreeMap<(usize, usize), usize>> = loop_start_positions.iter()
        .map(|pos| walk_loop(world, *pos))
        .collect();

    let mut max_distances = BTreeMap::new();

    for distances in walk_distances {
        for ((x, y), d) in distances {
            max_distances.entry((x, y))
                .and_modify(|cur_best| { if d < *cur_best { *cur_best = d } })
                .or_insert(d);
        }
    }

    let mut best_dist = ((start_x, start_y), 0);
    for ((x, y), d) in max_distances {
        if d > best_dist.1 {
            best_dist = ((x, y), d);
        }
    }
    best_dist
}

fn main() {
    let input = include_str!("../input1.txt");
    let world = World::new(input);
    let (_, dist) = find_largest_distance(&world);
    println!("Result: {}", dist);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_1() {
        let input = ".....
.S-7.
.|.|.
.L-J.
.....";

        let world = World::new(input);

        let ((x, y), dist) = find_largest_distance(&world);
        println!("{:?}", ((x, y), dist));
        assert_eq!(dist, 4);
    }

    #[test]
    fn test_input_2() {
        let input = "..F7.
.FJ|.
SJ.L7
|F--J
LJ...";

        let world = World::new(input);

        let ((x, y), dist) = find_largest_distance(&world);
        println!("{:?}", ((x, y), dist));
        assert_eq!(dist, 8);
    }
}