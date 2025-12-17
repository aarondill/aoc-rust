#![allow(unused)] // I never finished this
use grid::Grid;
use itertools::Itertools;

// True in the places where the gift occupies
#[derive(Debug, Clone, PartialEq, Eq, Default)]
struct Gift([[bool; 3]; 3]);
#[derive(Debug, Clone, PartialEq, Eq)]
struct Region {
    // n times the gift
    gifts: Vec<(u64, Gift)>,
    size: (usize, usize),
}
type Input = Vec<Region>;
#[aoc_generator(day12)]
fn parse(input: &str) -> Input {
    let mut gifts = Vec::new();
    let mut lines = input.lines().filter(|l| !l.is_empty()).peekable();
    while let Some(line) = lines.next() {
        debug_assert!(line[..line.len() - 1].parse::<usize>().is_ok());
        let gift = lines
            .next_array::<3>()
            .unwrap()
            .map(|i| i.chars().map(|c| c == '#').collect_array().unwrap());
        gifts.push(Gift(gift));

        // The regions have values after the colon
        let next = lines.peek().unwrap();
        let (_, b) = next.split_once(':').unwrap();
        if !b.trim().is_empty() {
            break;
        }
    }
    let gifts = gifts;

    lines
        .map(|line| {
            let (size, gifts_nums) = line.split_once(':').unwrap();
            let size =
                size.split('x').map(|s| s.parse::<usize>().unwrap()).collect_tuple().unwrap();
            let gifts_nums =
                gifts_nums.split_whitespace().map(|s| s.parse().unwrap()).collect::<Vec<_>>();
            let gifts =
                gifts_nums.into_iter().enumerate().map(|(i, n)| (n, gifts[i].clone())).collect();
            Region { gifts, size }
        })
        .collect()
}

#[aoc(day12, part1)]
fn part1(input: &Input) -> usize {
    input
        .iter()
        .filter(|r| {
            let gifts = &r.gifts;
            let grid = Grid::<bool>::new(r.size.0, r.size.1);
            // Fill grid and return true if it's possible to fit all the gifts in the grid
            todo!("part 1 is not implemented yet")
        })
        .count()
}

// #[aoc(day12, part2)]
// fn part2(input: &Input) -> u64 {
//     todo!("part 2 is not implemented yet")
// }

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "0:
###
##.
##.

1:
###
##.
.##

2:
.##
###
##.

3:
##.
###
##.

4:
###
#..
###

5:
###
.#.
###

4x4: 0 0 0 0 2 0
12x5: 1 0 1 0 2 2
12x5: 1 0 1 0 3 2";
    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse(INPUT)), 2);
    }
    // #[test]
    // fn test_part2() {
    //     assert_eq!(part2(&parse(INPUT)), _);
    // }
}
