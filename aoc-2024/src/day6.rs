use std::collections::{HashMap, HashSet};

use grid::Grid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
impl Direction {
    fn turn_right(&self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}
// Note that (0, 0) is considered to be top-left (ie, adding Up or Left will always fail)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Coord {
    x: usize,
    y: usize,
}
impl Coord {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    // Returns None if either coord results in invalid coords (<0 or overflow)
    fn add(&self, dir: Direction) -> Option<Self> {
        let (x, y) = (self.x, self.y);
        match dir {
            Direction::Up => y.checked_sub(1).map(|y| Self { x, y }),
            Direction::Down => y.checked_add(1).map(|y| Self { x, y }),
            Direction::Left => x.checked_sub(1).map(|x| Self { x, y }),
            Direction::Right => x.checked_add(1).map(|x| Self { x, y }),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Space {
    Empty,
    Obstacle,
}
#[derive(Debug, Clone)]
struct Input {
    grid: Grid<Space>,
    guard: Guard,
}
#[derive(Debug, Clone)]
struct Guard {
    position: Coord,
    direction: Direction,
    seen: HashMap<Coord, HashSet<Direction>>,
}
impl Guard {
    fn new(position: Coord, direction: Direction) -> Self {
        Self { position, direction, seen: HashMap::new() }
    }

    // Returns None if the guard would step off the grid
    // Panics if the guard is surrounded by obstacles
    fn step(&mut self, grid: &Grid<Space>) -> Option<Coord> {
        let starting_direction = self.direction;

        // place the current position in the seen map
        self.seen.entry(self.position).or_insert_with(HashSet::new).insert(starting_direction);

        self.position = loop {
            let next = self.position.add(self.direction)?;
            match grid.get(next.y, next.x)? {
                Space::Empty => break next,
                Space::Obstacle => {
                    self.direction = self.direction.turn_right();
                    if self.direction == starting_direction {
                        panic!("Guard is surrounded by obstacles");
                    }
                    continue; // try again
                }
            }
        };
        Some(self.position)
    }
}

#[aoc_generator(day6)]
fn parse(input: &str) -> Input {
    let mut guard = None;
    let grid = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| match c {
                    '.' => Space::Empty,
                    '#' => Space::Obstacle,
                    '^' => {
                        guard = Some(Coord::new(x, y));
                        Space::Empty
                    }
                    _ => unreachable!("Invalid char"),
                })
                .collect()
        })
        .collect::<Vec<_>>()
        .into();
    Input { grid, guard: Guard::new(guard.expect("No guard in map"), Direction::Up) }
}

#[aoc(day6, part1)]
fn part1(input: &Input) -> usize {
    let mut input = input.clone();
    while input.guard.step(&input.grid).is_some() {} // keep stepping until we can't anymore
    input.guard.seen.len()
}

// #[aoc(day6, part2)]
// fn part2(input: &Input) -> u64 {
//     todo!("part 2 is not implemented yet")
// }

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse(INPUT)), 41);
    }
    // #[test]
    // fn test_part2() {
    //     assert_eq!(part2(&parse(INPUT)), _);
    // }
}
