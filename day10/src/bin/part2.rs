#![feature(let_chains)]

use std::collections::{BTreeMap, BTreeSet, VecDeque};
use itertools::Itertools;
use crate::Tile::SW;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
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

    fn patch_start_pos(&mut self) {
        let connected_to_start = find_start_positions(self);
        let (start_x, start_y) = self.start_pos;

        // n s w e
        let start_directions = (
            if start_y > 0 {connected_to_start.contains(&(start_x, start_y - 1))} else {false},
            connected_to_start.contains(&(start_x, start_y + 1)),
            if start_x > 0 {connected_to_start.contains(&(start_x - 1, start_y))} else {false},
            connected_to_start.contains(&(start_x + 1, start_y)),
        );

        let start_replacement = match start_directions {
            (true, true, false, false) => Tile::NS,
            (false, false, true, true) => Tile::NS,
            (true, false, false, true) => Tile::NE,
            (true, false, true, false) => Tile::NW,
            (false, true, true, false) => Tile::SW,
            (false, true, false, true) => Tile::SE,
            _ => panic!("Invalid combo")
        };

        self.set(start_x, start_y, start_replacement)
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

    fn set(&mut self, x: usize, y: usize, tile: Tile) {
        if !self.is_in_bounds(x, y) { panic!("out of bounds") }
        self.tiles[x + y * self.width] = tile;
    }

    fn get_all_pos(&self, tile_type: Tile) -> Vec<(usize, usize)> {
        let mut positions = Vec::new();

        for x in 0..self.width {
            for y in 0..self.height {
                let cur_tile = self.get(x, y).unwrap();
                if cur_tile == tile_type {
                    positions.push((x, y));
                }
            }
        }

        positions
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

fn find_start_positions(world: &World) -> Vec<(usize, usize)> {
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

    loop_start_positions
}

fn find_largest_distance(world: &World) -> ((usize, usize), usize) {
    let mut loop_start_positions = find_start_positions(world);

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

    let mut best_dist = (world.start_pos, 0);
    for ((x, y), d) in max_distances {
        if d > best_dist.1 {
            best_dist = ((x, y), d);
        }
    }
    best_dist
}

fn count_loop_crossings(world: &World, loop_positions: &BTreeSet<(usize, usize)>, start_pos: (usize, usize)) -> usize {
    let (mut x, mut y) = start_pos;
    let mut n_loop_crossings = 0;

    while world.is_in_bounds(x, y) {
        let next_pos = (x + 1, y);

        if loop_positions.contains(&next_pos) {
            let next_type = world.get(next_pos.0, next_pos.1);

            match next_type {
                Some(Tile::NE) | Some(Tile::NW) | Some(Tile::NS) => { n_loop_crossings += 1 }
                Some(_other_tile) => (),
                None => ()
            }
        }
        (x, y) = next_pos;
    }

    n_loop_crossings
}

fn find_area_enclosed_by_loop(input: &str) -> usize {
    let mut world = World::new(input);
    world.patch_start_pos();

    let start_position = *find_start_positions(&world).first().unwrap();
    let loop_positions = walk_loop(&world, start_position)
        .keys()
        .copied()
        .collect::<BTreeSet<(usize, usize)>>();

    let mut non_loop_positions = BTreeSet::new();
    for x in 0..world.width {
        for y in 0..world.height {
            if !loop_positions.contains(&(x, y)) {
                non_loop_positions.insert((x, y));
            }
        }
    }

    non_loop_positions
        .iter().copied()
        .map(|(x, y)| {
            count_loop_crossings(&world, &loop_positions, (x, y)) % 2
        })
        .sum()
}

fn main() {
    let input = include_str!("../input1.txt");
    let num_inside = find_area_enclosed_by_loop(input);
    println!("Result: {}", num_inside);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_loop_crossings() {
        let input = "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";

        let mut world = World::new(input);
        world.patch_start_pos();

        let start_position = *find_start_positions(&world).first().unwrap();
        let loop_positions = walk_loop(&world, start_position)
            .keys()
            .copied()
            .collect::<BTreeSet<(usize, usize)>>();

        assert_eq!(count_loop_crossings(&world, &loop_positions, (0, 0)), 0);
        assert_eq!(count_loop_crossings(&world, &loop_positions, (0, 1)), 0);
        assert_eq!(count_loop_crossings(&world, &loop_positions, (0, 2)), 2);
        assert_eq!(count_loop_crossings(&world, &loop_positions, (0, 3)), 4);
    }

    #[test]
    fn test_input_1() {
        let input = "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";

        let num_inside = find_area_enclosed_by_loop(input);

        assert_eq!(num_inside, 4);
    }

    #[test]
    fn test_input_2() {
        let input = ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...";

        let num_inside = find_area_enclosed_by_loop(input);
        assert_eq!(num_inside, 8);
    }

    #[test]
    fn test_input_3() {
        let input = "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";

        let num_inside = find_area_enclosed_by_loop(input);
        assert_eq!(num_inside, 10);
    }
}