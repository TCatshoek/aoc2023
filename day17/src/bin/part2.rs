#![feature(let_chains)]

use std::collections::HashMap;
use glam::{IVec2};
use aoc2023::direction::Direction;
use aoc2023::map2d::Map2D;

fn step(world: &Map2D<u32>, mut visited: Map2D<bool>, pos: IVec2, n_steps: u32, loss: u32, direction: Direction, cur_best: &mut u32, cache: &mut HashMap<(IVec2, u32, u32, Direction), u32>) -> u32 {
    if cache.contains_key(&(pos, n_steps, loss, direction)) {
        return *cache.get(&(pos, n_steps, loss, direction)).unwrap();
    }

    let mut possibilities = Vec::new();

    let cur_loss = world.get_v(pos).unwrap();
    visited.set_v(pos, true);

    // At end pos
    if pos == IVec2::new(world.width as i32 - 1, world.height as i32 - 1) && n_steps >= 3 {
        println!("Reached end! {}, cur_best: {}", loss + cur_loss, cur_best);
        if loss + cur_loss < *cur_best {
            *cur_best = loss + cur_loss;
        }
        return loss + cur_loss;
    }

    // Continue in same direction
    if n_steps < 9 {
        let next_pos = pos + direction.as_delta();
        if let Some(has_visited) = visited.get_v(next_pos) && !has_visited {
            let new_loss = loss + cur_loss;
            if new_loss <= *cur_best {
                let same_dir = step(world, visited.clone(), pos + direction.as_delta(), n_steps + 1, loss + cur_loss, direction, cur_best, cache);
                possibilities.push(same_dir)
            }
        }
    }

    // Change direction
    if n_steps >= 3 {
        let directions = [Direction::East, Direction::South, Direction::West, Direction::North];
        directions.iter()
            .filter(|&&d| d != direction && d != direction.opposite())
            .for_each(|&d| {
                let next_pos = pos + d.as_delta();
                if let Some(has_visited) = visited.get_v(next_pos) && !has_visited {
                    let new_loss = loss + cur_loss;
                    if new_loss <= *cur_best {
                        let other_dir = step(world, visited.clone(), pos + d.as_delta(), 0, loss + cur_loss, d, cur_best, cache);
                        possibilities.push(other_dir);
                    }
                }
            });
    }

    let result = *possibilities.iter().min().unwrap_or(&u32::MAX);
    cache.insert((pos, n_steps, loss, direction), result);
    result
}

fn solve(world: &Map2D<u32>) -> u32 {
    let vistited = Map2D::<bool>::from_size(world.width, world.height, false);
    let pos = IVec2::new(0, 0);
    let n_steps = 0;
    let loss = 0;
    let mut cur_best = u32::MAX;
    let mut cache = HashMap::new();

    let start_directions = [Direction::East, Direction::South];
    start_directions.iter()
        .map(|d| step(world, vistited.clone(), pos + d.as_delta(), n_steps + 1, loss, *d, &mut cur_best, &mut cache))
        .min()
        .unwrap()
}

fn main() {
    let input = include_str!("../input1.txt");
    let world = Map2D::<u32>::new(input);
    let result = solve(&world);
    println!("Result: {}", result);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_input_1() {
        let input = "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";

        let world = Map2D::<u32>::new(input);
        let result = solve(&world);
        assert_eq!(result, 94);
    }

    #[test]
    fn test_input_2() {
        let input = "111111111111
999999999991
999999999991
999999999991
999999999991";

        let world = Map2D::<u32>::new(input);
        let result = solve(&world);
        assert_eq!(result, 71);
    }
}