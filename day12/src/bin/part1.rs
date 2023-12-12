use std::collections::HashMap;
use itertools::Itertools;

fn count(records: &str, groups: &[usize], cur_count: usize) -> usize {
    let mut cache = HashMap::new();

    fn inner<'a>(records: &'a str, groups: &'a [usize], cur_count: usize, cache: &mut HashMap<(&'a str, &'a [usize], usize), usize>) -> usize {
        if let Some(answer) = cache.get(&(records, groups, cur_count)) {
            return *answer
        }

        let mut num_possibilities = 0;

        // End case
        if records.is_empty() {
            return if groups.len() == 1 && cur_count == groups[0] || records.is_empty() && groups.is_empty() {
                1
            } else {
                0
            }
        }

        // Count in group case
        if records.starts_with('#') || records.starts_with('?') {
            num_possibilities += inner(&records[1..], groups, cur_count + 1, cache);
        }

        // Potential end of group case
        if records.starts_with('.') || records.starts_with('?') {
            // Not currently counting, just consume
            if cur_count == 0 {
                num_possibilities += inner(&records[1..], groups, cur_count, cache);
            }
            // Oh hey it's potentially the end of a group!
            if !groups.is_empty() && cur_count > 0 && cur_count == groups[0] {
                num_possibilities += inner(&records[1..], &groups[1..], 0, cache);
            }
        }

        cache.insert((records, groups, cur_count), num_possibilities);

        num_possibilities
    }

    inner(records, groups, cur_count, &mut cache)
}

fn parse_line(line: &str, part2: bool) -> (String, Vec<usize>) {
    let (records, groups) = line.split_whitespace().collect_tuple().unwrap();

    let mut records = if part2 {
        [records; 5].join("?")
    } else {
        String::from(records)
    };

    if !records.ends_with('.') {
        records.push('.')
    }

    let groups = groups.split(',')
        .map(|el| el.parse().unwrap())
        .collect::<Vec<usize>>();

    let groups = groups.repeat(if part2 {5} else {1}).to_vec();

    (records, groups)
}

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
    fn test_example_1() {
        let input = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";

        let result = input.lines()
            .map(|line| parse_line(line, false))
            .map(|(records, groups)| count(&records, &groups, 0))
            .sum::<usize>();

        assert_eq!(result, 21);
    }

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

    #[test]
    fn test_known() {
        assert_eq!(count("#.#.###", &[1,1,3], 0), 1);
        assert_eq!(count(".#...#....###.", &[1,1,3], 0), 1);
    }


    #[test]
    fn test_unknown() {
        assert_eq!(count("???.###", &[1,1,3], 0), 1);
        assert_eq!(count(".??..??...?##.", &[1,1,3], 0), 4);
        assert_eq!(count("?###????????.", &[3,2,1], 0), 10);
    }
}