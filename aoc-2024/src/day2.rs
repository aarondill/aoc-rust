#[derive(Debug, Clone, PartialEq, Eq)]
struct Report(pub Vec<u64>);
type Input = Vec<Report>;
#[aoc_generator(day2)]
fn parse(input: &str) -> Input {
    input
        .lines()
        .map(|line| Report(line.split_whitespace().map(|s| s.parse().unwrap()).collect()))
        .collect()
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Increasing,
    Decreasing,
}
impl Direction {
    fn from(a: u64, b: u64) -> Self {
        if a < b { Direction::Increasing } else { Direction::Decreasing }
    }

    fn check(&self, a: u64, b: u64) -> bool {
        match self {
            Direction::Increasing => a <= b,
            Direction::Decreasing => a >= b,
        }
    }
}

#[aoc(day2, part1)]
fn part1(input: &Input) -> u64 {
    input
        .iter()
        .filter(|r| {
            let direction = Direction::from(r.0[0], r.0[1]);
            r.0.array_windows().all(|&[a, b]| {
                let diff = a.abs_diff(b);
                direction.check(a, b) && diff >= 1 && diff <= 3
            })
        })
        .count() as u64
}

// #[aoc(day2, part2)]
// fn part2(input: &Input) -> u64 {
//     todo!("part 2 is not implemented yet")
// }

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse(INPUT)), 2);
    }
    // #[test]
    // fn test_part2() {
    //     assert_eq!(part2(&parse(INPUT)), 4);
    // }
}
