use itertools::Itertools;

type Input = (Vec<u64>, Vec<u64>);

#[aoc_generator(day1)]
fn parse(input: &str) -> Input {
    let mut one = Vec::new();
    let mut two = Vec::new();
    for line in input.lines() {
        let mut iter = line.split_whitespace();
        let a = iter.next().unwrap().parse().unwrap();
        let b = iter.next().unwrap().parse().unwrap();
        one.push(a);
        two.push(b);
    }
    (one, two)
}

#[aoc(day1, part1)]
fn part1(input: &Input) -> u64 {
    std::iter::zip(input.0.iter().sorted(), input.1.iter().sorted())
        .map(|(a, b)| a.abs_diff(*b))
        .sum()
}

#[aoc(day1, part2)]
fn part2(input: &Input) -> u64 {
    let counts = input.1.iter().counts();
    input.0.iter().map(|&b| b * counts.get(&b).cloned().unwrap_or(0) as u64).sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "3   4
4   3
2   5
1   3
3   9
3   3";
    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse(INPUT)), 11);
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2(&parse(INPUT)), 31);
    }
}
