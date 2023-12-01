fn solve(input: &str) -> i32 {
    return input.lines()
        .map(|line| {
            let mut first = None;
            let mut last = None;
            for char in line.chars() {
                match char {
                    '0'..='9' => {
                        if first == None {
                            first = Some(char);
                        }
                        last = Some(char);
                    }
                    _ => {}
                }
            }
            let s: String = [first.unwrap(), last.unwrap()].iter().collect();
            return s.parse::<i32>().unwrap();
        }).sum();
}

fn main() {
    let input = include_str!("../input1.txt");
    let result = solve(input);
    println!("Result: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

        let result = solve(input);
        assert_eq!(result, 142);
    }

    #[test]
    fn do_char_ranges_do_what_i_think() {
        for n in 0 ..= 9 {
            let c = char::from_digit(n, 10).unwrap();
            let is_in_range = match c {
                '0'..='9' => true,
                _ => false
            };
            assert!(is_in_range)
        };
    }
}