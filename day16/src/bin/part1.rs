use glam::IVec2;
use aoc2023::direction::Direction;
use aoc2023::world::World;
use day16::walk;

fn main() {
    let input = include_str!("../input1.txt");
    let world = World::new(input);
    let visited =  walk(&world, IVec2::new(0, 0), Direction::East);
    println!("Result: {}", visited.len());
}

#[cfg(test)]
mod test {
    use glam::IVec2;
    use aoc2023::direction::Direction;
    use super::*;

    #[test]
    fn test_input_1() {
        let input = r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....";

        let world = World::new(input);
        let visited = walk(&world, IVec2::new(0, 0), Direction::East);

        let mut world_visited = World::from_size(world.width, world.height, '.');
        for pos in &visited {
            world_visited.set(pos.x as usize, pos.y as usize, '#');
        }
        println!("{:?}", world_visited);

        assert_eq!(visited.len(), 46)
    }
}