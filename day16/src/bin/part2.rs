use std::collections::HashSet;
use glam::IVec2;
use rayon::prelude::*;
use aoc2023::direction::Direction;
use aoc2023::world::World;
use day16::walk;

fn generate_start_positions(world: &World) -> Vec<(IVec2, Direction)> {
    let mut edge_positions = HashSet::new();
    for x in 0..world.width {
        edge_positions.insert((x, 0));
        edge_positions.insert((x, world.height - 1));
    }

    for y in 0..world.height {
        edge_positions.insert((0, y));
        edge_positions.insert((world.width - 1, y));
    }

    let mut edge_pos_w_direction = Vec::new();
    for (x, y) in edge_positions {
        if x == 0 {
            edge_pos_w_direction.push(((x, y), Direction::East));
        }
        if x == world.width - 1 {
            edge_pos_w_direction.push(((x, y), Direction::West));
        }
        if y == 0 {
            edge_pos_w_direction.push(((x, y), Direction::South));
        }
        if y == world.height - 1 {
            edge_pos_w_direction.push(((x, y), Direction::North));
        }
    }

    edge_pos_w_direction.iter().copied()
        .map(|((x, y), direction)| (IVec2::new(x as i32, y as i32), direction))
        .collect()
}

fn main() {
    let input = include_str!("../input1.txt");
    let world = World::new(input);
    let start_positions = generate_start_positions(&world);
    let result = start_positions.par_iter()
        .map(|(pos, dir)| walk(&world, *pos, *dir).len())
        .max().unwrap();
    println!("Result: {}", result);
}

#[cfg(test)]
mod test {
    use glam::IVec2;
    use aoc2023::direction::Direction;
    use super::*;

    #[test]
    fn test_input_1() {
        let input = r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....";

        let world = World::new(input);
        let start_positions = generate_start_positions(&world);
        let result = start_positions.iter()
            .map(|(pos, dir)| walk(&world, *pos, *dir).len())
            .max().unwrap();

        assert_eq!(result, 51)
    }

    #[test]
    fn test_edge_pos() {
        let input = r"..
..";

        let world = World::new(input);
        assert_eq!(world.width, 2);
        assert_eq!(world.height, 2);

        let edge_positions = generate_start_positions(&world);
        let expected_edge_positions = [
            (IVec2::new(0, 0), Direction::East),
            (IVec2::new(0, 0), Direction::South),
            (IVec2::new(0, 1), Direction::East),
            (IVec2::new(0, 1), Direction::North),
            (IVec2::new(1, 0), Direction::West),
            (IVec2::new(1, 0), Direction::South),
            (IVec2::new(1, 1), Direction::West),
            (IVec2::new(1, 1), Direction::North)
        ];

        assert_eq!(
            HashSet::<_>::from_iter(edge_positions),
            HashSet::<_>::from_iter(expected_edge_positions)
        );

    }
}