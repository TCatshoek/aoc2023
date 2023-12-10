#![feature(let_chains)]

use std::collections::{BTreeMap, BTreeSet, VecDeque};

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
            Tile::SW => vec![(x, y - 1), (x, y + 1)],
            Tile::SE => vec![(x, y - 1), (x, y + 1)],
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
            start_pos
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

fn find_largest_distance(world: &World) -> ((usize, usize), i32) {
    let mut visited = BTreeMap::new();
    let mut to_visit = VecDeque::new();

    let mut best_tile = (world.start_pos, 0);

    to_visit.push_back((world.start_pos, 0));

    while let Some(((x, y), cur_distance)) = to_visit.pop_front() {


        visited.insert((x, y), cur_distance);

        if cur_distance > best_tile.1 {
            best_tile = ((x, y), cur_distance);
        }

        let cur_tile = world.get(x, y).unwrap();

        for (new_x, new_y) in cur_tile.next_positions(x, y) {

            // Only revisit if the current path is longer
            if let Some(distance) = visited.get(&(new_x, new_y)) && *distance > cur_distance + 1{
                continue
            }

            // Only visit within bounds of map & valid tile types
            match world.get(new_x, new_y) {
                None | Some(Tile::GROUND) => continue,
                Some(_) => ()
            }

            to_visit.push_back(((new_x, new_y), cur_distance + 1))
        }

        println!("To visit: {:?}", to_visit);
    }

    best_tile
}

fn main() {
    println!("Hello, world!");
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

        let (_, dist) = find_largest_distance(&world);

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

        assert_eq!(dist, 8);
    }
}