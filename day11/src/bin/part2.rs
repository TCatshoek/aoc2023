use itertools::Itertools;
use day11::*;

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