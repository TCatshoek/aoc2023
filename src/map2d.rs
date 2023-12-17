use std::fmt::{Debug, Display, Formatter};
use std::slice::{Chunks, Iter};
use glam::{IVec2};

#[derive(Clone, Eq, PartialEq)]
pub struct Map2D<T> {
    buf: Vec<T>,
    pub width: usize,
    pub height: usize,
}

impl Map2D<char> {
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
}

impl Map2D<u32> {
    pub fn new(input: &str) -> Self {
        let mut buf = Vec::new();
        let mut width = 0;
        let mut height = 0;

        for (y, line) in input.lines().enumerate() {
            height = y + 1;
            for (x, c) in line.chars().enumerate() {
                width = x + 1;
                buf.push(c.to_digit(10).unwrap());
            }
        }

        Self {
            buf,
            width,
            height,
        }
    }
}

impl<T> Map2D<T> where T: Copy {
    pub fn from_size(width: usize, height: usize, val: T) -> Self {
        let buf = vec![val; width * height];
        Self {
            buf,
            width,
            height,
        }
    }

    pub fn is_in_bounds(&self, x: usize, y: usize) -> bool {
        x < self.width && y < self.height
    }

    pub fn get(&self, x: usize, y: usize) -> Option<T> {
        match (x, y) {
            (x, y) if self.is_in_bounds(x, y) => {
                self.buf.get(x + y * self.width).copied()
            }
            _ => None
        }
    }

    pub fn get_v(&self, pos: IVec2) -> Option<T> {
        self.get(pos.x as usize, pos.y as usize)
    }

    pub fn set(&mut self, x: usize, y: usize, c: T) {
        if !self.is_in_bounds(x, y) { panic!("out of bounds") }
        self.buf[x + y * self.width] = c;
    }

    pub fn set_v(&mut self, pos: IVec2, c: T) {
        self.set(pos.x as usize, pos.y as usize, c);
    }

    pub fn iter_rows(&self) -> Chunks<'_, T> {
        self.buf.chunks(self.width)
    }

    pub fn iter_row(&self, row: usize) -> Iter<'_, T> {
        self.buf[row * self.width..row * self.width + self.width].into_iter()
    }

    pub fn iter_cols(&'_ self) -> impl Iterator<Item=impl Iterator<Item=T> + '_> {
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

impl<T> Debug for Map2D<T> where T: Display, T: Copy {
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