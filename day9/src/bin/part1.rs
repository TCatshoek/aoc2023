use std::time::Instant;
use day9::*;

fn main() {
    let input = include_str!("../input1.txt");

    let start = Instant::now();

    let result = input.lines()
        .map(parse_line)
        .map(|line| extrapolate(&line))
        .sum::<Num>();

    let duration = start.elapsed();

    println!("Result: {}", result);
    println!("Time: {:?}", duration);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

        let result = input.lines()
            .map(parse_line)
            .map(|line| extrapolate(&line))
            .sum::<Num>();

        assert_eq!(result, 114);
    }

    #[test]
    fn test_input_1() {
        let input = "0 3 6 9 12 15";
        let parsed = parse_line(input);
        let result = extrapolate(&parsed);

        assert_eq!(result, 18);
    }

    #[test]
    fn test_input_2() {
        let input = "1 3 6 10 15 21";
        let parsed = parse_line(input);
        let result = extrapolate(&parsed);

        assert_eq!(result, 28);
    }

    #[test]
    fn test_input_3() {
        let input = "10 13 16 21 30 45";
        let parsed = parse_line(input);
        let result = extrapolate(&parsed);

        assert_eq!(result, 68);
    }

    #[test]
    fn test_with_negative() {
        let input = "-6 -3 0 3 6 9 12 15";
        let parsed = parse_line(input);
        let result = extrapolate(&parsed);

        assert_eq!(result, 18);
    }

    #[test]
    fn test_big() {
        let input = "1 6 8 2 -10 -6 67 305 879 2127 4775 10402 22342 47347 98546 200559 398101 770084 1452146 2671762 4801686";
        let mut parsed = parse_line(input);

        // println!("{:?}", parsed);
        while !all_zeroes(&parsed) {
            calc_next(&mut parsed);
            // println!("{:?}", parsed);
        }

        assert!(!parsed.is_empty())
    }
}