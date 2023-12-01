
fn solve(input: &str) -> i32 {
    
    let mapping = vec![
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ];
    
    return input.lines()
        .map(|line| {
            let mut first = None;
            let mut last = None;

            for i in 0..line.len() {
                let line_rest = &line[i..];

                for (from, to) in &mapping {
                    if line_rest.starts_with(from) {
                        if first == None {
                            first = Some(to);
                        }
                        last = Some(to);
                        break;
                    }
                }
            }

            return first.unwrap() * 10 + last.unwrap();
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
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

        let result = solve(input);
        assert_eq!(result, 281);
    }

    #[test]
    fn it_works_2() {
        let input = "sevenfivezcbqhhsntfbpkz3one3
7onenine11
glghbpvrbfqsfvnvpxlb5vvqkt8eight
rvhqbhlmonenine96
npgvjlkdxmsevensixxrkhlt5
mhqf52scsgsxtwovcpq
zrvznqpcjhtsghfb7
five8threeonedl
8threefiveknqhmm4five2";

        let result = solve(input);
        assert_eq!(result, 555);
    }

    #[test]
    fn replacer() {
        let input = "eightwo";
        assert_eq!(solve(input), 82);
    }



}


