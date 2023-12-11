use std::slice::Chunks;
use glam::{IVec2, U64Vec2};
use itertools::Itertools;

struct World {
    buf: Vec<char>,
    width: usize,
    height: usize,
}

impl World {
    fn new(input: &str) -> Self {
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

    fn is_in_bounds(&self, x: usize, y: usize) -> bool {
        x < self.width && y < self.height
    }

    fn get(&self, x: usize, y: usize) -> Option<char> {
        match (x, y) {
            (x, y) if self.is_in_bounds(x, y) => {
                self.buf.get(x + y * self.width).copied()
            }
            _ => None
        }
    }

    fn set(&mut self, x: usize, y: usize, c: char) {
        if !self.is_in_bounds(x, y) { panic!("out of bounds") }
        self.buf[x + y * self.width] = c;
    }

    fn iter_rows(&self) -> Chunks<'_, char> {
        self.buf.chunks(self.width)
    }

    fn iter_cols<'a>(&'a self) -> impl Iterator<Item = impl Iterator<Item = char> + 'a> {
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

fn find_galaxies(world: &World, expansion_factor: usize) -> Vec<U64Vec2> {
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

    println!("Empty rows: {:?}", empty_rows);
    println!("Empty cols: {:?}", empty_cols);

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

fn manhattan_distance(a: U64Vec2, b: U64Vec2) -> u64 {
    a.x.abs_diff(b.x) + a.y.abs_diff(b.y)
}

fn main() {
    let input = include_str!("../input1.txt");
    let world = World::new(input);
    let galaxies = find_galaxies(&world, 1000000);

    let result: u64 = galaxies.iter()
        .combinations_with_replacement(2)
        .map(|combination| manhattan_distance(*combination[0], *combination[1]))
        .sum();

    println!("Result: {}", result);
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;
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
        let galaxies = find_galaxies(&map, 1);

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

    #[test]
    fn test_input_1 () {
        let input = INPUT_1;
        let world = World::new(input);
        let galaxies = find_galaxies(&world, 1);

        let result: u64 = galaxies.iter()
            .combinations_with_replacement(2)
            .map(|combination| manhattan_distance(*combination[0], *combination[1]))
            .sum();

        assert_eq!(result, 374);
    }

    #[test]
    fn test_input_1_expansion_10 () {
        let input = INPUT_1;
        let world = World::new(input);
        let galaxies = find_galaxies(&world, 10);

        let result: u64 = galaxies.iter()
            .combinations_with_replacement(2)
            .map(|combination| manhattan_distance(*combination[0], *combination[1]))
            .sum();

        assert_eq!(result, 1030);
    }

    #[test]
    fn test_input_1_expansion_100 () {
        let input = INPUT_1;
        let world = World::new(input);
        let galaxies = find_galaxies(&world, 100);

        let result: u64 = galaxies.iter()
            .combinations_with_replacement(2)
            .map(|combination| manhattan_distance(*combination[0], *combination[1]))
            .sum();

        assert_eq!(result, 8410);
    }
}