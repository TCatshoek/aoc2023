use std::slice::Chunks;
use glam::U64Vec2;

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

    pub fn iter_cols(&'_ self) -> impl Iterator<Item = impl Iterator<Item = char> + '_> {
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

pub fn find_galaxies(world: &World, expansion_factor: usize) -> Vec<U64Vec2> {
    if expansion_factor < 1 {
        panic!("Expansion factor must be > 0");
    }

    // Find empty row and column idxes
    let empty_rows = world.iter_rows()
        .enumerate()
        .filter_map(|(idx, row)| {
            if row.iter().all(|el| *el == '.') {
                Some(idx)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    let empty_cols = world.iter_cols()
        .enumerate()
        .filter_map(|(idx, mut col)| {
            if col.all(|el| el == '.') {
                Some(idx)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    let mut galaxies = Vec::new();

    let mut expansion_y = 0;
    for y in 0..world.height {
        let mut expansion_x = 0;
        for x in 0..world.width {
            match world.get(x, y) {
                None => {}
                Some('.') => {}
                Some('#') => {galaxies.push(U64Vec2::new((x + expansion_x) as u64, (y + expansion_y) as u64))}
                Some(c) => unimplemented!("{}", c)
            };

            if empty_cols.contains(&x) { expansion_x += expansion_factor - 1}
        }
        if empty_rows.contains(&y) { expansion_y += expansion_factor - 1}
    }

    galaxies
}

pub fn manhattan_distance(a: U64Vec2, b: U64Vec2) -> u64 {
    a.x.abs_diff(b.x) + a.y.abs_diff(b.y)
}


#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_1: &str = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

    #[test]
    fn test_iter_rows() {
        let input = INPUT_1;
        let map = World::new(input);
        let mut rows = map.iter_rows();

        assert_eq!(rows.next().unwrap(), ['.', '.', '.', '#', '.', '.', '.', '.', '.', '.']);
        assert_eq!(rows.next().unwrap(), ['.', '.', '.', '.', '.', '.', '.', '#', '.', '.']);
        assert_eq!(rows.next().unwrap(), ['#', '.', '.', '.', '.', '.', '.', '.', '.', '.']);
        assert_eq!(rows.next().unwrap(), ['.', '.', '.', '.', '.', '.', '.', '.', '.', '.']);
        assert_eq!(rows.next().unwrap(), ['.', '.', '.', '.', '.', '.', '#', '.', '.', '.']);
        assert_eq!(rows.next().unwrap(), ['.', '#', '.', '.', '.', '.', '.', '.', '.', '.']);
        assert_eq!(rows.next().unwrap(), ['.', '.', '.', '.', '.', '.', '.', '.', '.', '#']);
        assert_eq!(rows.next().unwrap(), ['.', '.', '.', '.', '.', '.', '.', '.', '.', '.']);
        assert_eq!(rows.next().unwrap(), ['.', '.', '.', '.', '.', '.', '.', '#', '.', '.']);
        assert_eq!(rows.next().unwrap(), ['#', '.', '.', '.', '#', '.', '.', '.', '.', '.']);
        assert_eq!(rows.next(), None);
    }

    #[test]
    fn test_iter_cols() {
        let input = INPUT_1;
        let map = World::new(input);
        let mut cols = map.iter_cols();

        assert_eq!(cols.next().unwrap().collect::<Vec<_>>(), ['.', '.', '#', '.', '.', '.', '.', '.', '.', '#']);
        assert_eq!(cols.next().unwrap().collect::<Vec<_>>(), ['.', '.', '.', '.', '.', '#', '.', '.', '.', '.']);
        assert_eq!(cols.next().unwrap().collect::<Vec<_>>(), ['.', '.', '.', '.', '.', '.', '.', '.', '.', '.']);
        assert_eq!(cols.next().unwrap().collect::<Vec<_>>(), ['#', '.', '.', '.', '.', '.', '.', '.', '.', '.']);
        assert_eq!(cols.next().unwrap().collect::<Vec<_>>(), ['.', '.', '.', '.', '.', '.', '.', '.', '.', '#']);
        assert_eq!(cols.next().unwrap().collect::<Vec<_>>(), ['.', '.', '.', '.', '.', '.', '.', '.', '.', '.']);
        assert_eq!(cols.next().unwrap().collect::<Vec<_>>(), ['.', '.', '.', '.', '#', '.', '.', '.', '.', '.']);
        assert_eq!(cols.next().unwrap().collect::<Vec<_>>(), ['.', '#', '.', '.', '.', '.', '.', '.', '#', '.']);
        assert_eq!(cols.next().unwrap().collect::<Vec<_>>(), ['.', '.', '.', '.', '.', '.', '.', '.', '.', '.']);
        assert_eq!(cols.next().unwrap().collect::<Vec<_>>(), ['.', '.', '.', '.', '.', '.', '#', '.', '.', '.']);
        assert!(cols.next().is_none());
    }

    #[test]
    fn test_find_galaxies_1() {
        let input = INPUT_1;
        let map = World::new(input);
        let galaxies = find_galaxies(&map, 2);

        let expected = [(4, 0), (9, 1), (0, 2), (8, 5), (1, 6), (12, 7), (9, 10), (0, 11), (5, 11)].iter()
            .map(|(x, y)| U64Vec2::new(*x, *y))
            .collect::<Vec<_>>();

        assert_eq!(expected, galaxies);
    }

    #[test]
    fn test_manhattan_distance() {
        assert_eq!(manhattan_distance(U64Vec2::new(1, 6), U64Vec2::new(5, 11)), 9);
        assert_eq!(manhattan_distance(U64Vec2::new(5, 11), U64Vec2::new(1, 6)), 9);
    }
}