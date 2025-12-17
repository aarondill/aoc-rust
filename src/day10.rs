#![allow(unused)] // I never finished this...
use itertools::Itertools;
type Button = Vec<usize>;
#[derive(Debug, Clone, PartialEq, Eq, Default)]
struct IndicatorLights {
    values: Vec<bool>,
}
#[derive(Debug, Clone, PartialEq, Eq)]
struct Machine {
    indicators: IndicatorLights,
    buttons: Vec<Button>,
    joltage: Vec<u8>,
}
impl IndicatorLights {
    fn press(&mut self, button: &Button) {
        for &i in button {
            self.values[i] ^= true;
        }
    }
}
type Input = Vec<Machine>;

#[aoc_generator(day10)]
fn parse(input: &str) -> Input {
    input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| {
            let mut tokens_it = l.split_whitespace();
            let indicators = tokens_it
                .next()
                .inspect(|t| assert!(t.starts_with('[') & t.ends_with(']')))
                .map(|t| t[1..t.len() - 1].chars().map(|c| c == '#').collect())
                .map(|values| IndicatorLights { values })
                .expect("indicator");
            let buttons = tokens_it
                .take_while_ref(|t| t.starts_with('('))
                .inspect(|t| assert!(t.ends_with(')')))
                .map(|t| t[1..t.len() - 1].split(',').map(|s| s.parse().unwrap()).collect())
                .collect();
            let joltage = tokens_it
                .next()
                .inspect(|t| assert!(t.starts_with('{') & t.ends_with('}')))
                .map(|t| t[1..t.len() - 1].split(',').map(|s| s.parse().unwrap()).collect())
                .expect("joltage");
            assert_eq!(tokens_it.next(), None);
            Machine { indicators, buttons, joltage }
        })
        .collect()
}

#[aoc(day10, part1)]
fn part1(input: &Input) -> u64 {
    panic!("I gave up on this...")
}

// #[aoc(day10, part2)]
// fn part2(input: &Input) -> u64 {
//     todo!("part 2 is not implemented yet")
// }

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";
    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse(INPUT)), 7);
    }
    // #[test]
    // fn test_part2() {
    //     assert_eq!(part2(&parse(INPUT)), _);
    // }
}
