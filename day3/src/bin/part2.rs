use std::collections::HashSet;
use std::fmt;

struct Map2D<T> {
    buf: Vec<T>,
    width: usize,
    height: usize,
}

impl<T> Map2D<T> where T: Clone {
    fn new(width: usize, height: usize, val: T) -> Self {
        Self {
            width,
            height,
            buf: vec![val; width * height],
        }
    }
}

impl<T> Map2D<T> {
    fn get(&self, x: usize, y: usize) -> Option<&T> {
        if x >= self.width || y >= self.height {
            return None;
        }
        self.buf.get(x + y * self.width)
    }

    fn set(&mut self, x: usize, y: usize, val: T) {
        if x >= self.width || y >= self.height {
            panic!("Set out of bounds");
        }
        self.buf[x + y * self.width] = val;
    }
}

impl<T> fmt::Display for Map2D<T> where T: fmt::Display {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                write!(f, "{} ", self.get(x, y).unwrap())?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Map2D<char> {
    fn from_str(input: &str) -> Self {
        let mut buf = Vec::new();
        let mut width = input.lines().next().unwrap().len();
        let mut height = 0;

        for line in input.lines() {
            height += 1;
            for c in line.chars() {
                buf.push(c);
            }
        }

        Self {
            buf,
            width,
            height,
        }
    }

    // Assumes (x, y) is the first digit of the number
    fn get_number(&self, x: usize, y: usize) -> Option<(String, Vec<(usize, usize)>)> {
        let mut cur_x = x;
        let start_idx = x + y * self.width;
        let mut end_idx = start_idx;

        if !self.buf[start_idx].is_numeric() { return None; }

        // Keep track of number positions
        let mut positions = Vec::new();

        let mut iter = self.buf.iter().skip(start_idx);
        while iter.next().is_some_and(|x| x.is_numeric()) && cur_x < self.width {
            positions.push((cur_x, y));
            cur_x += 1;
            end_idx += 1;
        }

        let num_str: String = self.buf.as_slice()[start_idx..end_idx].iter().collect();

        Some((num_str, positions))
    }

    fn get_check_positions(&self, positions: &[(usize, usize)]) -> Vec<(usize, usize)> {
        let offsets = [
            (1, 1),
            (-1, -1),
            (1, -1),
            (-1, 1),
            (0, 1),
            (0, -1),
            (1, 0),
            (-1, 0)
        ];

        positions.iter()
            .flat_map(|(x, y)| {
                offsets.iter()
                    .filter_map(|(v, h)| {
                        match (x.checked_add_signed(*v), y.checked_add_signed(*h)) {
                            (Some(x_new), Some(y_new)) => Some((x_new, y_new)),
                            _ => None
                        }
                    })
            })
            .collect::<HashSet<(usize, usize)>>()
            .difference(&HashSet::from_iter(positions.iter().copied()))
            .copied()
            .collect()
    }

    fn find_numbers(&self) -> Vec<(String, Vec<(usize, usize)>)> {
        let mut all_numbers = Vec::new();

        for y in 0..self.height {
            let mut x = 0;
            while x < self.width {
                if let Some(found) = self.get_number(x, y) {
                    x += &found.0.len();
                    all_numbers.push(found);
                } else {
                    x += 1;
                }
            }
        }

        all_numbers
    }

    fn find_all(&self, c: char) -> Vec<(usize, usize)> {
        let mut positions = Vec::new();

        for y in 0..self.height {
            let mut x = 0;
            while x < self.width {
                if let Some(cur_c) = self.get(x, y) {
                    if *cur_c == c {
                        positions.push((x, y))
                    }
                }
                x += 1;
            }
        }

        positions
    }
}

fn calc_gear_scores(input: &str) -> u32 {
    let map = Map2D::from_str(input);

    // Find all numbers on the map
    let numbers = map.find_numbers();

    // Create an index mapping for all the numbers on the map
    let mut pos_num_idx_map = Map2D::new(map.width, map.height, 0);
    for (idx, (n, positions)) in numbers.iter().enumerate() {
        for (x, y) in positions {
            pos_num_idx_map.set(*x, *y, idx + 1);
        }
    }

    // Find all * symbols, and check if they are valid gears
    let potential_gear_positions = map.find_all('*');

    // For each * symbol
    potential_gear_positions.iter()
        // Lookup the positions in the map to check for each '*'
        .map(|pos| (pos, map.get_check_positions(&[*pos])))
        .filter_map(|(pos, check_positions)| {

            // For each gear, find the array indices of the adjacent numbers
            let adjecent_number_idxes = check_positions.iter().copied()
                .filter_map(|(x, y)| match pos_num_idx_map.get(x, y).copied() {
                    Some(idx) => if idx > 0 { Some(idx - 1) } else { None },
                    None => None
                })
                .collect::<HashSet<usize>>();

            // If the gear has exactly two adjacent numbers, multiply them and return result
            match adjecent_number_idxes.len() {
                2 => Some(adjecent_number_idxes.iter()
                    .map(|idx| numbers.get(*idx).unwrap().0.parse::<u32>().unwrap())
                    .product::<u32>()),
                _ => None
            }
        })
        .sum()
}

fn main() {
    let input = include_str!("../input1.txt");

    let gear_scores = calc_gear_scores(input);

    println!("Result is: {}", gear_scores);
}


#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn find_valid_gears() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

        let gear_scores = calc_gear_scores(input);

        assert_eq!(gear_scores, 467835)
    }
}
