use std::fmt::{Debug, Formatter};
use std::slice::{Chunks, Iter};

#[derive(Clone, Eq, PartialEq)]
pub struct World {
    buf: Vec<char>,
    pub width: usize,
    pub height: usize,
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

    pub fn from_size(width: usize, height: usize, val: char) -> Self {
        let buf = vec![val; width * height];
        Self {
            buf,
            width,
            height
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