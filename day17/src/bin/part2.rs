#![feature(let_chains)]

use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use glam::{IVec2};
use rustc_hash::{FxHashMap, FxHashSet};
use aoc2023::direction::Direction;
use aoc2023::map2d::Map2D;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct State {
    loss: u32,
    pos: IVec2,
    direction: Direction,
    n_steps: u32,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.loss.cmp(&self.loss)
            .then_with(|| self.pos.x.cmp(&other.pos.x))
            .then_with(|| self.pos.y.cmp(&other.pos.y))
            .then_with(|| self.direction.cmp(&other.direction))
            .then_with(|| self.n_steps.cmp(&other.n_steps))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn get_possible_turns(direction: Direction) -> Vec<Direction> {
    [Direction::East, Direction::South, Direction::West, Direction::North].iter()
        .copied()
        .filter(|&d| d != direction && d != direction.opposite())
        .collect()
}

fn solve(world: &Map2D<u32>) -> Option<u32> {

    let mut heap = BinaryHeap::new();
    let mut visited = FxHashMap::default();
    // let mut visited = HashMap::new();

    let s1 = State {
        pos: IVec2::new(0, 0),
        direction: Direction::East,
        n_steps: 0,
        loss: 0,
    };

    let s2 = State {
        pos: IVec2::new(0, 0),
        direction: Direction::South,
        n_steps: 0,
        loss: 0,
    };

    heap.push(s1);
    heap.push(s2);

    visited.insert((s1.pos, s1.direction, s1.n_steps), s1.loss);
    visited.insert((s2.pos, s2.direction, s2.n_steps), s2.loss);

    while let Some(state) = heap.pop() {

        if state.pos == IVec2::new(world.width as i32 - 1, world.height as i32 - 1) && state.n_steps >= 4{
            return Some(state.loss)
        }

        visited.insert((state.pos, state.direction, state.n_steps), state.loss);

        // Move straight ahead
        if state.n_steps < 10 {

            let next_pos = state.pos + state.direction.as_delta();
            if let Some(cost) = world.get_v(next_pos) {

                let next_key = (next_pos, state.direction, state.n_steps + 1);
                let next_cost = state.loss + cost;

                if !visited.contains_key(&next_key) {
                    heap.push(State {
                        pos: next_pos,
                        direction: state.direction,
                        n_steps: state.n_steps + 1,
                        loss: next_cost,
                    });
                    visited.insert(next_key, next_cost);
                }

            }
        }

        // Take a turn
        if state.n_steps >= 4 {

            for direction in get_possible_turns(state.direction) {

                let next_pos = state.pos + direction.as_delta();
                if let Some(cost) = world.get_v(next_pos) {

                    let next_key = (next_pos, direction, 0);
                    let next_cost = state.loss + cost;

                    if !visited.contains_key(&next_key) {
                        heap.push(State {
                            pos: next_pos,
                            direction,
                            n_steps: 1,
                            loss: next_cost,
                        });
                        visited.insert(next_key, next_cost);
                    }

                }
            }
        }
    };

    None
}

fn main() {
    let input = include_str!("../input1.txt");
    let world = Map2D::<u32>::new(input);
    let result = solve(&world).unwrap();
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
        assert_eq!(result, Some(94));
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
        assert_eq!(result, Some(71));
    }
}