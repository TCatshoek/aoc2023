#![feature(let_chains)]

use std::hash::{DefaultHasher, Hasher};
use std::slice::Chunks;

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

fn check_fold(buf: &[u64], split: usize) -> bool {
    let iter_a = &mut buf[0..split].iter().rev().copied();
    let iter_b = &mut buf[split..].iter().copied();

    while let Some(a) = iter_a.next() && let Some(b) = iter_b.next() {
        if a != b {
            return false;
        }
    }
    true
}

fn check_folds(buf: &[u64]) -> Option<usize> {
    (1..buf.len()).find(|&i| check_fold(buf, i))
}

fn solve(input: &str) -> usize {
    let worlds = input.split("\n\n").map(World::new);

    let (v, h) = worlds.enumerate()
        .map(|(idx, world)| {
            let verticals = world.iter_cols()
                .map(|col| {
                    let mut hasher = DefaultHasher::new();
                    for c in col {
                        let mut c_b = [0; 4];
                        c.encode_utf8(&mut c_b);
                        hasher.write(&c_b)
                    }
                    hasher.finish()
                })
                .collect::<Vec<u64>>();

            let horizontals = world.iter_rows()
                .map(|row| {
                    let mut hasher = DefaultHasher::new();
                    let tmp = String::from_iter(row.iter());
                    hasher.write(tmp.as_bytes());
                    hasher.finish()
                })
                .collect::<Vec<u64>>();

            match (check_folds(&verticals), check_folds(&horizontals)) {
                (Some(v), Some(h)) => (v, h),
                (Some(v), None) => (v, 0),
                (None, Some(h)) => (0, h),
                (None, None) => panic!("No mirror found for world {}", idx)
            }
        })
        .fold((0, 0), |acc, el| {
            (acc.0 + el.0, acc.1 + el.1)
        });

    v + 100 * h
}

fn main() {
    let input = include_str!("../input1.txt");
    let result = solve(input);
    println!("Result: {}", result);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example_1() {
        let input = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";

        assert_eq!(solve(input), 405)
    }

    #[test]
    fn test_check_fold() {
        assert!(check_fold(&[1, 1, 2, 2, 1, 1], 3));
        assert!(check_fold(&[1, 3, 2, 2, 3, 1], 3));
        assert!(!check_fold(&[1, 3, 2, 2, 3, 4], 3));

        assert!(check_fold(&[1, 2, 2, 1, 1], 2));
        assert!(check_fold(&[1, 3, 2, 2, 3, 1, 10, 9, 8], 3));
        assert!(!check_fold(&[1, 2, 2, 2, 3, 4], 3));
    }
}