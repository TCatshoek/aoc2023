use day12::*;

fn main() {
    let input = include_str!("../input1.txt");

    let result = input.lines()
        .map(|line| parse_line(line, true))
        .map(|(records, groups)| count(&records, &groups, 0))
        .sum::<usize>();

    println!("Result: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_2() {
        let input = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";

        let result = input.lines()
            .map(|line| parse_line(line, true))
            .map(|(records, groups)| count(&records, &groups, 0))
            .sum::<usize>();

        assert_eq!(result, 525152);
    }


}