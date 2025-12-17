use itertools::Itertools;
type Input = Vec<[usize; 2]>;

#[aoc_generator(day2)]
fn parse(input: &str) -> Input {
    input
        .split(',')
        .filter(|s| !s.is_empty())
        .map(|range| range.split('-').next_array().unwrap().map(|s| s.parse().unwrap()))
        .collect()
}

#[aoc(day2, part1)]
fn part1(input: &Input) -> usize {
    fn is_invalid(num: usize) -> bool {
        let str = num.to_string();
        let (left, right) = str.split_at(str.len() / 2);
        left == right
    }
    input.iter().flat_map(|range| (range[0]..=range[1]).filter(|&i| is_invalid(i))).sum()
}

#[aoc(day2, part2)]
fn part2(input: &Input) -> usize {
    fn is_invalid(num: usize) -> bool {
        let str = num.to_string();
        // Not the most efficient way (this is basically a palindrome problem), but it works
        (1..=str.len() / 2).rev().any(|len| {
            let n = str.len() / len;
            let left = &str[..len];
            str == left.repeat(n)
        })
    }
    input.iter().flat_map(|range| (range[0]..=range[1]).filter(|&i| is_invalid(i))).sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse(INPUT)), 1227775554);
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2(&parse(INPUT)), 4174379265);
    }
}
