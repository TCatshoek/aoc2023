use aho_corasick::{AhoCorasick, MatchKind};



fn solve(input: &str) -> i32 {

    let patterns = &["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    let replacements = &["1", "2", "3", "4", "5", "6", "7", "8", "9"];
    let ac = AhoCorasick::builder()
        .match_kind(MatchKind::LeftmostFirst)
        .build(patterns).unwrap();

    return input.lines()
        .map(|line| {
            let l = ac.replace_all(line, replacements);
            println!("{}", l);
            l
        })
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
        let input = "eightwofhfhdj9";

        let patterns = &["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
        let replacements = &["1", "2", "3", "4", "5", "6", "7", "8", "9"];
        let ac = AhoCorasick::builder()
            .match_kind(MatchKind::LeftmostFirst)
            .build(patterns).unwrap();

        assert_eq!(ac.replace_all(input, replacements), "8wofhfhdj9");
    }



}


