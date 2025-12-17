use geo::prelude::*;
use geo::{Point, Polygon, Rect};
use itertools::Itertools;

type Input = Vec<Point<u64>>;
#[aoc_generator(day9)]
fn parse(input: &str) -> Input {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.split_once(',').unwrap())
        .map(|(x, y)| Point::new(x.parse().unwrap(), y.parse().unwrap()))
        .collect::<Vec<_>>()
}

fn area_of_corners([a, b]: [&Point<u64>; 2]) -> u64 {
    // Plus one because the end point is included in the distance
    (1 + a.x().abs_diff(b.x())) * (1 + a.y().abs_diff(b.y()))
}

#[aoc(day9, part1)]
fn part1(input: &Input) -> u64 {
    input.iter().array_combinations().map(area_of_corners).max().unwrap()
}

#[aoc(day9, part2)]
fn part2(input: &Input) -> u64 {
    // This is a bit of a brute force approach, but it finishes in less than a minute, so it's fine
    let input_f64 =
        input.iter().map(|p| Point::new(p.x() as f64, p.y() as f64)).collect::<Vec<_>>();
    let p = Polygon::new(input_f64.into(), vec![]);
    // Find each rectangle and check if it's inside the polygon
    input
        .iter()
        .array_combinations()
        .filter(|[a, b]| {
            // hack to make Polygon::contains work
            let a = Point::new(a.x() as f64, a.y() as f64);
            let b = Point::new(b.x() as f64, b.y() as f64);
            p.contains(&Rect::new(a, b))
        })
        .map(area_of_corners)
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";
    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse(INPUT)), 50);
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2(&parse(INPUT)), 24);
    }
}
