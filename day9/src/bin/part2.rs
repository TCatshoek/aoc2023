use std::time::Instant;
use day9::*;

fn main() {
    let input = include_str!("../input1.txt");

    let start = Instant::now();

    let result = input.lines()
        .map(parse_line)
        .map(|mut line| {
            line.reverse();
            extrapolate(&line)
        })
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
            .map(|mut line| {
                line.reverse();
                extrapolate(&line)
            })
            .sum::<Num>();

        assert_eq!(result, 2);
    }

    #[test]
    fn test_input_1() {
        let input = "10 13 16 21 30 45";
        let mut parsed = parse_line(input);
        parsed.reverse();
        let result = extrapolate(&parsed);

        assert_eq!(result, 5);
    }
}