use std::fmt::{Debug, Formatter};
use std::hash::{Hash};
use std::slice::{Chunks, Iter};
use std::time::Instant;

#[derive(Clone, Eq, PartialEq, Hash)]
pub struct World {
    buf: Vec<char>,
    width: usize,
    height: usize,
}

impl World {
    pub fn new(input: &str) -> Self {
        let mut buf = Vec::new();
        let mut width = 0;
        let mut height = 0;

        for (y, line) in input.lines().enumerate() {
            height = y + 1;
            for (x, c) in line.chars().enumerate() {
                width = x + 1;
                buf.push(c);
            }
        }

        Self {
            buf,
            width,
            height,
        }
    }

    pub fn is_in_bounds(&self, x: usize, y: usize) -> bool {
        x < self.width && y < self.height
    }

    #[inline]
    pub fn get(&self, x: usize, y: usize) -> char {
        self.buf[x + y * self.width]
    }

    #[inline]
    pub fn set(&mut self, x: usize, y: usize, c: char) {
        // if !self.is_in_bounds(x, y) { panic!("out of bounds") }
        self.buf[x + y * self.width] = c;
    }

    pub fn iter_rows(&self) -> Chunks<'_, char> {
        self.buf.chunks(self.width)
    }

    pub fn iter_row(&self, row: usize) -> Iter<'_, char> {
        self.buf[row * self.width..row * self.width + self.width].into_iter()
    }

    pub fn iter_cols(&'_ self) -> impl Iterator<Item=impl Iterator<Item=char> + '_> {
        (0..self.width).map(move |col_start| {
            let mut next_index = col_start;
            std::iter::from_fn(move || {
                if next_index < self.buf.len() {
                    let current = self.buf[next_index];
                    next_index += self.width;
                    Some(current)
                } else {
                    None
                }
            })
        })
    }
}

impl Debug for World {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for row in self.iter_rows() {
            for c in row {
                write!(f, "{}", c)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(Eq, PartialEq)]
enum Direction {
    North,
    West,
    South,
    East,
}

fn slide_rocks(world: &mut World, direction: Direction) {
    let mut converged = false;

    let delta = match direction {
        Direction::North => (0, -1),
        Direction::West => (-1, 0),
        Direction::South => (0, 1),
        Direction::East => (1, 0)
    };

    let x_range_normal = (0..world.width).collect::<Vec<_>>();
    let x_range_rev = (0..world.width).collect::<Vec<_>>();
    let y_range_normal = (0..world.height).collect::<Vec<_>>();
    let y_range_rev = (0..world.height).rev().collect::<Vec<_>>();

    let (range_x, range_y) = match direction {
        Direction::North => (x_range_normal, y_range_normal),
        Direction::West => (x_range_normal, y_range_normal),
        Direction::South => (x_range_normal, y_range_rev),
        Direction::East => (x_range_rev, y_range_normal)
    };

    while !converged {
        let mut n_moved = 0;
        for y in range_y.iter().copied() {
            for x in range_x.iter().copied() {

                // Edge checks
                match direction {
                    Direction::North if y == 0 => continue,
                    Direction::West if x == 0 => continue,
                    Direction::South if y == world.height - 1 => continue,
                    Direction::East if x == world.width - 1 => continue,
                    _ => ()
                }

                let (next_x, next_y) = (
                    (x as i32 + delta.0) as usize,
                    (y as i32 + delta.1) as usize
                );

                let cur = world.get(x, y);
                let next = world.get(next_x, next_y);

                // Roll
                if cur == 'O' && next == '.' {
                    n_moved += 1;
                    world.set(x, y, '.');
                    world.set(next_x, next_y, 'O')
                }
            }
        }
        converged = n_moved == 0;
    }
}

fn do_cycle(world: &mut World) {
    slide_rocks(world, Direction::North);
    slide_rocks(world, Direction::West);
    slide_rocks(world, Direction::South);
    slide_rocks(world, Direction::East);
}

fn calc_load(world: &World) -> usize {
    world.iter_rows().enumerate()
        .map(|(y, row)| {
            let multiplier = world.height - y;
            row.iter()
                .filter_map(|c| match c {
                    'O' => Some(multiplier),
                    _ => None
                })
                .sum::<usize>()
        })
        .sum()
}

fn find_period(signal: &[usize], n: usize) -> usize {
    let mut results = Vec::new();
    for offset in 0..n {
        let matches = (0..(signal.len() - offset))
            .map(|i| signal[i] == signal[i + offset])
            .all(|x| x);
        if matches {results.push(offset)}
    }
    results[1]
}

fn solve(world: &mut World, n_cycles: usize) -> usize {
    let n_settle = 160;
    let n_capture = 30;

    // Run for a few cycles to settle
    for _cycle in 0..n_settle {
        do_cycle(world);
    }

    // Capture sequence
    let sequence = (0..n_capture)
        .map(|_| {
            do_cycle(world);
            calc_load(world)
        })
        .collect::<Vec<_>>();

    let period = find_period(&sequence, n_capture);

    let leftover_cycles = n_cycles - n_capture - n_settle;
    let idx = leftover_cycles % period;

    for _cycle in 0..idx {
        do_cycle(world);
    }

    calc_load(world)
}


fn main() {
    let input = include_str!("../input1.txt");
    let mut world = World::new(input);
    let start = Instant::now();
    let result = solve(&mut world, 1000000000);
    let end = start.elapsed();

    println!("Result: {}", result);
    println!("Took: {:?}", end);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_input_1() {
        let input = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";

        let mut world = World::new(input);
        let result = solve(&mut world, 1000000000);

        assert_eq!(result, 64);
    }
}