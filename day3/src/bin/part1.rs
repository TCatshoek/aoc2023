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

    fn check_positions(&self, positions: &[(usize, usize)]) -> bool {
        for (x, y) in positions {

            let offsets = [
                (1 , 1),
                (-1, -1),
                (1, -1),
                (-1, 1),
                (0 , 1),
                (0 , -1),
                (1 , 0),
                (-1 , 0)
            ];

            let to_check = offsets.iter()
                .filter_map(|(v, h)| {
                    match (x.checked_add_signed(*v), y.checked_add_signed(*h)) {
                        (Some(x_new), Some(y_new)) => Some((x_new, y_new)),
                        _ => None
                    }
                });

            for (cur_x, cur_y) in to_check {
                if let Some(cur_char) = self.get(cur_x, cur_y) {
                    if !cur_char.is_numeric() && *cur_char != '.' {
                        return true
                    }
                }
            }
        }

        false
    }

    fn find_numbers(&self) -> Vec<u32> {
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

        all_numbers.iter()
            .filter_map(|(n, p)| match self.check_positions(p) {
                true => Some(n),
                false => None
            })
            .map(|n| {
                n.parse::<u32>().unwrap()
            })
            .collect::<Vec<u32>>()
    }
}


fn main() {
    let input = include_str!("../input1.txt");

    let map = Map2D::from_str(input);

    let result = map.find_numbers().iter().sum::<u32>();

    println!("Result is: {}", result);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_number() {
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

        let map = Map2D::from_str(input);

        let expected_results = vec![
            Some((String::from("467"), vec![(0, 0), (1, 0), (2, 0)])),
            Some((String::from("664"), vec![(1, 9), (2, 9), (3, 9)])),
            Some((String::from("617"), vec![(0, 4), (1, 4), (2, 4)])),
            None,
        ];

        assert_eq!(map.get_number(0, 0), expected_results[0]);
        assert_eq!(map.get_number(1, 9), expected_results[1]);
        assert_eq!(map.get_number(0, 4), expected_results[2]);
        assert_eq!(map.get_number(0, 1), expected_results[3]);
    }

    #[test]
    fn find_numbers() {
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

        let map = Map2D::from_str(input);

        let numbers = map.find_numbers();

        assert_eq!(numbers.iter().sum::<u32>(), 4361)
    }
}
