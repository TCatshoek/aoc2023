use std::slice::{Chunks, Iter};

#[derive(Clone, Eq, PartialEq)]
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

    pub fn get(&self, x: usize, y: usize) -> Option<char> {
        match (x, y) {
            (x, y) if self.is_in_bounds(x, y) => {
                self.buf.get(x + y * self.width).copied()
            }
            _ => None
        }
    }

    pub fn set(&mut self, x: usize, y: usize, c: char) {
        if !self.is_in_bounds(x, y) { panic!("out of bounds") }
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

fn slide_rocks(world: &mut World) {
    let mut prev_world = world.clone();
    let mut converged = false;

    while !converged {
        for y in 0..world.height {
            for x in 0..world.width {
                // We can't do anything on the top row
                if y == 0 { continue; }

                let cur = world.get(x, y).unwrap();
                let above = world.get(x, y - 1).unwrap();

                // Roll upwards
                if cur == 'O' && above == '.' {
                    world.set(x, y, '.');
                    world.set(x, y - 1, 'O')
                }
            }
        }

        converged = *world == prev_world;
        prev_world = world.clone();
    }
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


fn main() {
    let input = include_str!("../input1.txt");
    let mut world = World::new(input);
    slide_rocks(&mut world);
    let result = calc_load(&world);

    println!("Result: {}", result);
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
        slide_rocks(&mut world);
        let result = calc_load(&world);

        assert_eq!(result, 136);
    }
}